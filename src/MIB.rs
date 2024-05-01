use std::collections::hash_map::Values;
use std::collections::HashMap;
use rustc_serialize::json::Json;
use std::fs::File;
use std::io::Read;
use crate::d4::DATATYPE;

pub(crate) struct MIB {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) oid: String,
    pub(crate) value: Vec<u8>,
    pub(crate) datatype: String
}

impl MIB{
    pub(crate) fn from_bytes(bytes: Vec<u8>) -> MIB {
        let mut mib = MIB {
            name: "".to_string(),
            description: "".to_string(),
            oid: "".to_string(),
            value: Vec::new(),
            datatype: "".to_string(),
        };
        if bytes.len() < 2 {
            return mib;
        }
        let mut i = 0;
        if bytes[i] == 0x30 { //all mibs seem to start with 48, so we are just cheating
            i += 1;
        }
        let mut total_length :u32 = bytes[i] as u32;
        //if the mib is big, this may get the right length. Or not. I don't know.
        if bytes.len() > 254 {
            i += 1;
            total_length = total_length << 8;
            total_length += bytes[i]  as u32;
            i += 1;
        }
        else {
            i += 1;
        }
        let mut oid_length = bytes[i] * 2;
        i += 1;
        i += 1;
        let mut x = bytes[i] / 40;
        let mut y = bytes[i] % 40;
        if x > 2 {
            y += (x - 2) * 40;
            x = 2
        }
        let mut oid_string = format!(".{}.{}", x, y);
        i += 1;

        oid_length -= 1;
        let mut val = 0 as u32;
        let mut in_index = false;
        while i < bytes.len() {
            oid_length -= 1;
            if oid_length == 0 {
                mib.oid = oid_string.clone();
                mib.value = bytes[i..].to_vec();
                // println!("OID: {}", oid_string);
                return mib;
            }
            // let mut print_var = false;
            // if val != 0 {
            //     print!("Old val{}.", val);
            //     print_var = true;
            // }
            val = (val << 7) | ((bytes[i] as u32 & 0x7F));
            // if print_var {
            //     println!(" New val: {}", val);
            // }
            if in_index == false {
                if bytes[i] & 0x80 != 0x80 {
                    oid_string.push_str(format!(".{}", val).as_str());
                    val = 0
                }
                // else {
                //     println!("Val is {}", val);
                // }
             }
            i += 1;
        }
        println!("OID with garbage: {}", oid_string);
        mib
    }
}

pub(crate) struct MIBList {
    pub(crate) mibs: HashMap<String, MIB>,
}

impl MIBList {
    pub(crate) fn new() -> MIBList {
        MIBList {
            mibs: HashMap::new(),
        }
    }

    pub(crate) fn add_mib(&mut self, mib: MIB) {
        self.mibs.insert(mib.oid.clone(), mib);
    }

    pub(crate) fn get_mib(&self, oid: &str) -> Option<&MIB> {
        if self.mibs.contains_key(oid) {
            return Some(self.mibs.get(oid).unwrap());
        }
        else { return None; }
    }

    pub(crate) fn get_mibs(&self) -> Values<String, MIB> {
        self.mibs.values()
    }
    pub(crate) fn from_file(filename: &str) -> MIBList {
        let mut retval = MIBList::new();
        let mut file = File::open(filename).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let json = Json::from_str(&data).unwrap();
        for (key, value) in json.as_object().unwrap() {
            let mut mib = MIB {
                name: "".to_string(),
                description: "".to_string(),
                oid: key.clone(),
                datatype: "".to_string(),
                value: Vec::new(),
            };
            for (k, v) in value.as_object().unwrap() {
                match k.as_str() {
                    "name" => mib.name = v.as_string().unwrap().to_string(),
                    "description" => mib.description = v.as_string().unwrap().to_string(),
                    "syntax" => { if v.as_string().is_none() {
                        // do nothing
                    } else {
                        mib.datatype = v.as_string().unwrap().to_string()
                    }
                    },
                    _ => {}
                }
            }
            retval.add_mib(mib);
        }
        retval
}
}
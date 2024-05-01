use std::collections::hash_map::Values;
use std::collections::HashMap;
use rustc_serialize::json::Json;
use std::fs::File;
use std::io::Read;
use crate::d4::DATATYPE;

#[derive(Clone)]
pub(crate) struct MIB {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) oid: String,
    pub(crate) value: Vec<u8>,
    pub(crate) datatype: String,
    pub(crate) index: u8,
}



impl MIB{
    pub(crate) fn from_bytes(bytes: Vec<u8>) -> MIB {
        let mut oidDataTypes :HashMap<u8, String> = HashMap::new();
        oidDataTypes.insert(0x01, "Boolean".to_string());
        oidDataTypes.insert(0x02, "Integer32".to_string());
        oidDataTypes.insert(0x03, "BitString".to_string());
        oidDataTypes.insert(0x04, "OctetString".to_string());
        oidDataTypes.insert(0x05, "Null".to_string());
        oidDataTypes.insert(0x06, "ObjectIdentifier".to_string());
        oidDataTypes.insert(64, "IPAddress".to_string());
        oidDataTypes.insert(66, "Counter32".to_string());
        oidDataTypes.insert(103, "OctetString".to_string());

        let mut mib = MIB {
            name: "".to_string(),
            description: "".to_string(),
            oid: "".to_string(),
            value: Vec::new(),
            datatype: "".to_string(),
            index: 0,
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
        i += 1;
        let mut oid_length = bytes[i];

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
                // date type, something, length, actual data
                mib.index = bytes[i];
                i += 1;
                let data_type = bytes[i];

                if oidDataTypes.contains_key(&data_type) {
                    mib.datatype = oidDataTypes.get(&data_type).unwrap().to_string();
                }
                else {
                    mib.datatype = format!("Unknown: {}", data_type);
                }
                i += 1;
                let length = bytes[i];
                i += 1;

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
    pub(crate) fn translate_value(&self) -> String {
        let mut retval = String::new();
        if self.datatype == "OctetString" {
                for i in 0..self.value.len() {
                    retval.push_str(format!("{}", self.value[i as usize] as char).as_str());
                }
        }
        else if self.datatype == "Integer32" {
            let mut val = 0;
            for b in &self.value {
                val = (val << 8) | *b as u32;
            }
            retval = format!("{}", val);
        }
            else if self.datatype == "IPAddress" {
            retval = format!("{}.{}.{}.{}", self.value[0], self.value[1], self.value[2], self.value[3]);
            }
        else {
            retval = format!("{:?}", self.value);
        }
        retval
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
                index: 0,
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
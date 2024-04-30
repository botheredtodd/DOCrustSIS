use std::collections::hash_map::Values;
use std::collections::HashMap;

pub(crate) struct MIB {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) oid: String,
    pub(crate) value: Vec<u8>,
    pub(crate) children: Vec<MIB>,
}

impl MIB{
    pub(crate) fn from_bytes(bytes: Vec<u8>) -> MIB {
        let mut mib = MIB {
            name: "".to_string(),
            description: "".to_string(),
            oid: "".to_string(),
            value: Vec::new(),
            children: Vec::new(),
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
use std::collections::hash_map::Values;
use std::collections::HashMap;
use rustc_serialize::json::Json;
use std::fs::File;
use std::io::Read;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};
use std::fmt;

#[derive(Clone)]
pub(crate) struct MIB {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) oid: String,
    pub(crate) datatype: String,
    pub(crate) index: u8,
    pub(crate) value: Vec<u8>,
}

impl Serialize for MIB {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut bytes = "".to_string();
        for i in 0..self.value.len() {
            bytes.push_str(format!("{:02x}", self.value[i as usize]).as_str());
            if i < self.value.len() - 1 {
                bytes.push_str(":");
            }
        }
        let mut state = serializer.serialize_struct("MIB", 3)?;
        state.serialize_field("Name", &self.name)?;
        state.serialize_field("Description", &self.description)?;
        state.serialize_field("OID", &self.oid)?;
        state.serialize_field("DataType", &self.datatype)?;
        state.serialize_field("Index", &self.index)?;
        state.serialize_field("Value", &bytes)?;
        match self.datatype.as_str() {
            "Integer32" => {
                let mut val = 0;
                for b in &self.value {
                    val = (val << 8) | *b as u32;
                }
                state.serialize_field("DecodedValue", &val)?;
            }
            "OctetString" => {
                let mut val = String::new();
                for b in &self.value {
                    val.push_str(format!("{}", *b as char).as_str());
                }
                state.serialize_field("DecodedValue", &val)?;
            }
            "SnmpAdminString" => {
                let mut val = String::new();
                for b in &self.value {
                    val.push_str(format!("{}", *b as char).as_str());
                }
                state.serialize_field("DecodedValue", &val)?;
            }
            "IPAddress" => {
                let val = format!("{}.{}.{}.{}", self.value[0], self.value[1], self.value[2], self.value[3]);
                state.serialize_field("DecodedValue", &val)?;
            }
            "Boolean" => {
                let val = self.value[0] == 0x01;
                state.serialize_field("DecodedValue", &val)?;
            }
            "BitString" => {
                let mut val = 0;
                for b in &self.value {
                    val = (val << 8) | *b as u32;
                }
                state.serialize_field("DecodedValue", &val)?;
            }
            "Counter32" => {
                let mut val = 0;
                for b in &self.value {
                    val = (val << 8) | *b as u32;
                }
                state.serialize_field("DecodedValue", &val)?;
            }
            _ => {
                state.serialize_field("DecodedValue", &bytes)?;
            }
        }

        state.end()
    }
}

impl MIB{
    pub(crate) fn from_bytes(bytes: Vec<u8>) -> MIB {
        let mut oid_data_types :HashMap<u8, String> = HashMap::new();
        oid_data_types.insert(0x01, "Boolean".to_string());
        oid_data_types.insert(0x02, "Integer32".to_string());
        oid_data_types.insert(0x03, "BitString".to_string());
        oid_data_types.insert(0x04, "SnmpAdminString".to_string());
        oid_data_types.insert(0x05, "Null".to_string());
        oid_data_types.insert(0x06, "ObjectIdentifier".to_string());
        oid_data_types.insert(64, "IPAddress".to_string());
        oid_data_types.insert(66, "Counter32".to_string());
        oid_data_types.insert(103, "OctetString".to_string());

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
        while i < bytes.len() {
            oid_length -= 1;
            if oid_length == 0 {
                mib.oid = oid_string.clone();
                // date type, something, length, actual data
                mib.index = bytes[i];
                i += 1;
                let data_type = bytes[i];

                if oid_data_types.contains_key(&data_type) {
                    mib.datatype = oid_data_types.get(&data_type).unwrap().to_string();
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
            if bytes[i] & 0x80 != 0x80 {
                oid_string.push_str(format!(".{}", val).as_str());
                val = 0
            }
                // else {
                //     println!("Val is {}", val);
                // }

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
        else if self.datatype == "SnmpAdminString" {
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


impl<'de> Deserialize<'de> for MIB {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Name, DataType, Description, OID, Value, Index, DecodedValue }
        const FIELDS: &[&str] = &["Name", "DataType", "Description", "OID", "Value", "Index", "DecodedValue"];
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("Must be Name, DataType, Description, OID, Value, Index, DecodedValue")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "Name" => Ok(Field::Name),
                            "Description" => Ok(Field::Description),
                            "OID" => Ok(Field::OID),
                            "DataType" => Ok(Field::DataType),
                            "Index" => Ok(Field::Index),
                            "Value" => Ok(Field::Value),
                            "DecodedValue" => Ok(Field::DecodedValue),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),

                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct MibVisitor;

        impl<'de> Visitor<'de> for MibVisitor {
            type Value = MIB;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct MIB")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<MIB, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let name = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let description = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let oid = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let datatype = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(4, &self))?;
                let index = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(5, &self))?;
                let value = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(6, &self))?;
                Ok(MIB { name, description, oid, datatype, index, value  })
            }


            fn visit_map<V>(self, mut map: V) -> Result<MIB, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut name = None;
                let mut description = None;
                let mut oid = None;
                let mut datatype = None;
                let mut index = None;
                let mut value = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        Field::Description => {
                            if description.is_some() {
                                return Err(de::Error::duplicate_field("description"));
                            }
                            description = Some(map.next_value()?);
                        }
                        Field::OID => {
                            if oid.is_some() {
                                return Err(de::Error::duplicate_field("oid"));
                            }
                            oid = Some(map.next_value()?);
                        }
                        Field::DataType => {
                            if datatype.is_some() {
                                return Err(de::Error::duplicate_field("datatype"));
                            }
                            datatype = Some(map.next_value()?);
                        }
                        Field::Index => {
                            if index.is_some() {
                                return Err(de::Error::duplicate_field("index"));
                            }
                            index = Some(map.next_value()?);
                        }
                        Field::Value => {
                            if value.is_some() {
                                return Err(de::Error::duplicate_field("value"));
                            }
                            value = Some(map.next_value()?);
                        }
                        Field::DecodedValue => {
                            let _ = map.next_value::<String>()?;
                        }
                    }
                }
                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                let description = description.ok_or_else(|| de::Error::missing_field("description"))?;
                let oid = oid.ok_or_else(|| de::Error::missing_field("oid"))?;
                let datatype = datatype.ok_or_else(|| de::Error::missing_field("datatype"))?;
                let index = index.ok_or_else(|| de::Error::missing_field("index"))?;
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(MIB { name, description, oid, datatype, index, value })
            }
        }

        deserializer.deserialize_struct("MIB", FIELDS, MibVisitor)
    }
}
//Some TLV information for DOCSIS 4
// use error_chain::error_chain;

use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::{Display};
use std::io::Error;
use std::path::Path;
use directories::UserDirs;
use crate::tlv::TLV;
use crate::mib::MIB;
use crate::flows::upstreams;
use crate::drop_classifiers::upstream_drop_classifiers;
use crate::mib;
use crate::vendor_specific::docsis_vendor_specific;
use crate::erouter::erouter;
use crate::packet_classifiers::packet_classifiers;
use serde::ser::{Serialize, Serializer, SerializeStruct};
// error_chain! {
//     foreign_links {
//         Io(std::io::Error);
//     }
// }


#[derive(Clone)]
pub(crate) enum DATATYPE {
    UCHAR,
    UINT,
    USHORT,
    STRING,
    STRINGZERO,
    AGGREGATE,
    MIB,
    HEXSTR,
    MD5,
    LENZERO,
    DUAL_QTAG,
    ENCODE_IP,
    ENCODE_MAC,

}

impl Serialize for crate::d4::DATATYPE {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("DATATYPE", 3)?;
        match self {
            DATATYPE::UCHAR => { state.serialize_field("DATATYPE", "UCHAR")? }
            DATATYPE::UINT => { state.serialize_field("DATATYPE", "UINT")? }
            DATATYPE::USHORT => { state.serialize_field("DATATYPE", "USHORT")? }
            DATATYPE::STRING => { state.serialize_field("DATATYPE", "STRING")? }
            DATATYPE::STRINGZERO => { state.serialize_field("DATATYPE", "STRINGZERO")? }
            DATATYPE::AGGREGATE => { state.serialize_field("DATATYPE", "AGGREGATE")? }
            DATATYPE::MIB => { state.serialize_field("DATATYPE", "MIB")? }
            DATATYPE::HEXSTR => { state.serialize_field("DATATYPE", "HEXSTR")? }
            DATATYPE::MD5 => { state.serialize_field("DATATYPE", "MD5")? }
            DATATYPE::LENZERO => { state.serialize_field("DATATYPE", "LENZERO")? }
            DATATYPE::DUAL_QTAG => { state.serialize_field("DATATYPE", "DUAL_QTAG")? }
            DATATYPE::ENCODE_IP => { state.serialize_field("DATATYPE", "ENCODE_IP")? }
            DATATYPE::ENCODE_MAC => { state.serialize_field("DATATYPE", "ENCODE_MAC")? }
        }
        state.end()
    }
}


#[derive(Clone)]

pub(crate) struct DOCSIS4TLV {
    pub(crate) t: u8,
    pub(crate) description: String,
    pub(crate) data_type: DATATYPE,
    pub(crate) tlv: TLV,
    pub(crate) sub_tlvs: HashMap<u8, DOCSIS4TLV>,
    pub(crate) mib: Option<MIB>,
}


impl Serialize for DOCSIS4TLV {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("DOCSIS4TLV", 3)?;
        state.serialize_field("Description", &self.description)?;
        state.serialize_field("Type", &self.t)?;
        state.serialize_field("Length", &self.tlv.l)?;
        state.serialize_field("Value", &self.tlv.v)?;
        state.serialize_field("Data Type", &self.data_type)?;
        if vec![DATATYPE::AGGREGATE].contains(&self.data_type)  {
            state.serialize_field("Sub TLVs", &self.sub_tlvs)?;
        }
        else if vec![DATATYPE::MIB].contains(&self.data_type) {
            state.serialize_field("MIB", &self.mib)?;
        }
            else if vec![DATATYPE::DUAL_QTAG] .contains(&self.data_type){
            state.serialize_field("Decoded Value", "No idea at this point")?;
            }
        else {
            state.serialize_field("Decoded Value", &self.get_string_value().unwrap())?;
        }
        state.end()
    }
}

impl Default for DOCSIS4TLV {
    fn default() -> DOCSIS4TLV {
        DOCSIS4TLV {
            t: 0,
            description: "".to_string(),
            data_type: DATATYPE::UCHAR,
            tlv: TLV { t: 0, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
            mib: None,
        }
    }
}


impl PartialEq for DATATYPE {
    fn eq(&self, other: &Self) -> bool {
        match self {
            DATATYPE::UCHAR => {
                match other {
                    DATATYPE::UCHAR => true,
                    _ => false,
                }
            },
            DATATYPE::UINT => {
                match other {
                    DATATYPE::UINT => true,
                    _ => false,
                }
            },
            DATATYPE::USHORT => {
                match other {
                    DATATYPE::USHORT => true,
                    _ => false,
                }
            },
            DATATYPE::STRING => {
                match other {
                    DATATYPE::STRING => true,
                    _ => false,
                }
            },
            DATATYPE::STRINGZERO => {
                match other {
                    DATATYPE::STRINGZERO => true,
                    _ => false,
                }
            },
            DATATYPE::AGGREGATE => {
                match other {
                    DATATYPE::AGGREGATE => true,
                    _ => false,
                }
            },
            DATATYPE::MIB => {
                match other {
                    DATATYPE::MIB => true,
                    _ => false,
                }
            },
            DATATYPE::HEXSTR => {
                match other {
                    DATATYPE::HEXSTR => true,
                    _ => false,
                }
            },
            DATATYPE::MD5 => {
                match other {
                    DATATYPE::MD5 => true,
                    _ => false,
                }
            },
            DATATYPE::LENZERO => {
                match other {
                    DATATYPE::LENZERO => true,
                    _ => false,
                }
            },
            DATATYPE::DUAL_QTAG => {
                match other {
                    DATATYPE::DUAL_QTAG => true,
                    _ => false,
                }
            },
            DATATYPE::ENCODE_IP => {
                match other {
                    DATATYPE::ENCODE_IP => true,
                    _ => false,
                }
            },
            DATATYPE::ENCODE_MAC => {
                match other {
                    DATATYPE::ENCODE_MAC => true,
                    _ => false,
                }
            },
        }
    }
}

impl DOCSIS4TLV {
    pub(crate) fn display_tag(&self, parent: &str) -> String {
        format!("{}.{}", parent, self.t,)
    }
    pub(crate) fn get_int_value(&self) -> Result<u32, String>{
        match self.data_type {
            DATATYPE::UCHAR => {
                let mut v = 0;
                for i in 0..self.tlv.l {
                    v = v << 8;
                    v = v | self.tlv.v[i as usize] as u32;
                }
                Ok(v)
            },
            DATATYPE::UINT => {
                let mut v = 0;
                for i in 0..self.tlv.l {
                    v = v << 8;
                    v = v | self.tlv.v[i as usize] as u32;
                }
                Ok(v)
            },
            DATATYPE::USHORT => {
                let mut v = 0;
                for i in 0..self.tlv.l {
                    v = v << 8;
                    v = v | self.tlv.v[i as usize] as u32;
                }
                Ok(v)
            },
            DATATYPE::STRING => {
                Err("Not a number".to_string().into())
            },
            DATATYPE::STRINGZERO => {
                Err("Not a number".to_string().into())
            },
            DATATYPE::HEXSTR => {
                Err("Not a number".to_string().into())
            },
            DATATYPE::ENCODE_IP => {
                Err("Not a number".to_string().into())
            },
            DATATYPE::ENCODE_MAC => {
                Err("Not a number".to_string().into())
            },
            DATATYPE::AGGREGATE => {
                Err("Sub TLVs not working yet".to_string().into())
            },
            DATATYPE::MIB => {
                // let _ = MIB::from_bytes(self.tlv.v);
                Err("Not yet supported".to_string().into())
            }
            DATATYPE::MD5 => {
                Err("Not a number".to_string().into())
            },
            _ => {
                Err("Not yet supported".to_string().into())
            }
        }
    }

    pub(crate) fn set_uint_value(&mut self, v: u32) -> Result<u32, String> {
        match self.data_type{

            DATATYPE::MD5 => {
                Err("Not a number".to_string().into())
            },
            DATATYPE::ENCODE_IP => {
                Err("Not a number".to_string().into())
            },
            DATATYPE::ENCODE_MAC => {
                Err("Not a number".to_string().into())
            },
            DATATYPE::STRING => {
                Err("Not a number".to_string().into())
            },
            DATATYPE::HEXSTR => {
                Err("Not a number".to_string().into())
            },
            DATATYPE::STRINGZERO => {
                Err("Not a number".to_string().into())
            },
            DATATYPE::AGGREGATE => {
                Err("Sub TLVs not working yet".to_string().into())
            },
            DATATYPE::UCHAR => {
                let mut nv = v;
                let mut l = 0;
                while nv > 0 {
                    l += 1;
                    nv = nv >> 8;
                }
                let mut v = v;
                println!("Setting value from {} to {}", self.get_int_value().unwrap(), v);
                self.tlv.v = Vec::new();
                for i in 0..l {
                    self.tlv.v.push((v & 0xFF) as u8);
                    v = v >> 8;
                }
                self.tlv.l = l;
                Ok(v)
            },
            DATATYPE::UINT => {
                let mut nv = v;
                let mut l = 0;
                while nv > 0 {
                    l += 1;
                    nv = nv >> 8;
                }
                let mut v = v;
                println!("Setting value from {} to {}", self.get_int_value().unwrap(), v);
                self.tlv.v = Vec::new();
                for i in 0..l {
                    self.tlv.v.push((v & 0xFF) as u8);
                    v = v >> 8;
                }
                self.tlv.l = l;
                Ok(v)
            },
            DATATYPE::USHORT => {
                let mut nv = v;
                let mut l = 0;
                while nv > 0 {
                    l += 1;
                    nv = nv >> 8;
                }
                let mut v = v;
                println!("Setting value from {} to {}", self.get_int_value().unwrap(), v);
                self.tlv.v = Vec::new();
                for i in 0..l {
                    self.tlv.v.push((v & 0xFF) as u8);
                    v = v >> 8;
                }
                self.tlv.l = l;
                Ok(v)
            },
            _ => {
                Err("Not yet supported".to_string().into())
            }
        }

    }
    pub(crate) fn get_string_value(&self) -> Result<String, String> {
        match self.data_type {
            DATATYPE::UCHAR => {
                let mut s = String::new();
                for i in 0..self.tlv.l {
                    s.push_str(format!("{}", self.tlv.v[i as usize]).as_str());
                }
                Ok(s)
            },
            DATATYPE::UINT => {
                let mut s = String::new();
                for i in 0..self.tlv.l {
                    s.push_str(format!("{}", self.tlv.v[i as usize]).as_str());
                }
                Ok(s)
            },
            DATATYPE::USHORT => {
                let mut s = String::new();
                for i in 0..self.tlv.l {
                    s.push_str(format!("{}", self.tlv.v[i as usize]).as_str());
                }
                Ok(s)
            },
            DATATYPE::MD5 => {
                let mut s = String::new();
                for i in 0..self.tlv.l {
                    s.push_str(format!("{:02x}", self.tlv.v[i as usize]).as_str());
                }
                Ok(s)
            },
            DATATYPE::ENCODE_IP => {
                let mut s = String::new();
                if self.tlv.l > 4 {
                    println!("IPv6 is still being worked on");
                }
                for i in 0..self.tlv.l {
                    s.push_str(format!("{}", self.tlv.v[i as usize]).as_str());
                    if i < self.tlv.l - 1 {
                        s.push_str(".");
                    }
                }
                Ok(s)
            },
            DATATYPE::ENCODE_MAC => {
                let mut s = String::new();
                for i in 0..self.tlv.l {
                    s.push_str(format!("{}", self.tlv.v[i as usize]).as_str());
                    if i % 2 == 0 && i < self.tlv.l - 1{
                        s.push_str(":");
                    }
                }
                Ok(s)
            },
            DATATYPE::STRING => {
                let mut s = String::new();
                for i in 0..self.tlv.l {
                    s.push_str(format!("{}", self.tlv.v[i as usize] as char).as_str());
                }
                Ok(s)
            },
            DATATYPE::HEXSTR => {
                let mut s = String::new();
                for i in 0..self.tlv.l {
                    s.push_str(format!("{}", self.tlv.v[i as usize] as char).as_str());
                }
                Ok(s)
            },
            DATATYPE::STRINGZERO => {
                let mut s = String::new();
                for i in 0..self.tlv.l {
                    s.push_str(format!("{}", self.tlv.v[i as usize] as char).as_str());
                }
                Ok(s)
            },
            DATATYPE::AGGREGATE => {
                Err("Sub TLVs not working yet".to_string().into())
            },

            // _ => {
            //     Err("Not yet supported".to_string().into())
            // }
            DATATYPE::MIB => {Err("MIBs not yet supported".to_string().into())}
            DATATYPE::LENZERO => {Ok("Zero Length TLV".to_string())}
            DATATYPE::DUAL_QTAG => {Err("DUAL_QTAG not yet supported".to_string().into())}
        }
    }
    fn display_agg(&self) -> String {
        let mut s = String::new();
        s.push_str(format!("TLV: {}: {}", self.t, self.description).as_str());
        for (_, v) in self.sub_tlvs.iter() {
            if v.tlv.v != Vec::<u8>::new() {
                if v.data_type != DATATYPE::AGGREGATE {
                    s.push_str(format!("\n\tSub {}", v).as_str());
                } else {
                    s.push_str(format!("\n\tSub {} has its own sub tlvs", v.t).as_str());
                    for (_, sv) in v.sub_tlvs.iter() {
                        if sv.tlv.v != Vec::<u8>::new() {
                            if sv.data_type == DATATYPE::AGGREGATE {
                                s.push_str(format!("\n\t\tSub Sub {} has even more sub sub subs, and that's just crazy", sv.t).as_str());
                            } else {
                                s.push_str(format!("\n\t\tSub Sub {}", sv).as_str());
                            }
                        }
                    }
                }
            }
        }
        s
    }

}
impl Display for DOCSIS4TLV {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let mut s = String::new();
            match self.data_type {
                DATATYPE::UCHAR => {
                    s.push_str(format!("TLV: {}: {} {:?} ", self.t, self.description, self.tlv.v).as_str());
                    s.push_str(format!("Decoded: {}", self.get_int_value().unwrap()).as_str());
                },
                DATATYPE::UINT => {
                    s.push_str(format!("TLV: {}: {} {:?} ", self.t, self.description, self.tlv.v).as_str());
                    s.push_str(format!("Decoded: {}", self.get_int_value().unwrap()).as_str());
                },
                DATATYPE::USHORT => {
                    s.push_str(format!("TLV: {}: {} {:?} ", self.t, self.description, self.tlv.v).as_str());
                    s.push_str(format!("Decoded: {}", self.get_int_value().unwrap()).as_str());
                },
                DATATYPE::STRING => {
                    s.push_str(format!("TLV: {}: {} {:?} ", self.t, self.description, self.tlv.v).as_str());
                    s.push_str(format!("Decoded: {}", self.get_string_value().unwrap()).as_str());
                }
                DATATYPE::HEXSTR => {
                    s.push_str(format!("TLV: {}: {} {:?} ", self.t, self.description, self.tlv.v).as_str());
                    s.push_str(format!("Decoded: {}", self.get_string_value().unwrap()).as_str());
                }
                DATATYPE::MD5 => {
                    s.push_str(format!("TLV: {}: {} {:?} ", self.t, self.description, self.tlv.v).as_str());
                    s.push_str(format!("Decoded: {}", self.get_string_value().unwrap()).as_str());
                }
                DATATYPE::ENCODE_IP => {
                    s.push_str(format!("TLV: {}: {} {:?} ", self.t, self.description, self.tlv.v).as_str());
                    s.push_str(format!("Decoded: {}", self.get_string_value().unwrap()).as_str());
                }
                DATATYPE::ENCODE_MAC => {
                    s.push_str(format!("TLV: {}: {} {:?} ", self.t, self.description, self.tlv.v).as_str());
                    s.push_str(format!("Decoded: {}", self.get_string_value().unwrap()).as_str());
                }
                DATATYPE::MIB => {
                    let filename = format!("{}/.mibs.json", UserDirs::new().unwrap().home_dir().to_str().unwrap());
                    let mut miblist = mib::MIBList::new();
                    if Path::new(filename.as_str()).exists() {
                        miblist = mib::MIBList::from_file(filename.as_str());
                    }
                    s.push_str(format!("TLV: {}: {}  ", self.t, self.description).as_str());
                    // println!("Decoded: {}", this_d4_unwrapped.get_mib_value().unwrap());
                    if self.mib.is_none() {
                        s.push_str(format!("MIB TLV, but mib data is missing  ").as_str());
                    }
                    else {
                        let mb = self.mib.as_ref().unwrap();
                        let mbname = miblist.get_mib(mb.oid.as_str());
                        if mbname.is_some() {
                            s.push_str(format!("MIB: {} ({}): <{}> {}: {:?} Raw Bytes: {:?}", mb.oid, mbname.unwrap().name, mb.datatype, mb.index, mb.translate_value(), mb.value).as_str());
                        } else {
                            s.push_str(format!("MIB: {} ({}): <{}> {}: {:?} Raw Bytes: {:?}", mb.oid, "Unknown MIB", mb.datatype, mb.index, mb.translate_value(), mb.value).as_str());
                        }
                    }
                }
                DATATYPE::AGGREGATE => {
                    // s.push_str(format!("TLV: {}: {}", self.t, self.description).as_str());
                    s.push_str(format!("\n{}", self.display_agg()).as_str());
                }
                _ => {
                    s.push_str(format!("TLV: {}: {}  ", self.t, self.description).as_str());
                    s.push_str(format!("Not yet supported").as_str());
                }

        }
        write!(f, "{}", s)
    }
}
pub(crate) fn d4_defs() -> HashMap<u8, DOCSIS4TLV> {
    let mut d4_defs = HashMap::new();
    let mut tlv = DOCSIS4TLV {
        t: 0x01,
        description: "DownstreamFrequency".to_string(),
        data_type: DATATYPE::UINT,
        tlv: TLV { t: 0x01, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(tlv.t, tlv);
    let mut tlv = DOCSIS4TLV {
        t: 0x02,
        description: "UpstreamChannelId".to_string(),
        data_type: DATATYPE::UCHAR,
        tlv: TLV { t: 0x02, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(tlv.t, tlv);
    let mut tlv = DOCSIS4TLV {
        t: 0x03,
        description: "NetworkAccess".to_string(),
        data_type: DATATYPE::UCHAR,
        tlv: TLV { t: 0x03, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(tlv.t, tlv);
    let mut tlv = DOCSIS4TLV {
        t: 0x06,
        description: "CmMic".to_string(),
        data_type: DATATYPE::MD5,
        tlv: TLV { t: 0x06, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(tlv.t, tlv);
    let mut tlv = DOCSIS4TLV {
        t: 0x07,
        description: "CmMic".to_string(),
        data_type: DATATYPE::MD5,
        tlv: TLV { t: 0x07, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(tlv.t, tlv);

    let mut tlv = DOCSIS4TLV {
        t: 0x0b,
        description: "SnmpMibObject".to_string(),
        data_type: DATATYPE::MIB,
        tlv: TLV { t: 0x0b, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(tlv.t, tlv);

    let mut tlv = DOCSIS4TLV {
        t: 0x11,
        description: "BaselinePrivacy".to_string(),
        data_type: DATATYPE::AGGREGATE,
        tlv: TLV { t: 0x11, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    let sub_tlv = DOCSIS4TLV {
        t: 0x01,
        description: "AuthTimeout".to_string(),
        data_type: DATATYPE::UINT,
        tlv: TLV { t: 0x01, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    tlv.sub_tlvs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
        t: 0x02,
        description: "ReAuthTimeout".to_string(),
        data_type: DATATYPE::UINT,
        tlv: TLV { t: 0x02, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    tlv.sub_tlvs.insert(sub_tlv.t, sub_tlv);

    let sub_tlv = DOCSIS4TLV {
        t: 0x03,
        description: "AuthGraceTime".to_string(),
        data_type: DATATYPE::UINT,
        tlv: TLV { t: 0x03, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    tlv.sub_tlvs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
        t: 0x04,
        description: "OperTimeout".to_string(),
        data_type: DATATYPE::UINT,
        tlv: TLV { t: 0x04, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    tlv.sub_tlvs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
        t: 0x05,
        description: "ReKeyTimeout".to_string(),
        data_type: DATATYPE::UINT,
        tlv: TLV { t: 0x05, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    tlv.sub_tlvs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
        t: 0x06,
        description: "TEKGraceTime".to_string(),
        data_type: DATATYPE::UINT,
        tlv: TLV { t: 0x06, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    tlv.sub_tlvs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
        t: 0x07,
        description: "AuthRejectTimeout".to_string(),
        data_type: DATATYPE::UINT,
        tlv: TLV { t: 0x07, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    tlv.sub_tlvs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
        t: 0x08,
        description: "SAMapWaitTimeout".to_string(),
        data_type: DATATYPE::UINT,
        tlv: TLV { t: 0x08, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    tlv.sub_tlvs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
        t: 0x09,
        description: "SAMapMaxRetries".to_string(),
        data_type: DATATYPE::UINT,
        tlv: TLV { t: 0x09, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    tlv.sub_tlvs.insert(sub_tlv.t, sub_tlv);

    d4_defs.insert(tlv.t, tlv);


    let mut tlv = DOCSIS4TLV {
        t: 0x12,
        description: "MaxCPE".to_string(),
        data_type: DATATYPE::UCHAR,
        tlv: TLV { t: 0x12, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(tlv.t, tlv);

    let mut tlv = DOCSIS4TLV {
        t: 0x16,
        description: "UsPacketClass".to_string(),
        data_type: DATATYPE::AGGREGATE,
        tlv: TLV { t: 0x16, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: packet_classifiers(),
        mib: None,
        };
    d4_defs.insert(tlv.t, tlv);
    let mut tlv = DOCSIS4TLV {
        t: 0x17,
        description: "DsPacketClass".to_string(),
        data_type: DATATYPE::AGGREGATE,
        tlv: TLV { t: 0x17, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: packet_classifiers(),
        mib: None,
        };
    d4_defs.insert(tlv.t, tlv);

//Upstream Service Flow


    let mut tlv = DOCSIS4TLV {
        t: 0x18,
        description: "UsServiceFlow".to_string(),
        data_type: DATATYPE::AGGREGATE,
        tlv: TLV { t: 0x18, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: upstreams(),
        mib: None,
        };
    d4_defs.insert(tlv.t, tlv);

//Downstream Service Flow
    let mut tlv = DOCSIS4TLV {
        t: 0x19,
        description: "DsServiceFlow".to_string(),
        data_type: DATATYPE::AGGREGATE,
        tlv: TLV { t: 0x19, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: upstreams(), //for now this will work, but there may be some differences I haven't gotten to yet
        mib: None,
        };
    d4_defs.insert(tlv.t, tlv);

    let mut tlv = DOCSIS4TLV {
        t: 0x1d,
        description: "GlobalPrivacyEnable".to_string(),
        data_type: DATATYPE::UCHAR,
        tlv: TLV { t: 0x1d, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(tlv.t, tlv);

    let mut tlv = DOCSIS4TLV {
        t: 0x20,
        description: "MfgCVCData".to_string(),
        data_type: DATATYPE::HEXSTR,
        tlv: TLV { t: 0x20, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(tlv.t, tlv);

    let mut tlv = DOCSIS4TLV {
        t: 0x21,
        description: "CoSignerCVCData".to_string(),
        data_type: DATATYPE::HEXSTR,
        tlv: TLV { t: 0x21, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(tlv.t, tlv);

    let mut tlv = DOCSIS4TLV {
        t: 0x2b,
        description: "DsServiceFlow".to_string(),
        data_type: DATATYPE::AGGREGATE,
        tlv: TLV { t: 0x2b, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: docsis_vendor_specific(),
        mib: None,
        };
    d4_defs.insert(tlv.t, tlv);

    let mut tlv = DOCSIS4TLV {
        t: 0x3c,
        description: "UpstreamDropPacketClassification".to_string(),
        data_type: DATATYPE::AGGREGATE,
        tlv: TLV { t: 0x32, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: upstream_drop_classifiers(),
        mib: None,
        };
    d4_defs.insert(tlv.t, tlv);

    let mut tlv = DOCSIS4TLV {
        t: 0x54,
        description: "DiplexerBandEdge".to_string(),
        data_type: DATATYPE::AGGREGATE,
        tlv: TLV { t: 0x54, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
        };
    let mut sub_tlv = DOCSIS4TLV {
        t: 0x01,
        description: "DiplexerUpstreamUpperBandEdge".to_string(),
        data_type: DATATYPE::UCHAR,
        tlv: TLV { t: 0x01, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    tlv.sub_tlvs.insert(sub_tlv.t, sub_tlv);
    let mut sub_tlv = DOCSIS4TLV {
        t: 0x02,
        description: "DiplexerDownstreamLowerBandEdge".to_string(),
        data_type: DATATYPE::UCHAR,
        tlv: TLV { t: 0x02, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    tlv.sub_tlvs.insert(sub_tlv.t, sub_tlv);
    let mut sub_tlv = DOCSIS4TLV {
        t: 0x03,
        description: "DiplexerDownstreamUpperBandEdge".to_string(),
        data_type: DATATYPE::UCHAR,
        tlv: TLV { t: 0x03, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    tlv.sub_tlvs.insert(sub_tlv.t, sub_tlv);
    d4_defs.insert(tlv.t, tlv);

    let mut tlv = DOCSIS4TLV {
        t: 0xca,
        description: "eRouter".to_string(),
        data_type: DATATYPE::AGGREGATE,
        tlv: TLV { t: 0xca, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: erouter(),
        mib: None,
        };
    d4_defs.insert(tlv.t, tlv);

    let mut tlv = DOCSIS4TLV {
        t: 0xff,
        description: "/*EndOfDataMkr*/".to_string(),
        data_type: DATATYPE::UINT,
        tlv: TLV { t: 0xff, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(tlv.t, tlv);
    d4_defs
}



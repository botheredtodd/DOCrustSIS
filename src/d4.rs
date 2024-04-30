//Some TLV information for DOCSIS 4

use std::cmp::PartialEq;
use std::collections::HashMap;
use crate::TLV::TLV;
#[derive(Clone)]
pub(crate) enum DATATYPE {
    UCHAR,
    UINT,
    STRING,

}
#[derive(Clone)]
pub(crate) struct DOCSIS4TLV {
    pub(crate) t: u8,
    pub(crate) description: String,
    pub(crate) dataType: DATATYPE,
    pub(crate) tlv: TLV,
    pub(crate) sub_tlvs: Vec<DOCSIS4TLV>,
}



impl DOCSIS4TLV {
    pub(crate) fn get_int_value(self) -> Result<u32, String>{
        match self.dataType {
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
            DATATYPE::STRING => {
                Err("Not a number".to_string())
            }
        }
    }
    pub(crate) fn set_int_value(&mut self, v: i32) -> Result<i32, String> {
        match self.dataType{
            DATATYPE::UCHAR => {
                let mut v = v;
                let mut l = 0;
                while v > 0 {
                    l += 1;
                    v = v >> 8;
                }
                let mut v = v;
                for i in 0..l {
                    self.tlv.v.push((v & 0xFF) as u8);
                    v = v >> 8;
                }
                self.tlv.l = l;
                Ok(v)
            },
            DATATYPE::UINT => {
                let mut v = v;
                let mut l = 0;
                while v > 0 {
                    l += 1;
                    v = v >> 8;
                }
                let mut v = v;
                for i in 0..l {
                    self.tlv.v.push((v & 0xFF) as u8);
                    v = v >> 8;
                }
                self.tlv.l = l;
                Ok(v)
            },
            DATATYPE::STRING => {
                Err("Not a number".to_string())
            }
        }

    }
    pub(crate) fn get_string_value(self) -> Result<String, String> {
        match self.dataType {
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
            DATATYPE::STRING => {
                let mut s = String::new();
                for i in 0..self.tlv.l {
                    s.push_str(format!("{}", self.tlv.v[i as usize] as char).as_str());
                }
                Ok(s)
            }
        }
    }


}
pub(crate) fn d4_defs() -> HashMap<u8, DOCSIS4TLV> {
    let mut d4_defs = HashMap::new();
    let mut tlv = DOCSIS4TLV {
        t: 0x01,
        description: "DownstreamFrequency".to_string(),
        dataType: DATATYPE::UINT,
        tlv: TLV { t: 0x01, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: Vec::new(),
    };
    d4_defs.insert(tlv.t, tlv);
    let mut tlv = DOCSIS4TLV {
        t: 0x02,
        description: "UpstreamChannelId".to_string(),
        dataType: DATATYPE::UCHAR,
        tlv: TLV { t: 0x02, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: Vec::new(),
    };
    d4_defs.insert(tlv.t, tlv);
    let mut tlv = DOCSIS4TLV {
        t: 0x03,
        description: "NetworkAccess".to_string(),
        dataType: DATATYPE::UCHAR,
        tlv: TLV { t: 0x03, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: Vec::new(),
    };
    d4_defs.insert(tlv.t, tlv);
    d4_defs
}

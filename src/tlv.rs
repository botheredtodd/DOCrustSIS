use std::fmt;
use serde::Serialize;

#[derive(Debug, Clone)]
#[derive(Serialize)]
pub(crate) struct TLV {
    pub(crate) t: u8,
    pub(crate) l: u8,
    pub(crate) v: Vec<u8>,
    pub(crate) sub_tlvs: Vec<TLV>,
}

impl fmt::Display for TLV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.sub_tlvs.len() > 0 {
            let mut s = String::new();
            s.push_str(format!("Type: {}, Length: {},  Sub-TLVs:", self.t, self.l).as_str());
            for st in &*self.sub_tlvs {
                s.push_str(format!("\tType: {}, Length: {}, Value: {:?}", st.t, st.l, st.v).as_str())
            }
            write!(f, "{}", s)
        } else {
            write!(f, "Type: {}, Length: {}, Value: {:?}", self.t, self.l, self.v)
        }
    }
}
pub(crate) struct TLVList {
    pub(crate) tlvs: Vec<TLV>,
}
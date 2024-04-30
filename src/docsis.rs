use crate::TLV::TLV;
use std::fmt;
fn decode(TLVs: Vec<TLV>) -> String {
    let mut s = String::new();
    for tlv in TLVs {
        s.push_str(&format!("{:?}", tlv));
    }
    s
}
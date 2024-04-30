use crate::TLV::TLV;
use std::fmt;
use crate::d4::d4_defs;
use crate::d4::DOCSIS4TLV;
pub(crate) fn decode(tlv: TLV) -> Result<DOCSIS4TLV, String> {

    let d4 = d4_defs();
    if d4.contains_key(&tlv.t) {
        println!("Found TLV: {}: {}", tlv.t, d4.get(&tlv.t).unwrap().description);
        let mut ret = d4.get(&tlv.t).unwrap().clone();
        ret.tlv = tlv.clone();
        return Ok(ret);
    }
    else {
        return Err(format!("TLV: {} not found", tlv.t));
    }


}
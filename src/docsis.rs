use std::collections::HashMap;
use crate::tlv::TLV;
use std::fmt;
use crate::d4::d4_defs;
use crate::d4::DOCSIS4TLV;
use crate::d4::DATATYPE;
use crate::mib::MIB;
pub(crate) fn decode(tlv: TLV, d4: HashMap<u8, DOCSIS4TLV>) -> Result<DOCSIS4TLV, String> {


    if d4.contains_key(&tlv.t) {
        // println!("Found TLV: {}: {}", tlv.t, d4.get(&tlv.t).unwrap().description);
        let mut ret = d4.get(&tlv.t).unwrap().clone();
        ret.tlv = tlv.clone();
        match ret.data_type {
            DATATYPE::AGGREGATE => {
                let mut i = 0;
                let d4sub = d4.get(&ret.t).unwrap().clone();
                while i < tlv.v.len() {
                    let stlv = TLV { t: tlv.v[i], l: tlv.v[i + 1], v: tlv.v[i + 2..i + 2 + tlv.v[i + 1] as usize].to_vec(), sub_tlvs: Vec::new() };
                    let this_d4 = decode(stlv, d4sub.sub_tlvs.clone());
                    if this_d4.is_ok() {
                        let this_d4_unwrapped = this_d4.unwrap();
                        ret.sub_tlvs.insert(this_d4_unwrapped.t, this_d4_unwrapped);
                    }
                    i += 2 + tlv.v[i + 1] as usize;
                }

            },
            DATATYPE::MIB => {
                let mb = MIB::from_bytes(tlv.v.clone());
                ret.mib = Some(mb);
            },
            _ => {}
        }

        return Ok(ret);
    }
    else {
        return Err(format!("TLV: {} not found", tlv.t));
    }


}
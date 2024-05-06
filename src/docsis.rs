use std::collections::HashMap;
use crate::tlv::TLV;
use crate::d4::DOCSIS4TLV;
use crate::d4::DATATYPE;
use crate::mib::MIB;
pub(crate) fn decode(tlv: TLV, d4: HashMap<u8, DOCSIS4TLV>) -> Result<DOCSIS4TLV, String> {
    return if d4.contains_key(&tlv.t) {
        // println!("Found TLV: {}: {}", tlv.t, d4.get(&tlv.t).unwrap().description);
        let mut ret = d4.get(&tlv.t).unwrap().clone();
        ret.tlv = tlv.clone();
        ret.sub_tlvs = HashMap::new();
        match ret.data_type {
            DATATYPE::AGGREGATE => {
                let mut i = 0;
                let d4sub = d4.get(&ret.t).unwrap().clone();
                while i < tlv.v.len() {
                    let sub_tlv = TLV { t: tlv.v[i], l: tlv.v[i + 1], v: tlv.v[i + 2..i + 2 + tlv.v[i + 1] as usize].to_vec(), sub_tlvs: Vec::new() };
                    let this_d4 = decode(sub_tlv.clone(), d4sub.sub_tlvs.clone());
                    if this_d4.is_ok() {
                        let this_d4_unwrapped = this_d4.unwrap();
                        ret.sub_tlvs.insert(this_d4_unwrapped.t, this_d4_unwrapped);
                    } else {
                        println!("Could not decode: {}.{}", ret.t, sub_tlv.t);
                    }
                    i += 2 + tlv.v[i + 1] as usize;
                }
                // Clear out the empty sub_tlvs
                // let mut sub_keys:Vec<u8> = Vec::new();
                // for k in ret.sub_tlvs.keys() {
                //     let j = k.clone();
                //     sub_keys.push(j);
                // }
                // for k in sub_keys {
                //     let ll = ret.sub_tlvs.get(&k).unwrap().tlv.l.clone();
                //     if ll == 0 {
                //         ret.sub_tlvs.remove(&k);
                //     }
                // }
            },
            DATATYPE::MIB => {
                let mb = MIB::from_bytes(tlv.v.clone());
                ret.mib = Some(mb);
            },
            _ => {}
        }
        Ok(ret)
    } else {
        Err(format!("TLV: {} not found", tlv.t))
    }


}
//Some TLV information for DOCSIS 4

use std::cmp::PartialEq;
use std::collections::HashMap;
use crate::TLV::TLV;
use crate::MIB::MIB;
#[derive(Clone)]
pub(crate) enum DATATYPE {
    UCHAR,
    UINT,
    USHORT,
    STRING,
    STRINGZERO,
    AGGREGATE,
    MIB,


}
#[derive(Clone)]
pub(crate) struct DOCSIS4TLV {
    pub(crate) t: u8,
    pub(crate) description: String,
    pub(crate) dataType: DATATYPE,
    pub(crate) tlv: TLV,
    pub(crate) sub_tlvs: HashMap<u8, DOCSIS4TLV>,
    pub(crate) mib: Option<MIB>,
}
impl Default for DOCSIS4TLV {
    fn default() -> DOCSIS4TLV {
        DOCSIS4TLV {
            t: 0,
            description: "".to_string(),
            dataType: DATATYPE::UCHAR,
            tlv: TLV { t: 0, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
            mib: None,
        }
    }
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
            DATATYPE::USHORT => {
                let mut v = 0;
                for i in 0..self.tlv.l {
                    v = v << 8;
                    v = v | self.tlv.v[i as usize] as u32;
                }
                Ok(v)
            },
            DATATYPE::STRING => {
                Err("Not a number".to_string())
            },
            DATATYPE::STRINGZERO => {
                Err("Not a number".to_string())
            },
            DATATYPE::AGGREGATE => {
                Err("Sub TLVs not working yet".to_string())
            },
            DATATYPE::MIB => {
                let _ = MIB::from_bytes(self.tlv.v);
                Err("Not yet supported".to_string())
            }
            _ => {
                Err("Not yet supported".to_string())
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
            DATATYPE::USHORT => {
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
            },
            DATATYPE::STRINGZERO => {
                Err("Not a number".to_string())
            },
            DATATYPE::AGGREGATE => {
                Err("Sub TLVs not working yet".to_string())
            },
            _ => {
                Err("Not yet supported".to_string())
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
            DATATYPE::USHORT => {
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
            },
            DATATYPE::STRINGZERO => {
                let mut s = String::new();
                for i in 0..self.tlv.l {
                    s.push_str(format!("{}", self.tlv.v[i as usize] as char).as_str());
                }
                Ok(s)
            },
            DATATYPE::AGGREGATE => {
                Err("Sub TLVs not working yet".to_string())
            },
            _ => {
                Err("Not yet supported".to_string())
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
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(tlv.t, tlv);
    let mut tlv = DOCSIS4TLV {
        t: 0x02,
        description: "UpstreamChannelId".to_string(),
        dataType: DATATYPE::UCHAR,
        tlv: TLV { t: 0x02, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(tlv.t, tlv);
    let mut tlv = DOCSIS4TLV {
        t: 0x03,
        description: "NetworkAccess".to_string(),
        dataType: DATATYPE::UCHAR,
        tlv: TLV { t: 0x03, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(tlv.t, tlv);

    let mut tlv = DOCSIS4TLV {
        t: 0x0b,
        description: "SnmpMibObject".to_string(),
        dataType: DATATYPE::MIB,
        tlv: TLV { t: 0x0b, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(tlv.t, tlv);


    let mut tlv = DOCSIS4TLV {
        t: 0x18,
        description: "UsServiceFlow".to_string(),
        dataType: DATATYPE::AGGREGATE,
        tlv: TLV { t: 0x18, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
        };
    let mut sub_tlv = DOCSIS4TLV {
        t: 0x01,
        description: "UsServiceFlowRef".to_string(),
        dataType: DATATYPE::USHORT,
        tlv: TLV { t: 0x01, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    tlv.sub_tlvs.insert(sub_tlv.t, sub_tlv);
    let mut sub_tlv = DOCSIS4TLV {
        t: 0x02,
        description: "UsServiceFlowId".to_string(),
        dataType: DATATYPE::UINT,
        tlv: TLV { t: 0x02, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    tlv.sub_tlvs.insert(sub_tlv.t, sub_tlv);
    let mut sub_tlv = DOCSIS4TLV {
        t: 0x03,
        description: "ServiceIdentifier".to_string(),
        dataType: DATATYPE::USHORT,
        tlv: TLV { t: 0x03, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    tlv.sub_tlvs.insert(sub_tlv.t, sub_tlv);
    let mut sub_tlv = DOCSIS4TLV {
        t: 0x04,
        description: "ServiceClassName".to_string(),
        dataType: DATATYPE::STRINGZERO,
        tlv: TLV { t: 0x04, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    tlv.sub_tlvs.insert(sub_tlv.t, sub_tlv);
    let mut sub_tlv = DOCSIS4TLV {
        t: 0x06,
        description: "QosParamSetType".to_string(),
        dataType: DATATYPE::UCHAR,
        tlv: TLV { t: 0x06, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    tlv.sub_tlvs.insert(sub_tlv.t, sub_tlv);
    let mut sub_tlv = DOCSIS4TLV {
            t: 0x07,
            description: "TrafficPriority".to_string(),
            dataType: DATATYPE::UCHAR,
            tlv: TLV { t: 0x07, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    tlv.sub_tlvs.insert(sub_tlv.t, sub_tlv);
    let mut sub_tlv = DOCSIS4TLV {
            t: 0x08,
            description: "MaxRateSustained".to_string(),
            dataType: DATATYPE::UINT,
            tlv: TLV { t: 0x08, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    tlv.sub_tlvs.insert(sub_tlv.t, sub_tlv);
    let mut sub_tlv = DOCSIS4TLV {
            t: 0x09,
            description: "MaxTrafficBurst".to_string(),
            dataType: DATATYPE::UINT,
            tlv: TLV { t: 0x09, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    tlv.sub_tlvs.insert(sub_tlv.t, sub_tlv);
    let mut sub_tlv = DOCSIS4TLV {
            t: 0x0a,
            description: "MinReservedRate".to_string(),
            dataType: DATATYPE::UINT,
            tlv: TLV { t: 0x0a, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    tlv.sub_tlvs.insert(sub_tlv.t, sub_tlv);
    let mut sub_tlv = DOCSIS4TLV {
            t: 0x0b,
            description: "MinResPacketSize".to_string(),
            dataType: DATATYPE::USHORT,
            tlv: TLV { t: 0x0b, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    tlv.sub_tlvs.insert(sub_tlv.t, sub_tlv);
    let mut sub_tlv = DOCSIS4TLV {
            t: 0x0c,
            description: "ActQosParamsTimeout".to_string(),
            dataType: DATATYPE::USHORT,
            tlv: TLV { t: 0x0c, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    tlv.sub_tlvs.insert(sub_tlv.t, sub_tlv);
    let mut sub_tlv = DOCSIS4TLV {
            t: 0x0d,
            description: "AdmQosParamsTimeout".to_string(),
            dataType: DATATYPE::USHORT,
            tlv: TLV { t: 0x0d, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    tlv.sub_tlvs.insert(sub_tlv.t, sub_tlv);

    d4_defs.insert(tlv.t, tlv);

    d4_defs
}




/*


DocsisTlvs["24"]["subTlvs"]["14"] = {}
DocsisTlvs["24"]["subTlvs"]["14"]["description"] = "MaxConcatenatedBurst"
DocsisTlvs["24"]["subTlvs"]["14"]["hex"] = "0e"
DocsisTlvs["24"]["subTlvs"]["14"]["datatype"] = "ushort"
DocsisTlvs["24"]["subTlvs"]["14"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["15"] = {}
DocsisTlvs["24"]["subTlvs"]["15"]["description"] = "SchedulingType"
DocsisTlvs["24"]["subTlvs"]["15"]["hex"] = "0f"
DocsisTlvs["24"]["subTlvs"]["15"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["15"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["16"] = {}
DocsisTlvs["24"]["subTlvs"]["16"]["description"] = "RequestOrTxPolicy"
DocsisTlvs["24"]["subTlvs"]["16"]["hex"] = "10"
DocsisTlvs["24"]["subTlvs"]["16"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["16"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["17"] = {}
DocsisTlvs["24"]["subTlvs"]["17"]["description"] = "NominalPollInterval"
DocsisTlvs["24"]["subTlvs"]["17"]["hex"] = "11"
DocsisTlvs["24"]["subTlvs"]["17"]["datatype"] = "uint"
DocsisTlvs["24"]["subTlvs"]["17"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["18"] = {}
DocsisTlvs["24"]["subTlvs"]["18"]["description"] = "ToleratedPollJitter"
DocsisTlvs["24"]["subTlvs"]["18"]["hex"] = "12"
DocsisTlvs["24"]["subTlvs"]["18"]["datatype"] = "uint"
DocsisTlvs["24"]["subTlvs"]["18"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["19"] = {}
DocsisTlvs["24"]["subTlvs"]["19"]["description"] = "UnsolicitedGrantSize"
DocsisTlvs["24"]["subTlvs"]["19"]["hex"] = "13"
DocsisTlvs["24"]["subTlvs"]["19"]["datatype"] = "ushort"
DocsisTlvs["24"]["subTlvs"]["19"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["20"] = {}
DocsisTlvs["24"]["subTlvs"]["20"]["description"] = "NominalGrantInterval"
DocsisTlvs["24"]["subTlvs"]["20"]["hex"] = "14"
DocsisTlvs["24"]["subTlvs"]["20"]["datatype"] = "uint"
DocsisTlvs["24"]["subTlvs"]["20"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["21"] = {}
DocsisTlvs["24"]["subTlvs"]["21"]["description"] = "ToleratedGrantJitter"
DocsisTlvs["24"]["subTlvs"]["21"]["hex"] = "15"
DocsisTlvs["24"]["subTlvs"]["21"]["datatype"] = "uint"
DocsisTlvs["24"]["subTlvs"]["21"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["22"] = {}
DocsisTlvs["24"]["subTlvs"]["22"]["description"] = "GrantsPerInterval"
DocsisTlvs["24"]["subTlvs"]["22"]["hex"] = "16"
DocsisTlvs["24"]["subTlvs"]["22"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["22"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["23"] = {}
DocsisTlvs["24"]["subTlvs"]["23"]["description"] = "IpTosOverwrite"
DocsisTlvs["24"]["subTlvs"]["23"]["hex"] = "17"
DocsisTlvs["24"]["subTlvs"]["23"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["23"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["26"] = {}
DocsisTlvs["24"]["subTlvs"]["26"]["description"] = "MultipliertoNumberofBytesRequested"
DocsisTlvs["24"]["subTlvs"]["26"]["hex"] = "1a"
DocsisTlvs["24"]["subTlvs"]["26"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["26"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["27"] = {}
DocsisTlvs["24"]["subTlvs"]["27"]["description"] = "UpstreamPeakTrafficRate"
DocsisTlvs["24"]["subTlvs"]["27"]["hex"] = "1b"
DocsisTlvs["24"]["subTlvs"]["27"]["datatype"] = "uint"
DocsisTlvs["24"]["subTlvs"]["27"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["31"] = {}
DocsisTlvs["24"]["subTlvs"]["31"]["description"] = "ServiceFlowRequiredAttributeMask"
DocsisTlvs["24"]["subTlvs"]["31"]["hex"] = "1f"
DocsisTlvs["24"]["subTlvs"]["31"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["31"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["32"] = {}
DocsisTlvs["24"]["subTlvs"]["32"]["description"] = "ServiceFlowForbiddenAttributeMask"
DocsisTlvs["24"]["subTlvs"]["32"]["hex"] = "20"
DocsisTlvs["24"]["subTlvs"]["32"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["32"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["33"] = {}
DocsisTlvs["24"]["subTlvs"]["33"]["description"] = "ServiceFlowAttributeAggregationRuleMask"
DocsisTlvs["24"]["subTlvs"]["33"]["hex"] = "21"
DocsisTlvs["24"]["subTlvs"]["33"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["33"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["34"] = {}
DocsisTlvs["24"]["subTlvs"]["34"]["description"] = "ApplicationIdentifier"
DocsisTlvs["24"]["subTlvs"]["34"]["hex"] = "22"
DocsisTlvs["24"]["subTlvs"]["34"]["datatype"] = "uint"
DocsisTlvs["24"]["subTlvs"]["34"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["35"] = {}
DocsisTlvs["24"]["subTlvs"]["35"]["description"] = "BufferControl"
DocsisTlvs["24"]["subTlvs"]["35"]["hex"] = "23"
DocsisTlvs["24"]["subTlvs"]["35"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["35"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["35"]["subTlvs"]["01"] = {}
DocsisTlvs["24"]["subTlvs"]["35"]["subTlvs"]["01"]["description"] = "MinimumBuffer"
DocsisTlvs["24"]["subTlvs"]["35"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["24"]["subTlvs"]["35"]["subTlvs"]["01"]["datatype"] = "uint"
DocsisTlvs["24"]["subTlvs"]["35"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["35"]["subTlvs"]["02"] = {}
DocsisTlvs["24"]["subTlvs"]["35"]["subTlvs"]["02"]["description"] = "TargetBuffer"
DocsisTlvs["24"]["subTlvs"]["35"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["24"]["subTlvs"]["35"]["subTlvs"]["02"]["datatype"] = "uint"
DocsisTlvs["24"]["subTlvs"]["35"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["35"]["subTlvs"]["03"] = {}
DocsisTlvs["24"]["subTlvs"]["35"]["subTlvs"]["03"]["description"] = "MaximumBuffer"
DocsisTlvs["24"]["subTlvs"]["35"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["24"]["subTlvs"]["35"]["subTlvs"]["03"]["datatype"] = "uint"
DocsisTlvs["24"]["subTlvs"]["35"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["36"] = {}
DocsisTlvs["24"]["subTlvs"]["36"]["description"] = "UpstreamAggregateServiceFlowReference"
DocsisTlvs["24"]["subTlvs"]["36"]["hex"] = "24"
DocsisTlvs["24"]["subTlvs"]["36"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["36"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["37"] = {}
DocsisTlvs["24"]["subTlvs"]["37"]["description"] = "UpstreamMESPReference"
DocsisTlvs["24"]["subTlvs"]["37"]["hex"] = "25"
DocsisTlvs["24"]["subTlvs"]["37"]["datatype"] = "ushort"
DocsisTlvs["24"]["subTlvs"]["37"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["40"] = {}
DocsisTlvs["24"]["subTlvs"]["40"]["description"] = "AQMEncodings"
DocsisTlvs["24"]["subTlvs"]["40"]["hex"] = "28"
DocsisTlvs["24"]["subTlvs"]["40"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["40"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["40"]["subTlvs"]["01"] = {}
DocsisTlvs["24"]["subTlvs"]["40"]["subTlvs"]["01"]["description"] = "SFAQMDisable"
DocsisTlvs["24"]["subTlvs"]["40"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["24"]["subTlvs"]["40"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["40"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["40"]["subTlvs"]["02"] = {}
DocsisTlvs["24"]["subTlvs"]["40"]["subTlvs"]["02"]["description"] = "SFAQMLatencyTarget"
DocsisTlvs["24"]["subTlvs"]["40"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["24"]["subTlvs"]["40"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["40"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["41"] = {}
DocsisTlvs["24"]["subTlvs"]["41"]["description"] = "DataRateUnitSetting"
DocsisTlvs["24"]["subTlvs"]["41"]["hex"] = "29"
DocsisTlvs["24"]["subTlvs"]["41"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["41"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["description"] = "VendorSpecific"
DocsisTlvs["24"]["subTlvs"]["43"]["hex"] = "2b"
DocsisTlvs["24"]["subTlvs"]["43"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["01"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["01"]["description"] = "CMLoadBalancingPolicyID"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["01"]["datatype"] = "uint"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["02"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["02"]["description"] = "CMLoadBalancingPriority"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["02"]["datatype"] = "uint"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["03"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["03"]["description"] = "CMLoadBalancingGroupID"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["03"]["datatype"] = "uint"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["04"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["04"]["description"] = "CMRangingClassIDExtension"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["04"]["datatype"] = "ushort"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["description"] = "L2VPNEncoding"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["01"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["01"]["description"] = "VPNIdentifier"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["description"] = "NSIEncapsulation"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["01"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["01"]["description"] = "ServiceMultiplexingValueOther"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["01"]["datatype"] = "lenzero"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["02"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["02"]["description"] = "NSIEncapsulationSingleQTag"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["02"]["datatype"] = "ushort"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["03"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["03"]["description"] = "NSIEncapsulationDualQTag"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["03"]["datatype"] = "dual_qtag"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["description"] = "ServiceMultiplexingValueMPLSPW"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["01"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["01"]["description"] = "MPLSPseudowireID"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["01"]["datatype"] = "uint"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["02"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["02"]["description"] = "MPLSPeerIpAddress"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["02"]["datatype"] = "char_ip_ip6"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["03"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["03"]["description"] = "MPLSPseudowireType"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["03"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["04"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["04"]["description"] = "MPLSBackupPseudowireID"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["04"]["datatype"] = "uint"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["05"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["05"]["description"] = "MPLSBackupPeerIpAddress"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["05"]["datatype"] = "char_ip_ip6"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["05"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["05"]["description"] = "ServiceMultiplexingValueL2TPv3Peer"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["05"]["datatype"] = "char_ip_ip6"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["description"] = "IEEE8021ahEncapsulation"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["01"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["01"]["description"] = "ITCIEncapsulation"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["02"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["02"]["description"] = "BDAEncapsulation"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["03"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["03"]["description"] = "BTCIEncapsulation"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["04"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["04"]["description"] = "ITPIDEncapsulation"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["05"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["05"]["description"] = "IPCPEncapsulation"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["05"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["06"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["06"]["description"] = "IDEIEncapsulation"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["06"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["07"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["07"]["description"] = "IUCAEncapsulation"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["07"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["08"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["08"]["description"] = "ISIDEncapsulation"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["09"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["09"]["description"] = "BTPIDEncapsulation"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["09"]["hex"] = "09"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["09"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["09"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["10"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["10"]["description"] = "BPCPEncapsulation"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["10"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["11"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["11"]["description"] = "BDEIEncapsulation"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["11"]["hex"] = "0b"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["11"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["11"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["12"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["12"]["description"] = "BVIDEncapsulation"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["12"]["hex"] = "0c"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["12"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["12"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["08"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["08"]["description"] = "ServiceMultiplexingValueIEEE8021adSTPID"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["03"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["03"]["description"] = "eSAFEDHCPSnooping"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["04"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["04"]["description"] = "CMInterfaceMaskCMIMSubtype"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["05"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["05"]["description"] = "AttachmentGroupID"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["05"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["06"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["06"]["description"] = "SourceAttachmentIndividualID"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["06"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["07"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["07"]["description"] = "TargetAttachmentIndividualID"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["07"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["08"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["08"]["description"] = "IngressUserPriority"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["08"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["09"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["09"]["description"] = "UserPriorityRange"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["09"]["hex"] = "09"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["09"]["datatype"] = "char_list"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["09"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["10"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["10"]["description"] = "L2VPNSADescriptorSubtype"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["10"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["12"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["12"]["description"] = "PseudowireType"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["12"]["hex"] = "0c"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["12"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["12"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["13"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["13"]["description"] = "L2VPNMode"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["13"]["hex"] = "0d"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["13"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["13"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["description"] = "TPIDTranslation"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["hex"] = "0e"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["01"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["01"]["description"] = "UpstreamTPIDTranslation"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["02"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["02"]["description"] = "DownstreamTPIDTranslation"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["03"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["03"]["description"] = "UpstreamSTPIDTranslation"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["04"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["04"]["description"] = "DownstreamSTPIDTranslation"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["05"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["05"]["description"] = "UpstreamBTPIDTranslation"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["05"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["06"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["06"]["description"] = "DownstreamBTPIDTranslation"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["06"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["07"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["07"]["description"] = "UpstreamITPIDTranslation"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["07"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["08"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["08"]["description"] = "DownstreamITPIDTranslation"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["description"] = "L2CPProcessing"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["hex"] = "0f"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["01"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["01"]["description"] = "L2CPTunnelMode"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["02"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["02"]["description"] = "L2CPDMACAddress"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["02"]["datatype"] = "ether"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["03"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["03"]["description"] = "L2CPOverwrotingDMACAddress"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["03"]["datatype"] = "ether"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["16"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["16"]["description"] = "DACDisableEnableConfiguration"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["16"]["hex"] = "10"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["16"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["16"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["18"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["18"]["description"] = "PseudowireClass"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["18"]["hex"] = "12"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["18"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["18"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["description"] = "ServiceDelimiter"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["hex"] = "13"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["01"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["01"]["description"] = "CVIDDelimiter"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["02"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["02"]["description"] = "SVIDDelimiter"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["03"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["03"]["description"] = "ISIDDelimiter"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["04"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["04"]["description"] = "BVIDDelimiter"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["description"] = "VirtualSwitchInstanceEncoding"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["hex"] = "14"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["01"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["01"]["description"] = "VPLSClass"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["02"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["02"]["description"] = "ETreeRole"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["03"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["03"]["description"] = "ETreeRootVID"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["04"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["04"]["description"] = "ETreeLeafVID"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["description"] = "BGPAttribute"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["hex"] = "15"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["01"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["01"]["description"] = "BGPVPNID"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["01"]["datatype"] = "uint"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["02"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["02"]["description"] = "RouteDistinguisher"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["03"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["03"]["description"] = "RouteTargetImport"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["04"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["04"]["description"] = "RouteTargetExport"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["05"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["05"]["description"] = "CEIDVEID"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["05"]["datatype"] = "ushort"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["23"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["23"]["description"] = "PseudowireSignaling"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["23"]["hex"] = "17"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["23"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["23"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["description"] = "SOAMSubtype"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["hex"] = "18"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["description"] = "MEPConfiguration"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["01"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["01"]["description"] = "MDLevel"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["02"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["02"]["description"] = "MDName"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["02"]["datatype"] = "string"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["03"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["03"]["description"] = "MAName"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["03"]["datatype"] = "string"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["04"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["04"]["description"] = "MEPID"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["04"]["datatype"] = "ushort"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["description"] = "RemoteMEPConfiguration"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["01"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["01"]["description"] = "RemoteMDLevel"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["02"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["02"]["description"] = "RemoteMDName"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["02"]["datatype"] = "string"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["03"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["03"]["description"] = "RemoteMAName"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["03"]["datatype"] = "string"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["04"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["04"]["description"] = "RemoteMEPID"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["04"]["datatype"] = "ushort"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["description"] = "FaultManagementConfiguration"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["01"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["01"]["description"] = "ContinuityCheckMessages"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["02"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["02"]["description"] = "LoopbackFunction"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["03"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["03"]["description"] = "LinktraceFunction"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["03"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["description"] = "PerformanceManagementConfiguration"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["description"] = "FrameDelayMeasurement"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["01"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["01"]["description"] = "FrameDelayMeasurementEnable"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["02"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["02"]["description"] = "FrameDelayMeasurementOneWayTwoWay"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["03"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["03"]["description"] = "FrameDelayMeasurementTransmissionPeriodicity"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["03"]["datatype"] = "ushort"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["description"] = "FrameLossMeasurement"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["01"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["01"]["description"] = "FrameLossMeasurementEnable"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["02"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["02"]["description"] = "FrameLossMeasurementTransmissionPeriodicity"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["26"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["26"]["description"] = "L2VPNDSID"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["26"]["hex"] = "1a"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["26"]["datatype"] = "uint24"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["26"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["description"] = "VendorSpecificL2VPNSubtype"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["hex"] = "2b"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"]["08"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"]["08"]["description"] = "VendorIdentifier"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["06"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["06"]["description"] = "ExtendedCMTSMICConfigurationSetting"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["06"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["01"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["01"]["description"] = "ExtendedCMTSMICHMACtype"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["02"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["02"]["description"] = "ExtendedCMTSMICBitmap"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["03"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["03"]["description"] = "ExplicitExtendedCMTSMICDigest"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["07"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["07"]["description"] = "SAVAuthorizationEncoding"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["07"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["01"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["01"]["description"] = "SAVGroupName"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["01"]["datatype"] = "string"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["description"] = "SAVStaticPrefixRule"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["01"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["01"]["description"] = "SAVStaticPrefixAddress"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["01"]["datatype"] = "ip_ip6"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["02"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["02"]["description"] = "SAVStaticPrefixLength"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["08"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["08"]["description"] = "VendorIdentifier"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["09"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["09"]["description"] = "CMAttributeMasks"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["09"]["hex"] = "09"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["09"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["01"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["01"]["description"] = "CMDownstreamRequiredAttributeMask"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["02"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["02"]["description"] = "CMDownstreamForbiddenAttributeMask"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["03"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["03"]["description"] = "CMUpstreamRequiredAttributeMask"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["04"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["04"]["description"] = "CMUpstreamForbiddenAttributeMask"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["description"] = "IPMulticastJoinAuthorization"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["01"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["01"]["description"] = "IPMulticastProfileName"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["01"]["datatype"] = "string"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["description"] = "IPMulticastJoinAuthStaticSessionRule"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["01"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["01"]["description"] = "MulticastRulePriority"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["02"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["02"]["description"] = "AuthorizationAction"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["03"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["03"]["description"] = "SourcePrefixAddress"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["03"]["datatype"] = "ip_ip6"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["04"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["04"]["description"] = "SourcePrefixLength"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["04"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["05"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["05"]["description"] = "GroupPrefixAddress"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["05"]["datatype"] = "ip_ip6"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["06"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["06"]["description"] = "GroupPrefixLength"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["06"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["03"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["03"]["description"] = "MaximumMulticastSessions"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["03"]["datatype"] = "ushort"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["11"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["11"]["description"] = "ServiceTypeIdentifier"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["11"]["hex"] = "0b"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["11"]["datatype"] = "string"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["11"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["12"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["12"]["description"] = "DEMARCAutoConfiguration"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["12"]["hex"] = "0c"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["12"]["datatype"] = "aggregate"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["01"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["01"]["description"] = "DACDisableEnableConfig"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["02"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["02"]["description"] = "CMIMEncoding"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["03"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["03"]["description"] = "UpstreamServiceClassName"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["03"]["datatype"] = "strzero"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["04"] = {}
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["04"]["description"] = "DownstreamServiceClassName"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["04"]["datatype"] = "strzero"
DocsisTlvs["24"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["04"]["subTlvs"] = {}
*/
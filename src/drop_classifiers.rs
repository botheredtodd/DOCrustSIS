use std::collections::HashMap;
use crate::tlv::TLV;

use crate::d4::DATATYPE;
use crate::d4::DOCSIS4TLV;

pub(crate) fn upstream_drop_classifiers() -> HashMap<u8, DOCSIS4TLV> {
    let mut d4_defs: HashMap<u8, DOCSIS4TLV> = HashMap::new();
    let sub_tlv = DOCSIS4TLV {
        t: 0x01,
        description: "ClassifierReference".to_string(),
        data_type: DATATYPE::UCHAR,
        tlv: TLV { t: 0x01, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
        t: 0x02,
        description: "ClassifierIdentifier".to_string(),
        data_type: DATATYPE::USHORT,
        tlv: TLV { t: 0x02, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
        t: 0x05,
        description: "RulePriority".to_string(),
        data_type: DATATYPE::UCHAR,
        tlv: TLV { t: 0x05, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
        t: 0x07,
        description: "DynamicServiceChangeAction".to_string(),
        data_type: DATATYPE::UCHAR,
        tlv: TLV { t: 0x07, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let mut sub_tlv = DOCSIS4TLV {
        t: 0x09,
        description: "DynamicServiceChangeAction".to_string(),
        data_type: DATATYPE::AGGREGATE,
        tlv: TLV { t: 0x09, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    let sub_sub_tlv = DOCSIS4TLV {
        t: 0x01,
        description: "IPv4Tos".to_string(),
        data_type: DATATYPE::HEXSTR,
        tlv: TLV { t: 0x01, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_tlv.sub_tlvs.insert(sub_sub_tlv.t, sub_sub_tlv);
    let sub_sub_tlv = DOCSIS4TLV {
        t: 0x02,
        description: "IPProtocol".to_string(),
        data_type: DATATYPE::USHORT,
        tlv: TLV { t: 0x02, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_tlv.sub_tlvs.insert(sub_sub_tlv.t, sub_sub_tlv);
    let sub_sub_tlv = DOCSIS4TLV {
        t: 0x03,
        description: "IPv4SourceAddress".to_string(),
        data_type: DATATYPE::ENCODE_IP,
        tlv: TLV { t: 0x03, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_tlv.sub_tlvs.insert(sub_sub_tlv.t, sub_sub_tlv);
    let sub_sub_tlv = DOCSIS4TLV {
        t: 0x04,
        description: "IPv4SourceMask".to_string(),
        data_type: DATATYPE::ENCODE_IP,
        tlv: TLV { t: 0x04, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_tlv.sub_tlvs.insert(sub_sub_tlv.t, sub_sub_tlv);
    let sub_sub_tlv = DOCSIS4TLV {
        t: 0x05,
        description: "IPv4DestinationAddress".to_string(),
        data_type: DATATYPE::ENCODE_IP,
        tlv: TLV { t: 0x05, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_tlv.sub_tlvs.insert(sub_sub_tlv.t, sub_sub_tlv);
    let sub_sub_tlv = DOCSIS4TLV {
        t: 0x06,
        description: "IPv4DestinationMask".to_string(),
        data_type: DATATYPE::ENCODE_IP,
        tlv: TLV { t: 0x06, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_tlv.sub_tlvs.insert(sub_sub_tlv.t, sub_sub_tlv);
    let sub_sub_tlv = DOCSIS4TLV {
        t: 0x07,
        description: "SourcePortStart".to_string(),
        data_type: DATATYPE::USHORT,
        tlv: TLV { t: 0x07, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_tlv.sub_tlvs.insert(sub_sub_tlv.t, sub_sub_tlv);
    let sub_sub_tlv = DOCSIS4TLV {
        t: 0x08,
        description: "SourcePortEnd".to_string(),
        data_type: DATATYPE::USHORT,
        tlv: TLV { t: 0x08, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_tlv.sub_tlvs.insert(sub_sub_tlv.t, sub_sub_tlv);
    let sub_sub_tlv = DOCSIS4TLV {
        t: 0x09,
        description: "DestinationPortStart".to_string(),
        data_type: DATATYPE::USHORT,
        tlv: TLV { t: 0x09, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_tlv.sub_tlvs.insert(sub_sub_tlv.t, sub_sub_tlv);
    let sub_sub_tlv = DOCSIS4TLV {
        t: 0x0a,
        description: "DestinationPortEnd".to_string(),
        data_type: DATATYPE::USHORT,
        tlv: TLV { t: 0x0a, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_tlv.sub_tlvs.insert(sub_sub_tlv.t, sub_sub_tlv);
    //TODO: Add the rest of the sub_sub_tlvs
    d4_defs.insert(sub_tlv.t, sub_tlv);

    let sub_tlv = DOCSIS4TLV {
        t: 0x0d,
        description: "CMInterfaceMaskEncoding".to_string(),
        data_type: DATATYPE::HEXSTR,
        tlv: TLV { t: 0x0d, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);

    d4_defs
}

/*


DocsisTlvs["60"]["subTlvs"]["10"] = {}
DocsisTlvs["60"]["subTlvs"]["10"]["description"] = "EthernetLLCPacketClassification"
DocsisTlvs["60"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["60"]["subTlvs"]["10"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["10"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["10"]["subTlvs"]["01"]["description"] = "DestinationMACAddress"
DocsisTlvs["60"]["subTlvs"]["10"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["10"]["subTlvs"]["01"]["datatype"] = "(encode_ethermask)"
DocsisTlvs["60"]["subTlvs"]["10"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["10"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["10"]["subTlvs"]["02"]["description"] = "SourceMACAddress"
DocsisTlvs["60"]["subTlvs"]["10"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["10"]["subTlvs"]["02"]["datatype"] = "ether"
DocsisTlvs["60"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["10"]["subTlvs"]["03"] = {}
DocsisTlvs["60"]["subTlvs"]["10"]["subTlvs"]["03"]["description"] = "EthertypeDSAPMacType"
DocsisTlvs["60"]["subTlvs"]["10"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["60"]["subTlvs"]["10"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["10"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["11"] = {}
DocsisTlvs["60"]["subTlvs"]["11"]["description"] = "IEEE8021PQPacketClassification"
DocsisTlvs["60"]["subTlvs"]["11"]["hex"] = "0b"
DocsisTlvs["60"]["subTlvs"]["11"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["11"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["11"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["11"]["subTlvs"]["01"]["description"] = "UserPriority"
DocsisTlvs["60"]["subTlvs"]["11"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["11"]["subTlvs"]["01"]["datatype"] = "char_list"
DocsisTlvs["60"]["subTlvs"]["11"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["11"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["11"]["subTlvs"]["02"]["description"] = "VlanID"
DocsisTlvs["60"]["subTlvs"]["11"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["11"]["subTlvs"]["02"]["datatype"] = "ushort"
DocsisTlvs["60"]["subTlvs"]["11"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["12"] = {}
DocsisTlvs["60"]["subTlvs"]["12"]["description"] = "IPv6PacketClassification"
DocsisTlvs["60"]["subTlvs"]["12"]["hex"] = "0c"
DocsisTlvs["60"]["subTlvs"]["12"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["01"]["description"] = "IPv6TrafficClassRangeandMask"
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["02"]["description"] = "IPv6FlowLabel"
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["03"] = {}
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["03"]["description"] = "IPv6NextHeaderType"
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["03"]["datatype"] = "ushort"
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["04"] = {}
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["04"]["description"] = "IPv6SourceAddress"
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["04"]["datatype"] = "encode_ip"
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["05"] = {}
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["05"]["description"] = "IPv6SourcePrefixLength"
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["05"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["06"] = {}
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["06"]["description"] = "IPv6DestinationAddress"
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["06"]["datatype"] = "encode_ip"
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["07"] = {}
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["07"]["description"] = "IPv6DestinationPrefixLength"
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["07"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["12"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["14"] = {}
DocsisTlvs["60"]["subTlvs"]["14"]["description"] = "STagCTagFrameClassification"
DocsisTlvs["60"]["subTlvs"]["14"]["hex"] = "0e"
DocsisTlvs["60"]["subTlvs"]["14"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["01"]["description"] = "STPID"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["02"]["description"] = "SVID"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["03"] = {}
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["03"]["description"] = "SPCP"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["04"] = {}
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["04"]["description"] = "SDEI"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["05"] = {}
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["05"]["description"] = "CTPID"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["05"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["06"] = {}
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["06"]["description"] = "CVID"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["06"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["07"] = {}
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["07"]["description"] = "CPCP"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["07"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["08"] = {}
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["08"]["description"] = "CCFI"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["09"] = {}
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["09"]["description"] = "STCI"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["09"]["hex"] = "09"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["09"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["09"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["10"] = {}
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["10"]["description"] = "CTCI"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["10"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["14"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["15"] = {}
DocsisTlvs["60"]["subTlvs"]["15"]["description"] = "IEEE8021ahPacketClassification"
DocsisTlvs["60"]["subTlvs"]["15"]["hex"] = "0f"
DocsisTlvs["60"]["subTlvs"]["15"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["01"]["description"] = "ITPID"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["02"]["description"] = "ISID"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["03"] = {}
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["03"]["description"] = "ITCI"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["04"] = {}
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["04"]["description"] = "IPCP"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["05"] = {}
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["05"]["description"] = "IDEI"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["05"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["06"] = {}
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["06"]["description"] = "IUCA"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["06"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["07"] = {}
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["07"]["description"] = "BTPID"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["07"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["08"] = {}
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["08"]["description"] = "BTCI"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["09"] = {}
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["09"]["description"] = "BPCP"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["09"]["hex"] = "09"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["09"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["09"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["10"] = {}
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["10"]["description"] = "BDEI"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["10"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["11"] = {}
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["11"]["description"] = "BVID"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["11"]["hex"] = "0b"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["11"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["11"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["12"] = {}
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["12"]["description"] = "BDA"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["12"]["hex"] = "0c"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["12"]["datatype"] = "ether"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["12"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["13"] = {}
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["13"]["description"] = "BSA"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["13"]["hex"] = "0d"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["13"]["datatype"] = "ether"
DocsisTlvs["60"]["subTlvs"]["15"]["subTlvs"]["13"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["16"] = {}
DocsisTlvs["60"]["subTlvs"]["16"]["description"] = "ICMPv4ICMPv6PacketClassification"
DocsisTlvs["60"]["subTlvs"]["16"]["hex"] = "10"
DocsisTlvs["60"]["subTlvs"]["16"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["16"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["16"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["16"]["subTlvs"]["01"]["description"] = "ICMPv4ICMPv6TypeStart"
DocsisTlvs["60"]["subTlvs"]["16"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["16"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["16"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["16"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["16"]["subTlvs"]["02"]["description"] = "ICMPv4ICMPv6TypeEnd"
DocsisTlvs["60"]["subTlvs"]["16"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["16"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["16"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["17"] = {}
DocsisTlvs["60"]["subTlvs"]["17"]["description"] = "MPLSClassificationEncoding"
DocsisTlvs["60"]["subTlvs"]["17"]["hex"] = "11"
DocsisTlvs["60"]["subTlvs"]["17"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["17"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["17"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["17"]["subTlvs"]["01"]["description"] = "MPLSTCbits"
DocsisTlvs["60"]["subTlvs"]["17"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["17"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["17"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["17"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["17"]["subTlvs"]["02"]["description"] = "MPLSLabel"
DocsisTlvs["60"]["subTlvs"]["17"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["17"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["17"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["description"] = "VendorSpecific"
DocsisTlvs["60"]["subTlvs"]["43"]["hex"] = "2b"
DocsisTlvs["60"]["subTlvs"]["43"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["01"]["description"] = "CMLoadBalancingPolicyID"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["01"]["datatype"] = "uint"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["02"]["description"] = "CMLoadBalancingPriority"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["02"]["datatype"] = "uint"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["03"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["03"]["description"] = "CMLoadBalancingGroupID"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["03"]["datatype"] = "uint"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["04"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["04"]["description"] = "CMRangingClassIDExtension"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["04"]["datatype"] = "ushort"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["description"] = "L2VPNEncoding"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["01"]["description"] = "VPNIdentifier"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["description"] = "NSIEncapsulation"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["01"]["description"] = "ServiceMultiplexingValueOther"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["01"]["datatype"] = "lenzero"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["02"]["description"] = "NSIEncapsulationSingleQTag"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["02"]["datatype"] = "ushort"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["03"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["03"]["description"] = "NSIEncapsulationDualQTag"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["03"]["datatype"] = "dual_qtag"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["description"] = "ServiceMultiplexingValueMPLSPW"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["01"]["description"] = "MPLSPseudowireID"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["01"]["datatype"] = "uint"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["02"]["description"] = "MPLSPeerIpAddress"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["02"]["datatype"] = "char_ip_ip6"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["03"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["03"]["description"] = "MPLSPseudowireType"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["03"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["04"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["04"]["description"] = "MPLSBackupPseudowireID"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["04"]["datatype"] = "uint"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["05"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["05"]["description"] = "MPLSBackupPeerIpAddress"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["05"]["datatype"] = "char_ip_ip6"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["05"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["05"]["description"] = "ServiceMultiplexingValueL2TPv3Peer"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["05"]["datatype"] = "char_ip_ip6"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["description"] = "IEEE8021ahEncapsulation"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["01"]["description"] = "ITCIEncapsulation"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["02"]["description"] = "BDAEncapsulation"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["03"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["03"]["description"] = "BTCIEncapsulation"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["04"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["04"]["description"] = "ITPIDEncapsulation"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["05"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["05"]["description"] = "IPCPEncapsulation"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["05"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["06"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["06"]["description"] = "IDEIEncapsulation"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["06"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["07"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["07"]["description"] = "IUCAEncapsulation"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["07"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["08"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["08"]["description"] = "ISIDEncapsulation"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["09"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["09"]["description"] = "BTPIDEncapsulation"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["09"]["hex"] = "09"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["09"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["09"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["10"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["10"]["description"] = "BPCPEncapsulation"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["10"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["11"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["11"]["description"] = "BDEIEncapsulation"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["11"]["hex"] = "0b"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["11"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["11"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["12"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["12"]["description"] = "BVIDEncapsulation"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["12"]["hex"] = "0c"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["12"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["12"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["08"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["08"]["description"] = "ServiceMultiplexingValueIEEE8021adSTPID"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["03"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["03"]["description"] = "eSAFEDHCPSnooping"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["04"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["04"]["description"] = "CMInterfaceMaskCMIMSubtype"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["05"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["05"]["description"] = "AttachmentGroupID"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["05"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["06"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["06"]["description"] = "SourceAttachmentIndividualID"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["06"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["07"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["07"]["description"] = "TargetAttachmentIndividualID"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["07"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["08"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["08"]["description"] = "IngressUserPriority"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["08"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["09"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["09"]["description"] = "UserPriorityRange"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["09"]["hex"] = "09"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["09"]["datatype"] = "char_list"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["09"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["10"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["10"]["description"] = "L2VPNSADescriptorSubtype"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["10"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["12"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["12"]["description"] = "PseudowireType"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["12"]["hex"] = "0c"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["12"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["12"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["13"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["13"]["description"] = "L2VPNMode"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["13"]["hex"] = "0d"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["13"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["13"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["description"] = "TPIDTranslation"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["hex"] = "0e"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["01"]["description"] = "UpstreamTPIDTranslation"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["02"]["description"] = "DownstreamTPIDTranslation"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["03"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["03"]["description"] = "UpstreamSTPIDTranslation"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["04"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["04"]["description"] = "DownstreamSTPIDTranslation"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["05"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["05"]["description"] = "UpstreamBTPIDTranslation"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["05"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["06"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["06"]["description"] = "DownstreamBTPIDTranslation"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["06"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["07"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["07"]["description"] = "UpstreamITPIDTranslation"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["07"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["08"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["08"]["description"] = "DownstreamITPIDTranslation"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["description"] = "L2CPProcessing"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["hex"] = "0f"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["01"]["description"] = "L2CPTunnelMode"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["02"]["description"] = "L2CPDMACAddress"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["02"]["datatype"] = "ether"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["03"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["03"]["description"] = "L2CPOverwrotingDMACAddress"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["03"]["datatype"] = "ether"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["16"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["16"]["description"] = "DACDisableEnableConfiguration"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["16"]["hex"] = "10"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["16"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["16"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["18"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["18"]["description"] = "PseudowireClass"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["18"]["hex"] = "12"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["18"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["18"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["description"] = "ServiceDelimiter"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["hex"] = "13"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["01"]["description"] = "CVIDDelimiter"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["02"]["description"] = "SVIDDelimiter"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["03"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["03"]["description"] = "ISIDDelimiter"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["04"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["04"]["description"] = "BVIDDelimiter"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["description"] = "VirtualSwitchInstanceEncoding"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["hex"] = "14"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["01"]["description"] = "VPLSClass"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["02"]["description"] = "ETreeRole"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["03"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["03"]["description"] = "ETreeRootVID"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["04"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["04"]["description"] = "ETreeLeafVID"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["description"] = "BGPAttribute"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["hex"] = "15"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["01"]["description"] = "BGPVPNID"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["01"]["datatype"] = "uint"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["02"]["description"] = "RouteDistinguisher"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["03"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["03"]["description"] = "RouteTargetImport"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["04"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["04"]["description"] = "RouteTargetExport"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["05"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["05"]["description"] = "CEIDVEID"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["05"]["datatype"] = "ushort"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["23"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["23"]["description"] = "PseudowireSignaling"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["23"]["hex"] = "17"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["23"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["23"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["description"] = "SOAMSubtype"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["hex"] = "18"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["description"] = "MEPConfiguration"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["01"]["description"] = "MDLevel"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["02"]["description"] = "MDName"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["02"]["datatype"] = "string"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["03"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["03"]["description"] = "MAName"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["03"]["datatype"] = "string"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["04"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["04"]["description"] = "MEPID"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["04"]["datatype"] = "ushort"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["description"] = "RemoteMEPConfiguration"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["01"]["description"] = "RemoteMDLevel"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["02"]["description"] = "RemoteMDName"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["02"]["datatype"] = "string"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["03"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["03"]["description"] = "RemoteMAName"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["03"]["datatype"] = "string"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["04"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["04"]["description"] = "RemoteMEPID"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["04"]["datatype"] = "ushort"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["description"] = "FaultManagementConfiguration"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["01"]["description"] = "ContinuityCheckMessages"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["02"]["description"] = "LoopbackFunction"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["03"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["03"]["description"] = "LinktraceFunction"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["03"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["description"] = "PerformanceManagementConfiguration"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["description"] = "FrameDelayMeasurement"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["01"]["description"] = "FrameDelayMeasurementEnable"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["02"]["description"] = "FrameDelayMeasurementOneWayTwoWay"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["03"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["03"]["description"] = "FrameDelayMeasurementTransmissionPeriodicity"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["03"]["datatype"] = "ushort"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["description"] = "FrameLossMeasurement"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["01"]["description"] = "FrameLossMeasurementEnable"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["02"]["description"] = "FrameLossMeasurementTransmissionPeriodicity"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["26"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["26"]["description"] = "L2VPNDSID"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["26"]["hex"] = "1a"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["26"]["datatype"] = "uint24"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["26"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["description"] = "VendorSpecificL2VPNSubtype"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["hex"] = "2b"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"]["08"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"]["08"]["description"] = "VendorIdentifier"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["06"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["06"]["description"] = "ExtendedCMTSMICConfigurationSetting"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["06"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["01"]["description"] = "ExtendedCMTSMICHMACtype"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["02"]["description"] = "ExtendedCMTSMICBitmap"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["03"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["03"]["description"] = "ExplicitExtendedCMTSMICDigest"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["07"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["07"]["description"] = "SAVAuthorizationEncoding"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["07"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["01"]["description"] = "SAVGroupName"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["01"]["datatype"] = "string"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["description"] = "SAVStaticPrefixRule"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["01"]["description"] = "SAVStaticPrefixAddress"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["01"]["datatype"] = "ip_ip6"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["02"]["description"] = "SAVStaticPrefixLength"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["08"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["08"]["description"] = "VendorIdentifier"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["09"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["09"]["description"] = "CMAttributeMasks"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["09"]["hex"] = "09"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["09"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["01"]["description"] = "CMDownstreamRequiredAttributeMask"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["02"]["description"] = "CMDownstreamForbiddenAttributeMask"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["03"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["03"]["description"] = "CMUpstreamRequiredAttributeMask"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["04"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["04"]["description"] = "CMUpstreamForbiddenAttributeMask"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["description"] = "IPMulticastJoinAuthorization"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["01"]["description"] = "IPMulticastProfileName"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["01"]["datatype"] = "string"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["description"] = "IPMulticastJoinAuthStaticSessionRule"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["01"]["description"] = "MulticastRulePriority"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["02"]["description"] = "AuthorizationAction"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["03"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["03"]["description"] = "SourcePrefixAddress"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["03"]["datatype"] = "ip_ip6"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["04"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["04"]["description"] = "SourcePrefixLength"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["04"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["05"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["05"]["description"] = "GroupPrefixAddress"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["05"]["datatype"] = "ip_ip6"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["06"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["06"]["description"] = "GroupPrefixLength"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["06"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["03"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["03"]["description"] = "MaximumMulticastSessions"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["03"]["datatype"] = "ushort"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["11"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["11"]["description"] = "ServiceTypeIdentifier"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["11"]["hex"] = "0b"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["11"]["datatype"] = "string"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["11"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["12"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["12"]["description"] = "DEMARCAutoConfiguration"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["12"]["hex"] = "0c"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["12"]["datatype"] = "aggregate"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["01"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["01"]["description"] = "DACDisableEnableConfig"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["02"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["02"]["description"] = "CMIMEncoding"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["03"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["03"]["description"] = "UpstreamServiceClassName"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["03"]["datatype"] = "strzero"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["04"] = {}
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["04"]["description"] = "DownstreamServiceClassName"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["04"]["datatype"] = "strzero"
DocsisTlvs["60"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["04"]["subTlvs"] = {}

 */
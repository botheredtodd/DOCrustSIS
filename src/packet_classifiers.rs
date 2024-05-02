use std::collections::HashMap;
use crate::tlv::TLV;

use crate::d4::DATATYPE;
use crate::d4::DOCSIS4TLV;
pub(crate) fn packet_classifiers() -> HashMap<u8, DOCSIS4TLV> {
    let mut d4_defs: HashMap<u8, DOCSIS4TLV> = HashMap::new();
    let mut sub_tlv = DOCSIS4TLV {
        t: 0x01,
        description: "ClassifierRef".to_string(),
        data_type: DATATYPE::UCHAR,
        tlv: TLV { t: 0x01, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);

    d4_defs
}

/*

DocsisTlvs["22"] = {}
DocsisTlvs["22"]["description"] = "UsPacketClass"
DocsisTlvs["22"]["hex"] = "16"
DocsisTlvs["22"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"] = {}


DocsisTlvs["22"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["02"]["description"] = "ClassifierId"
DocsisTlvs["22"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["02"]["datatype"] = "ushort"
DocsisTlvs["22"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["03"] = {}
DocsisTlvs["22"]["subTlvs"]["03"]["description"] = "ServiceFlowRef"
DocsisTlvs["22"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["22"]["subTlvs"]["03"]["datatype"] = "ushort"
DocsisTlvs["22"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["04"] = {}
DocsisTlvs["22"]["subTlvs"]["04"]["description"] = "ServiceFlowId"
DocsisTlvs["22"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["22"]["subTlvs"]["04"]["datatype"] = "uint"
DocsisTlvs["22"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["05"] = {}
DocsisTlvs["22"]["subTlvs"]["05"]["description"] = "RulePriority"
DocsisTlvs["22"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["22"]["subTlvs"]["05"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["06"] = {}
DocsisTlvs["22"]["subTlvs"]["06"]["description"] = "ActivationState"
DocsisTlvs["22"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["22"]["subTlvs"]["06"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["07"] = {}
DocsisTlvs["22"]["subTlvs"]["07"]["description"] = "DscAction"
DocsisTlvs["22"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["22"]["subTlvs"]["07"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["09"] = {}
DocsisTlvs["22"]["subTlvs"]["09"]["description"] = "IpPacketClassifier"
DocsisTlvs["22"]["subTlvs"]["09"]["hex"] = "09"
DocsisTlvs["22"]["subTlvs"]["09"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["01"]["description"] = "IpTos"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["02"]["description"] = "IpProto"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["02"]["datatype"] = "ushort"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["03"] = {}
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["03"]["description"] = "IpSrcAddr"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["03"]["datatype"] = "encode_ip"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["04"] = {}
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["04"]["description"] = "IpSrcMask"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["04"]["datatype"] = "encode_ip"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["05"] = {}
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["05"]["description"] = "IpDstAddr"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["05"]["datatype"] = "encode_ip"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["06"] = {}
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["06"]["description"] = "IpDstMask"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["06"]["datatype"] = "encode_ip"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["07"] = {}
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["07"]["description"] = "SrcPortStart"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["07"]["datatype"] = "ushort"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["08"] = {}
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["08"]["description"] = "SrcPortEnd"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["08"]["datatype"] = "ushort"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["09"] = {}
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["09"]["description"] = "DstPortStart"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["09"]["hex"] = "09"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["09"]["datatype"] = "ushort"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["09"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["10"] = {}
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["10"]["description"] = "DstPortEnd"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["10"]["datatype"] = "ushort"
DocsisTlvs["22"]["subTlvs"]["09"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["10"] = {}
DocsisTlvs["22"]["subTlvs"]["10"]["description"] = "LLCPacketClassifier"
DocsisTlvs["22"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["22"]["subTlvs"]["10"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["10"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["10"]["subTlvs"]["01"]["description"] = "DstMacAddress"
DocsisTlvs["22"]["subTlvs"]["10"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["10"]["subTlvs"]["01"]["datatype"] = "(encode_ethermask)"
DocsisTlvs["22"]["subTlvs"]["10"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["10"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["10"]["subTlvs"]["02"]["description"] = "SrcMacAddress"
DocsisTlvs["22"]["subTlvs"]["10"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["10"]["subTlvs"]["02"]["datatype"] = "ether"
DocsisTlvs["22"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["10"]["subTlvs"]["03"] = {}
DocsisTlvs["22"]["subTlvs"]["10"]["subTlvs"]["03"]["description"] = "EtherType"
DocsisTlvs["22"]["subTlvs"]["10"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["22"]["subTlvs"]["10"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["10"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["11"] = {}
DocsisTlvs["22"]["subTlvs"]["11"]["description"] = "IEEE802Classifier"
DocsisTlvs["22"]["subTlvs"]["11"]["hex"] = "0b"
DocsisTlvs["22"]["subTlvs"]["11"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["11"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["11"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["11"]["subTlvs"]["01"]["description"] = "UserPriority"
DocsisTlvs["22"]["subTlvs"]["11"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["11"]["subTlvs"]["01"]["datatype"] = "char_list"
DocsisTlvs["22"]["subTlvs"]["11"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["11"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["11"]["subTlvs"]["02"]["description"] = "VlanID"
DocsisTlvs["22"]["subTlvs"]["11"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["11"]["subTlvs"]["02"]["datatype"] = "ushort"
DocsisTlvs["22"]["subTlvs"]["11"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["12"] = {}
DocsisTlvs["22"]["subTlvs"]["12"]["description"] = "PcIPv6PacketClassification"
DocsisTlvs["22"]["subTlvs"]["12"]["hex"] = "0c"
DocsisTlvs["22"]["subTlvs"]["12"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["01"]["description"] = "PcIPv6TrafficClassRangeAndMask"
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["02"]["description"] = "PcIPv6FlowLabel"
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["03"] = {}
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["03"]["description"] = "PcIPv6NextHeaderType"
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["03"]["datatype"] = "ushort"
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["04"] = {}
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["04"]["description"] = "PcIPv6SourceAddress"
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["04"]["datatype"] = "encode_ip"
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["05"] = {}
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["05"]["description"] = "PcIPv6SourcePrefixLength"
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["05"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["06"] = {}
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["06"]["description"] = "PcIPv6DestAddress"
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["06"]["datatype"] = "encode_ip"
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["07"] = {}
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["07"]["description"] = "PcIPv6DestPrefixLength"
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["07"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["12"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["13"] = {}
DocsisTlvs["22"]["subTlvs"]["13"]["description"] = "PcCMIMEncoding"
DocsisTlvs["22"]["subTlvs"]["13"]["hex"] = "0d"
DocsisTlvs["22"]["subTlvs"]["13"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["13"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["14"] = {}
DocsisTlvs["22"]["subTlvs"]["14"]["description"] = "STagCTagFrameClassification"
DocsisTlvs["22"]["subTlvs"]["14"]["hex"] = "0e"
DocsisTlvs["22"]["subTlvs"]["14"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["01"]["description"] = "STPID"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["02"]["description"] = "SVID"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["03"] = {}
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["03"]["description"] = "SPCP"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["04"] = {}
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["04"]["description"] = "SDEI"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["05"] = {}
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["05"]["description"] = "CTPID"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["05"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["06"] = {}
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["06"]["description"] = "CVID"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["06"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["07"] = {}
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["07"]["description"] = "CPCP"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["07"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["08"] = {}
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["08"]["description"] = "CCFI"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["09"] = {}
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["09"]["description"] = "STCI"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["09"]["hex"] = "09"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["09"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["09"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["10"] = {}
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["10"]["description"] = "CTCI"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["10"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["14"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["15"] = {}
DocsisTlvs["22"]["subTlvs"]["15"]["description"] = "IEEE8021ahPacketClassification"
DocsisTlvs["22"]["subTlvs"]["15"]["hex"] = "0f"
DocsisTlvs["22"]["subTlvs"]["15"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["01"]["description"] = "ITPID"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["02"]["description"] = "ISID"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["03"] = {}
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["03"]["description"] = "ITCI"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["04"] = {}
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["04"]["description"] = "IPCP"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["05"] = {}
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["05"]["description"] = "IDEI"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["05"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["06"] = {}
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["06"]["description"] = "IUCA"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["06"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["07"] = {}
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["07"]["description"] = "BTPID"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["07"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["08"] = {}
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["08"]["description"] = "BTCI"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["09"] = {}
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["09"]["description"] = "BPCP"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["09"]["hex"] = "09"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["09"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["09"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["10"] = {}
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["10"]["description"] = "BDEI"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["10"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["11"] = {}
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["11"]["description"] = "BVID"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["11"]["hex"] = "0b"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["11"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["11"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["12"] = {}
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["12"]["description"] = "BDA"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["12"]["hex"] = "0c"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["12"]["datatype"] = "ether"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["12"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["13"] = {}
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["13"]["description"] = "BSA"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["13"]["hex"] = "0d"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["13"]["datatype"] = "ether"
DocsisTlvs["22"]["subTlvs"]["15"]["subTlvs"]["13"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["16"] = {}
DocsisTlvs["22"]["subTlvs"]["16"]["description"] = "ICMPv4ICMPv6PacketClassification"
DocsisTlvs["22"]["subTlvs"]["16"]["hex"] = "10"
DocsisTlvs["22"]["subTlvs"]["16"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["16"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["16"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["16"]["subTlvs"]["01"]["description"] = "ICMPv4ICMPv6TypeStart"
DocsisTlvs["22"]["subTlvs"]["16"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["16"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["16"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["16"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["16"]["subTlvs"]["02"]["description"] = "ICMPv4ICMPv6TypeEnd"
DocsisTlvs["22"]["subTlvs"]["16"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["16"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["16"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["17"] = {}
DocsisTlvs["22"]["subTlvs"]["17"]["description"] = "MPLSClassificationEncoding"
DocsisTlvs["22"]["subTlvs"]["17"]["hex"] = "11"
DocsisTlvs["22"]["subTlvs"]["17"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["17"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["17"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["17"]["subTlvs"]["01"]["description"] = "MPLSTCbits"
DocsisTlvs["22"]["subTlvs"]["17"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["17"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["17"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["17"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["17"]["subTlvs"]["02"]["description"] = "MPLSLabel"
DocsisTlvs["22"]["subTlvs"]["17"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["17"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["17"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["description"] = "VendorSpecific"
DocsisTlvs["22"]["subTlvs"]["43"]["hex"] = "2b"
DocsisTlvs["22"]["subTlvs"]["43"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["01"]["description"] = "CMLoadBalancingPolicyID"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["01"]["datatype"] = "uint"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["02"]["description"] = "CMLoadBalancingPriority"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["02"]["datatype"] = "uint"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["03"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["03"]["description"] = "CMLoadBalancingGroupID"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["03"]["datatype"] = "uint"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["04"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["04"]["description"] = "CMRangingClassIDExtension"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["04"]["datatype"] = "ushort"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["description"] = "L2VPNEncoding"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["01"]["description"] = "VPNIdentifier"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["description"] = "NSIEncapsulation"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["01"]["description"] = "ServiceMultiplexingValueOther"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["01"]["datatype"] = "lenzero"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["02"]["description"] = "NSIEncapsulationSingleQTag"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["02"]["datatype"] = "ushort"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["03"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["03"]["description"] = "NSIEncapsulationDualQTag"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["03"]["datatype"] = "dual_qtag"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["description"] = "ServiceMultiplexingValueMPLSPW"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["01"]["description"] = "MPLSPseudowireID"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["01"]["datatype"] = "uint"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["02"]["description"] = "MPLSPeerIpAddress"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["02"]["datatype"] = "char_ip_ip6"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["03"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["03"]["description"] = "MPLSPseudowireType"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["03"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["04"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["04"]["description"] = "MPLSBackupPseudowireID"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["04"]["datatype"] = "uint"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["05"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["05"]["description"] = "MPLSBackupPeerIpAddress"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["05"]["datatype"] = "char_ip_ip6"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["05"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["05"]["description"] = "ServiceMultiplexingValueL2TPv3Peer"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["05"]["datatype"] = "char_ip_ip6"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["description"] = "IEEE8021ahEncapsulation"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["01"]["description"] = "ITCIEncapsulation"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["02"]["description"] = "BDAEncapsulation"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["03"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["03"]["description"] = "BTCIEncapsulation"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["04"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["04"]["description"] = "ITPIDEncapsulation"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["05"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["05"]["description"] = "IPCPEncapsulation"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["05"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["06"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["06"]["description"] = "IDEIEncapsulation"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["06"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["07"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["07"]["description"] = "IUCAEncapsulation"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["07"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["08"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["08"]["description"] = "ISIDEncapsulation"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["09"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["09"]["description"] = "BTPIDEncapsulation"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["09"]["hex"] = "09"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["09"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["09"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["10"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["10"]["description"] = "BPCPEncapsulation"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["10"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["11"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["11"]["description"] = "BDEIEncapsulation"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["11"]["hex"] = "0b"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["11"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["11"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["12"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["12"]["description"] = "BVIDEncapsulation"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["12"]["hex"] = "0c"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["12"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["12"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["08"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["08"]["description"] = "ServiceMultiplexingValueIEEE8021adSTPID"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["03"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["03"]["description"] = "eSAFEDHCPSnooping"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["04"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["04"]["description"] = "CMInterfaceMaskCMIMSubtype"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["05"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["05"]["description"] = "AttachmentGroupID"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["05"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["06"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["06"]["description"] = "SourceAttachmentIndividualID"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["06"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["07"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["07"]["description"] = "TargetAttachmentIndividualID"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["07"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["08"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["08"]["description"] = "IngressUserPriority"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["08"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["09"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["09"]["description"] = "UserPriorityRange"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["09"]["hex"] = "09"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["09"]["datatype"] = "char_list"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["09"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["10"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["10"]["description"] = "L2VPNSADescriptorSubtype"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["10"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["12"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["12"]["description"] = "PseudowireType"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["12"]["hex"] = "0c"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["12"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["12"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["13"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["13"]["description"] = "L2VPNMode"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["13"]["hex"] = "0d"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["13"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["13"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["description"] = "TPIDTranslation"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["hex"] = "0e"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["01"]["description"] = "UpstreamTPIDTranslation"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["02"]["description"] = "DownstreamTPIDTranslation"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["03"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["03"]["description"] = "UpstreamSTPIDTranslation"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["04"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["04"]["description"] = "DownstreamSTPIDTranslation"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["05"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["05"]["description"] = "UpstreamBTPIDTranslation"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["05"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["06"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["06"]["description"] = "DownstreamBTPIDTranslation"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["06"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["07"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["07"]["description"] = "UpstreamITPIDTranslation"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["07"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["08"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["08"]["description"] = "DownstreamITPIDTranslation"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["description"] = "L2CPProcessing"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["hex"] = "0f"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["01"]["description"] = "L2CPTunnelMode"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["02"]["description"] = "L2CPDMACAddress"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["02"]["datatype"] = "ether"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["03"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["03"]["description"] = "L2CPOverwrotingDMACAddress"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["03"]["datatype"] = "ether"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["16"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["16"]["description"] = "DACDisableEnableConfiguration"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["16"]["hex"] = "10"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["16"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["16"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["18"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["18"]["description"] = "PseudowireClass"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["18"]["hex"] = "12"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["18"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["18"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["description"] = "ServiceDelimiter"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["hex"] = "13"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["01"]["description"] = "CVIDDelimiter"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["02"]["description"] = "SVIDDelimiter"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["03"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["03"]["description"] = "ISIDDelimiter"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["04"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["04"]["description"] = "BVIDDelimiter"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["description"] = "VirtualSwitchInstanceEncoding"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["hex"] = "14"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["01"]["description"] = "VPLSClass"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["02"]["description"] = "ETreeRole"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["03"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["03"]["description"] = "ETreeRootVID"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["04"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["04"]["description"] = "ETreeLeafVID"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["description"] = "BGPAttribute"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["hex"] = "15"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["01"]["description"] = "BGPVPNID"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["01"]["datatype"] = "uint"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["02"]["description"] = "RouteDistinguisher"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["03"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["03"]["description"] = "RouteTargetImport"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["04"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["04"]["description"] = "RouteTargetExport"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["05"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["05"]["description"] = "CEIDVEID"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["05"]["datatype"] = "ushort"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["23"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["23"]["description"] = "PseudowireSignaling"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["23"]["hex"] = "17"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["23"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["23"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["description"] = "SOAMSubtype"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["hex"] = "18"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["description"] = "MEPConfiguration"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["01"]["description"] = "MDLevel"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["02"]["description"] = "MDName"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["02"]["datatype"] = "string"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["03"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["03"]["description"] = "MAName"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["03"]["datatype"] = "string"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["04"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["04"]["description"] = "MEPID"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["04"]["datatype"] = "ushort"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["description"] = "RemoteMEPConfiguration"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["01"]["description"] = "RemoteMDLevel"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["02"]["description"] = "RemoteMDName"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["02"]["datatype"] = "string"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["03"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["03"]["description"] = "RemoteMAName"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["03"]["datatype"] = "string"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["04"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["04"]["description"] = "RemoteMEPID"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["04"]["datatype"] = "ushort"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["description"] = "FaultManagementConfiguration"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["01"]["description"] = "ContinuityCheckMessages"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["02"]["description"] = "LoopbackFunction"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["03"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["03"]["description"] = "LinktraceFunction"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["03"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["description"] = "PerformanceManagementConfiguration"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["description"] = "FrameDelayMeasurement"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["01"]["description"] = "FrameDelayMeasurementEnable"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["02"]["description"] = "FrameDelayMeasurementOneWayTwoWay"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["03"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["03"]["description"] = "FrameDelayMeasurementTransmissionPeriodicity"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["03"]["datatype"] = "ushort"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["description"] = "FrameLossMeasurement"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["01"]["description"] = "FrameLossMeasurementEnable"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["02"]["description"] = "FrameLossMeasurementTransmissionPeriodicity"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["26"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["26"]["description"] = "L2VPNDSID"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["26"]["hex"] = "1a"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["26"]["datatype"] = "uint24"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["26"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["description"] = "VendorSpecificL2VPNSubtype"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["hex"] = "2b"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"]["08"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"]["08"]["description"] = "VendorIdentifier"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["06"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["06"]["description"] = "ExtendedCMTSMICConfigurationSetting"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["06"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["01"]["description"] = "ExtendedCMTSMICHMACtype"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["02"]["description"] = "ExtendedCMTSMICBitmap"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["03"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["03"]["description"] = "ExplicitExtendedCMTSMICDigest"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["07"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["07"]["description"] = "SAVAuthorizationEncoding"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["07"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["01"]["description"] = "SAVGroupName"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["01"]["datatype"] = "string"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["description"] = "SAVStaticPrefixRule"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["01"]["description"] = "SAVStaticPrefixAddress"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["01"]["datatype"] = "ip_ip6"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["02"]["description"] = "SAVStaticPrefixLength"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["08"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["08"]["description"] = "VendorIdentifier"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["09"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["09"]["description"] = "CMAttributeMasks"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["09"]["hex"] = "09"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["09"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["01"]["description"] = "CMDownstreamRequiredAttributeMask"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["02"]["description"] = "CMDownstreamForbiddenAttributeMask"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["03"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["03"]["description"] = "CMUpstreamRequiredAttributeMask"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["04"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["04"]["description"] = "CMUpstreamForbiddenAttributeMask"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["description"] = "IPMulticastJoinAuthorization"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["01"]["description"] = "IPMulticastProfileName"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["01"]["datatype"] = "string"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["description"] = "IPMulticastJoinAuthStaticSessionRule"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["01"]["description"] = "MulticastRulePriority"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["02"]["description"] = "AuthorizationAction"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["03"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["03"]["description"] = "SourcePrefixAddress"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["03"]["datatype"] = "ip_ip6"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["04"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["04"]["description"] = "SourcePrefixLength"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["04"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["05"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["05"]["description"] = "GroupPrefixAddress"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["05"]["datatype"] = "ip_ip6"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["06"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["06"]["description"] = "GroupPrefixLength"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["06"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["03"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["03"]["description"] = "MaximumMulticastSessions"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["03"]["datatype"] = "ushort"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["11"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["11"]["description"] = "ServiceTypeIdentifier"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["11"]["hex"] = "0b"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["11"]["datatype"] = "string"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["11"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["12"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["12"]["description"] = "DEMARCAutoConfiguration"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["12"]["hex"] = "0c"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["12"]["datatype"] = "aggregate"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["01"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["01"]["description"] = "DACDisableEnableConfig"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["02"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["02"]["description"] = "CMIMEncoding"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["03"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["03"]["description"] = "UpstreamServiceClassName"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["03"]["datatype"] = "strzero"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["04"] = {}
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["04"]["description"] = "DownstreamServiceClassName"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["04"]["datatype"] = "strzero"
DocsisTlvs["22"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["23"] = {}
DocsisTlvs["23"]["description"] = "DsPacketClass"
DocsisTlvs["23"]["hex"] = "17"
DocsisTlvs["23"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"] = {}


DocsisTlvs["23"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["02"]["description"] = "ClassifierId"
DocsisTlvs["23"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["02"]["datatype"] = "ushort"
DocsisTlvs["23"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["03"] = {}
DocsisTlvs["23"]["subTlvs"]["03"]["description"] = "ServiceFlowRef"
DocsisTlvs["23"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["23"]["subTlvs"]["03"]["datatype"] = "ushort"
DocsisTlvs["23"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["04"] = {}
DocsisTlvs["23"]["subTlvs"]["04"]["description"] = "ServiceFlowId"
DocsisTlvs["23"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["23"]["subTlvs"]["04"]["datatype"] = "uint"
DocsisTlvs["23"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["05"] = {}
DocsisTlvs["23"]["subTlvs"]["05"]["description"] = "RulePriority"
DocsisTlvs["23"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["23"]["subTlvs"]["05"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["06"] = {}
DocsisTlvs["23"]["subTlvs"]["06"]["description"] = "ActivationState"
DocsisTlvs["23"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["23"]["subTlvs"]["06"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["07"] = {}
DocsisTlvs["23"]["subTlvs"]["07"]["description"] = "DscAction"
DocsisTlvs["23"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["23"]["subTlvs"]["07"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["09"] = {}
DocsisTlvs["23"]["subTlvs"]["09"]["description"] = "IpPacketClassifier"
DocsisTlvs["23"]["subTlvs"]["09"]["hex"] = "09"
DocsisTlvs["23"]["subTlvs"]["09"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["01"]["description"] = "IpTos"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["02"]["description"] = "IpProto"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["02"]["datatype"] = "ushort"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["03"] = {}
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["03"]["description"] = "IpSrcAddr"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["03"]["datatype"] = "encode_ip"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["04"] = {}
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["04"]["description"] = "IpSrcMask"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["04"]["datatype"] = "encode_ip"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["05"] = {}
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["05"]["description"] = "IpDstAddr"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["05"]["datatype"] = "encode_ip"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["06"] = {}
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["06"]["description"] = "IpDstMask"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["06"]["datatype"] = "encode_ip"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["07"] = {}
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["07"]["description"] = "SrcPortStart"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["07"]["datatype"] = "ushort"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["08"] = {}
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["08"]["description"] = "SrcPortEnd"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["08"]["datatype"] = "ushort"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["09"] = {}
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["09"]["description"] = "DstPortStart"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["09"]["hex"] = "09"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["09"]["datatype"] = "ushort"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["09"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["10"] = {}
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["10"]["description"] = "DstPortEnd"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["10"]["datatype"] = "ushort"
DocsisTlvs["23"]["subTlvs"]["09"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["10"] = {}
DocsisTlvs["23"]["subTlvs"]["10"]["description"] = "LLCPacketClassifier"
DocsisTlvs["23"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["23"]["subTlvs"]["10"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["10"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["10"]["subTlvs"]["01"]["description"] = "DstMacAddress"
DocsisTlvs["23"]["subTlvs"]["10"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["10"]["subTlvs"]["01"]["datatype"] = "(encode_ethermask)"
DocsisTlvs["23"]["subTlvs"]["10"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["10"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["10"]["subTlvs"]["02"]["description"] = "SrcMacAddress"
DocsisTlvs["23"]["subTlvs"]["10"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["10"]["subTlvs"]["02"]["datatype"] = "ether"
DocsisTlvs["23"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["10"]["subTlvs"]["03"] = {}
DocsisTlvs["23"]["subTlvs"]["10"]["subTlvs"]["03"]["description"] = "EtherType"
DocsisTlvs["23"]["subTlvs"]["10"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["23"]["subTlvs"]["10"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["10"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["11"] = {}
DocsisTlvs["23"]["subTlvs"]["11"]["description"] = "IEEE802Classifier"
DocsisTlvs["23"]["subTlvs"]["11"]["hex"] = "0b"
DocsisTlvs["23"]["subTlvs"]["11"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["11"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["11"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["11"]["subTlvs"]["01"]["description"] = "UserPriority"
DocsisTlvs["23"]["subTlvs"]["11"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["11"]["subTlvs"]["01"]["datatype"] = "char_list"
DocsisTlvs["23"]["subTlvs"]["11"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["11"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["11"]["subTlvs"]["02"]["description"] = "VlanID"
DocsisTlvs["23"]["subTlvs"]["11"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["11"]["subTlvs"]["02"]["datatype"] = "ushort"
DocsisTlvs["23"]["subTlvs"]["11"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["12"] = {}
DocsisTlvs["23"]["subTlvs"]["12"]["description"] = "PcIPv6PacketClassification"
DocsisTlvs["23"]["subTlvs"]["12"]["hex"] = "0c"
DocsisTlvs["23"]["subTlvs"]["12"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["01"]["description"] = "PcIPv6TrafficClassRangeAndMask"
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["02"]["description"] = "PcIPv6FlowLabel"
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["03"] = {}
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["03"]["description"] = "PcIPv6NextHeaderType"
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["03"]["datatype"] = "ushort"
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["04"] = {}
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["04"]["description"] = "PcIPv6SourceAddress"
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["04"]["datatype"] = "encode_ip"
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["05"] = {}
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["05"]["description"] = "PcIPv6SourcePrefixLength"
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["05"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["06"] = {}
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["06"]["description"] = "PcIPv6DestAddress"
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["06"]["datatype"] = "encode_ip"
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["07"] = {}
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["07"]["description"] = "PcIPv6DestPrefixLength"
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["07"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["12"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["13"] = {}
DocsisTlvs["23"]["subTlvs"]["13"]["description"] = "PcCMIMEncoding"
DocsisTlvs["23"]["subTlvs"]["13"]["hex"] = "0d"
DocsisTlvs["23"]["subTlvs"]["13"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["13"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["14"] = {}
DocsisTlvs["23"]["subTlvs"]["14"]["description"] = "STagCTagFrameClassification"
DocsisTlvs["23"]["subTlvs"]["14"]["hex"] = "0e"
DocsisTlvs["23"]["subTlvs"]["14"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["01"]["description"] = "STPID"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["02"]["description"] = "SVID"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["03"] = {}
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["03"]["description"] = "SPCP"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["04"] = {}
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["04"]["description"] = "SDEI"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["05"] = {}
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["05"]["description"] = "CTPID"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["05"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["06"] = {}
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["06"]["description"] = "CVID"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["06"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["07"] = {}
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["07"]["description"] = "CPCP"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["07"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["08"] = {}
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["08"]["description"] = "CCFI"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["09"] = {}
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["09"]["description"] = "STCI"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["09"]["hex"] = "09"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["09"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["09"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["10"] = {}
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["10"]["description"] = "CTCI"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["10"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["14"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["15"] = {}
DocsisTlvs["23"]["subTlvs"]["15"]["description"] = "IEEE8021ahPacketClassification"
DocsisTlvs["23"]["subTlvs"]["15"]["hex"] = "0f"
DocsisTlvs["23"]["subTlvs"]["15"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["01"]["description"] = "ITPID"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["02"]["description"] = "ISID"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["03"] = {}
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["03"]["description"] = "ITCI"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["04"] = {}
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["04"]["description"] = "IPCP"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["05"] = {}
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["05"]["description"] = "IDEI"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["05"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["06"] = {}
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["06"]["description"] = "IUCA"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["06"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["07"] = {}
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["07"]["description"] = "BTPID"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["07"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["08"] = {}
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["08"]["description"] = "BTCI"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["09"] = {}
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["09"]["description"] = "BPCP"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["09"]["hex"] = "09"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["09"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["09"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["10"] = {}
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["10"]["description"] = "BDEI"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["10"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["11"] = {}
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["11"]["description"] = "BVID"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["11"]["hex"] = "0b"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["11"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["11"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["12"] = {}
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["12"]["description"] = "BDA"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["12"]["hex"] = "0c"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["12"]["datatype"] = "ether"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["12"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["13"] = {}
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["13"]["description"] = "BSA"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["13"]["hex"] = "0d"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["13"]["datatype"] = "ether"
DocsisTlvs["23"]["subTlvs"]["15"]["subTlvs"]["13"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["16"] = {}
DocsisTlvs["23"]["subTlvs"]["16"]["description"] = "ICMPv4ICMPv6PacketClassification"
DocsisTlvs["23"]["subTlvs"]["16"]["hex"] = "10"
DocsisTlvs["23"]["subTlvs"]["16"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["16"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["16"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["16"]["subTlvs"]["01"]["description"] = "ICMPv4ICMPv6TypeStart"
DocsisTlvs["23"]["subTlvs"]["16"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["16"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["16"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["16"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["16"]["subTlvs"]["02"]["description"] = "ICMPv4ICMPv6TypeEnd"
DocsisTlvs["23"]["subTlvs"]["16"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["16"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["16"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["17"] = {}
DocsisTlvs["23"]["subTlvs"]["17"]["description"] = "MPLSClassificationEncoding"
DocsisTlvs["23"]["subTlvs"]["17"]["hex"] = "11"
DocsisTlvs["23"]["subTlvs"]["17"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["17"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["17"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["17"]["subTlvs"]["01"]["description"] = "MPLSTCbits"
DocsisTlvs["23"]["subTlvs"]["17"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["17"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["17"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["17"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["17"]["subTlvs"]["02"]["description"] = "MPLSLabel"
DocsisTlvs["23"]["subTlvs"]["17"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["17"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["17"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["description"] = "VendorSpecific"
DocsisTlvs["23"]["subTlvs"]["43"]["hex"] = "2b"
DocsisTlvs["23"]["subTlvs"]["43"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["01"]["description"] = "CMLoadBalancingPolicyID"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["01"]["datatype"] = "uint"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["02"]["description"] = "CMLoadBalancingPriority"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["02"]["datatype"] = "uint"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["03"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["03"]["description"] = "CMLoadBalancingGroupID"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["03"]["datatype"] = "uint"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["04"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["04"]["description"] = "CMRangingClassIDExtension"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["04"]["datatype"] = "ushort"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["description"] = "L2VPNEncoding"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["01"]["description"] = "VPNIdentifier"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["description"] = "NSIEncapsulation"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["01"]["description"] = "ServiceMultiplexingValueOther"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["01"]["datatype"] = "lenzero"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["02"]["description"] = "NSIEncapsulationSingleQTag"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["02"]["datatype"] = "ushort"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["03"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["03"]["description"] = "NSIEncapsulationDualQTag"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["03"]["datatype"] = "dual_qtag"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["description"] = "ServiceMultiplexingValueMPLSPW"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["01"]["description"] = "MPLSPseudowireID"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["01"]["datatype"] = "uint"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["02"]["description"] = "MPLSPeerIpAddress"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["02"]["datatype"] = "char_ip_ip6"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["03"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["03"]["description"] = "MPLSPseudowireType"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["03"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["04"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["04"]["description"] = "MPLSBackupPseudowireID"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["04"]["datatype"] = "uint"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["05"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["05"]["description"] = "MPLSBackupPeerIpAddress"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["05"]["datatype"] = "char_ip_ip6"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["05"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["05"]["description"] = "ServiceMultiplexingValueL2TPv3Peer"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["05"]["datatype"] = "char_ip_ip6"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["description"] = "IEEE8021ahEncapsulation"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["01"]["description"] = "ITCIEncapsulation"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["02"]["description"] = "BDAEncapsulation"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["03"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["03"]["description"] = "BTCIEncapsulation"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["04"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["04"]["description"] = "ITPIDEncapsulation"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["05"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["05"]["description"] = "IPCPEncapsulation"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["05"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["06"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["06"]["description"] = "IDEIEncapsulation"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["06"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["07"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["07"]["description"] = "IUCAEncapsulation"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["07"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["08"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["08"]["description"] = "ISIDEncapsulation"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["09"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["09"]["description"] = "BTPIDEncapsulation"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["09"]["hex"] = "09"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["09"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["09"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["10"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["10"]["description"] = "BPCPEncapsulation"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["10"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["11"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["11"]["description"] = "BDEIEncapsulation"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["11"]["hex"] = "0b"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["11"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["11"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["12"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["12"]["description"] = "BVIDEncapsulation"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["12"]["hex"] = "0c"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["12"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["12"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["08"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["08"]["description"] = "ServiceMultiplexingValueIEEE8021adSTPID"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["03"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["03"]["description"] = "eSAFEDHCPSnooping"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["04"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["04"]["description"] = "CMInterfaceMaskCMIMSubtype"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["05"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["05"]["description"] = "AttachmentGroupID"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["05"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["06"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["06"]["description"] = "SourceAttachmentIndividualID"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["06"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["07"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["07"]["description"] = "TargetAttachmentIndividualID"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["07"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["08"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["08"]["description"] = "IngressUserPriority"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["08"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["09"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["09"]["description"] = "UserPriorityRange"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["09"]["hex"] = "09"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["09"]["datatype"] = "char_list"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["09"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["10"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["10"]["description"] = "L2VPNSADescriptorSubtype"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["10"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["12"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["12"]["description"] = "PseudowireType"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["12"]["hex"] = "0c"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["12"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["12"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["13"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["13"]["description"] = "L2VPNMode"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["13"]["hex"] = "0d"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["13"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["13"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["description"] = "TPIDTranslation"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["hex"] = "0e"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["01"]["description"] = "UpstreamTPIDTranslation"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["02"]["description"] = "DownstreamTPIDTranslation"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["03"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["03"]["description"] = "UpstreamSTPIDTranslation"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["04"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["04"]["description"] = "DownstreamSTPIDTranslation"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["05"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["05"]["description"] = "UpstreamBTPIDTranslation"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["05"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["06"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["06"]["description"] = "DownstreamBTPIDTranslation"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["06"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["07"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["07"]["description"] = "UpstreamITPIDTranslation"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["07"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["08"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["08"]["description"] = "DownstreamITPIDTranslation"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["description"] = "L2CPProcessing"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["hex"] = "0f"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["01"]["description"] = "L2CPTunnelMode"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["02"]["description"] = "L2CPDMACAddress"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["02"]["datatype"] = "ether"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["03"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["03"]["description"] = "L2CPOverwrotingDMACAddress"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["03"]["datatype"] = "ether"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["16"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["16"]["description"] = "DACDisableEnableConfiguration"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["16"]["hex"] = "10"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["16"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["16"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["18"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["18"]["description"] = "PseudowireClass"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["18"]["hex"] = "12"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["18"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["18"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["description"] = "ServiceDelimiter"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["hex"] = "13"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["01"]["description"] = "CVIDDelimiter"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["02"]["description"] = "SVIDDelimiter"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["03"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["03"]["description"] = "ISIDDelimiter"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["04"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["04"]["description"] = "BVIDDelimiter"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["description"] = "VirtualSwitchInstanceEncoding"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["hex"] = "14"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["01"]["description"] = "VPLSClass"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["02"]["description"] = "ETreeRole"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["03"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["03"]["description"] = "ETreeRootVID"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["04"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["04"]["description"] = "ETreeLeafVID"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["description"] = "BGPAttribute"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["hex"] = "15"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["01"]["description"] = "BGPVPNID"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["01"]["datatype"] = "uint"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["02"]["description"] = "RouteDistinguisher"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["03"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["03"]["description"] = "RouteTargetImport"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["04"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["04"]["description"] = "RouteTargetExport"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["05"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["05"]["description"] = "CEIDVEID"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["05"]["datatype"] = "ushort"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["23"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["23"]["description"] = "PseudowireSignaling"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["23"]["hex"] = "17"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["23"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["23"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["description"] = "SOAMSubtype"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["hex"] = "18"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["description"] = "MEPConfiguration"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["01"]["description"] = "MDLevel"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["02"]["description"] = "MDName"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["02"]["datatype"] = "string"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["03"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["03"]["description"] = "MAName"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["03"]["datatype"] = "string"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["04"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["04"]["description"] = "MEPID"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["04"]["datatype"] = "ushort"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["description"] = "RemoteMEPConfiguration"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["01"]["description"] = "RemoteMDLevel"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["02"]["description"] = "RemoteMDName"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["02"]["datatype"] = "string"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["03"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["03"]["description"] = "RemoteMAName"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["03"]["datatype"] = "string"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["04"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["04"]["description"] = "RemoteMEPID"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["04"]["datatype"] = "ushort"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["description"] = "FaultManagementConfiguration"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["01"]["description"] = "ContinuityCheckMessages"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["02"]["description"] = "LoopbackFunction"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["03"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["03"]["description"] = "LinktraceFunction"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["03"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["description"] = "PerformanceManagementConfiguration"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["description"] = "FrameDelayMeasurement"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["01"]["description"] = "FrameDelayMeasurementEnable"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["02"]["description"] = "FrameDelayMeasurementOneWayTwoWay"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["03"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["03"]["description"] = "FrameDelayMeasurementTransmissionPeriodicity"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["03"]["datatype"] = "ushort"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["description"] = "FrameLossMeasurement"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["01"]["description"] = "FrameLossMeasurementEnable"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["02"]["description"] = "FrameLossMeasurementTransmissionPeriodicity"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["26"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["26"]["description"] = "L2VPNDSID"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["26"]["hex"] = "1a"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["26"]["datatype"] = "uint24"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["26"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["description"] = "VendorSpecificL2VPNSubtype"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["hex"] = "2b"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"]["08"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"]["08"]["description"] = "VendorIdentifier"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["06"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["06"]["description"] = "ExtendedCMTSMICConfigurationSetting"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["06"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["01"]["description"] = "ExtendedCMTSMICHMACtype"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["02"]["description"] = "ExtendedCMTSMICBitmap"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["03"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["03"]["description"] = "ExplicitExtendedCMTSMICDigest"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["06"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["07"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["07"]["description"] = "SAVAuthorizationEncoding"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["07"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["01"]["description"] = "SAVGroupName"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["01"]["datatype"] = "string"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["description"] = "SAVStaticPrefixRule"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["01"]["description"] = "SAVStaticPrefixAddress"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["01"]["datatype"] = "ip_ip6"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["02"]["description"] = "SAVStaticPrefixLength"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["08"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["08"]["description"] = "VendorIdentifier"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["09"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["09"]["description"] = "CMAttributeMasks"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["09"]["hex"] = "09"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["09"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["01"]["description"] = "CMDownstreamRequiredAttributeMask"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["02"]["description"] = "CMDownstreamForbiddenAttributeMask"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["03"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["03"]["description"] = "CMUpstreamRequiredAttributeMask"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["04"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["04"]["description"] = "CMUpstreamForbiddenAttributeMask"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["09"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["description"] = "IPMulticastJoinAuthorization"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["01"]["description"] = "IPMulticastProfileName"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["01"]["datatype"] = "string"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["description"] = "IPMulticastJoinAuthStaticSessionRule"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["01"]["description"] = "MulticastRulePriority"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["02"]["description"] = "AuthorizationAction"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["03"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["03"]["description"] = "SourcePrefixAddress"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["03"]["datatype"] = "ip_ip6"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["04"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["04"]["description"] = "SourcePrefixLength"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["04"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["05"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["05"]["description"] = "GroupPrefixAddress"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["05"]["datatype"] = "ip_ip6"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["06"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["06"]["description"] = "GroupPrefixLength"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["06"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["03"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["03"]["description"] = "MaximumMulticastSessions"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["03"]["datatype"] = "ushort"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["10"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["11"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["11"]["description"] = "ServiceTypeIdentifier"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["11"]["hex"] = "0b"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["11"]["datatype"] = "string"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["11"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["12"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["12"]["description"] = "DEMARCAutoConfiguration"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["12"]["hex"] = "0c"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["12"]["datatype"] = "aggregate"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["01"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["01"]["description"] = "DACDisableEnableConfig"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["02"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["02"]["description"] = "CMIMEncoding"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["03"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["03"]["description"] = "UpstreamServiceClassName"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["03"]["datatype"] = "strzero"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["04"] = {}
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["04"]["description"] = "DownstreamServiceClassName"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["04"]["datatype"] = "strzero"
DocsisTlvs["23"]["subTlvs"]["43"]["subTlvs"]["12"]["subTlvs"]["04"]["subTlvs"] = {}

 */
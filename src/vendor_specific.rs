use std::collections::HashMap;
use crate::tlv::TLV;

use crate::d4::DATATYPE;
use crate::d4::DOCSIS4TLV;
pub(crate) fn docsis_vendor_specific() -> HashMap<u8, DOCSIS4TLV> {
    let mut d4_defs: HashMap<u8, DOCSIS4TLV> = HashMap::new();
    let sub_tlv = DOCSIS4TLV {
        t: 0x01,
        description: "CMLoadBalancingPolicyID".to_string(),
        data_type: DATATYPE::UINT,
        tlv: TLV { t: 0x01, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);

    let sub_tlv = DOCSIS4TLV {
        t: 0x08,
        description: "VendorIdentifier".to_string(),
        data_type: DATATYPE::HEXSTR,
        tlv: TLV { t: 0x08, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);

    let mut sub_tlv = DOCSIS4TLV {
        t: 0x09,
        description: "CMAttributeMasks".to_string(),
        data_type: DATATYPE::AGGREGATE,
        tlv: TLV { t: 0x09, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    let sub_sub_tlv = DOCSIS4TLV {
        t: 0x04,
        description: "CMUpstreamForbiddenAttributeMask".to_string(),
        data_type: DATATYPE::HEXSTR,
        tlv: TLV { t: 0x04, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_tlv.sub_tlvs.insert(sub_sub_tlv.t, sub_sub_tlv);
    d4_defs.insert(sub_tlv.t, sub_tlv);

    let sub_tlv = DOCSIS4TLV {
        t: 0xa1,
        description: "vendor specific unknown".to_string(),
        data_type: DATATYPE::HEXSTR,
        tlv: TLV { t: 0xa1, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);

    d4_defs
}

/*


DocsisTlvs["43"]["subTlvs"]["02"] = {}
DocsisTlvs["43"]["subTlvs"]["02"]["description"] = "CMLoadBalancingPriority"
DocsisTlvs["43"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["43"]["subTlvs"]["02"]["datatype"] = "uint"
DocsisTlvs["43"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["03"] = {}
DocsisTlvs["43"]["subTlvs"]["03"]["description"] = "CMLoadBalancingGroupID"
DocsisTlvs["43"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["43"]["subTlvs"]["03"]["datatype"] = "uint"
DocsisTlvs["43"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["04"] = {}
DocsisTlvs["43"]["subTlvs"]["04"]["description"] = "CMRangingClassIDExtension"
DocsisTlvs["43"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["43"]["subTlvs"]["04"]["datatype"] = "ushort"
DocsisTlvs["43"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["description"] = "L2VPNEncoding"
DocsisTlvs["43"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["43"]["subTlvs"]["05"]["datatype"] = "aggregate"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["01"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["01"]["description"] = "VPNIdentifier"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["description"] = "NSIEncapsulation"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["datatype"] = "aggregate"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["01"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["01"]["description"] = "ServiceMultiplexingValueOther"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["01"]["datatype"] = "lenzero"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["02"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["02"]["description"] = "NSIEncapsulationSingleQTag"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["02"]["datatype"] = "ushort"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["03"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["03"]["description"] = "NSIEncapsulationDualQTag"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["03"]["datatype"] = "dual_qtag"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["description"] = "ServiceMultiplexingValueMPLSPW"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["datatype"] = "aggregate"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["01"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["01"]["description"] = "MPLSPseudowireID"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["01"]["datatype"] = "uint"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["02"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["02"]["description"] = "MPLSPeerIpAddress"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["02"]["datatype"] = "char_ip_ip6"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["03"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["03"]["description"] = "MPLSPseudowireType"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["03"]["datatype"] = "uchar"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["04"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["04"]["description"] = "MPLSBackupPseudowireID"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["04"]["datatype"] = "uint"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["05"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["05"]["description"] = "MPLSBackupPeerIpAddress"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["05"]["datatype"] = "char_ip_ip6"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["05"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["05"]["description"] = "ServiceMultiplexingValueL2TPv3Peer"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["05"]["datatype"] = "char_ip_ip6"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["description"] = "IEEE8021ahEncapsulation"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["datatype"] = "aggregate"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["01"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["01"]["description"] = "ITCIEncapsulation"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["02"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["02"]["description"] = "BDAEncapsulation"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["03"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["03"]["description"] = "BTCIEncapsulation"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["04"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["04"]["description"] = "ITPIDEncapsulation"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["05"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["05"]["description"] = "IPCPEncapsulation"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["05"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["06"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["06"]["description"] = "IDEIEncapsulation"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["06"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["07"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["07"]["description"] = "IUCAEncapsulation"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["07"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["08"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["08"]["description"] = "ISIDEncapsulation"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["09"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["09"]["description"] = "BTPIDEncapsulation"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["09"]["hex"] = "09"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["09"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["09"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["10"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["10"]["description"] = "BPCPEncapsulation"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["10"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["11"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["11"]["description"] = "BDEIEncapsulation"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["11"]["hex"] = "0b"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["11"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["11"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["12"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["12"]["description"] = "BVIDEncapsulation"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["12"]["hex"] = "0c"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["12"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"]["12"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["08"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["08"]["description"] = "ServiceMultiplexingValueIEEE8021adSTPID"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["02"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["03"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["03"]["description"] = "eSAFEDHCPSnooping"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["04"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["04"]["description"] = "CMInterfaceMaskCMIMSubtype"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["05"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["05"]["description"] = "AttachmentGroupID"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["05"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["06"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["06"]["description"] = "SourceAttachmentIndividualID"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["06"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["07"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["07"]["description"] = "TargetAttachmentIndividualID"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["07"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["08"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["08"]["description"] = "IngressUserPriority"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["08"]["datatype"] = "uchar"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["09"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["09"]["description"] = "UserPriorityRange"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["09"]["hex"] = "09"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["09"]["datatype"] = "char_list"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["09"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["10"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["10"]["description"] = "L2VPNSADescriptorSubtype"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["10"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["12"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["12"]["description"] = "PseudowireType"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["12"]["hex"] = "0c"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["12"]["datatype"] = "uchar"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["12"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["13"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["13"]["description"] = "L2VPNMode"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["13"]["hex"] = "0d"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["13"]["datatype"] = "uchar"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["13"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["description"] = "TPIDTranslation"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["hex"] = "0e"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["datatype"] = "aggregate"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["01"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["01"]["description"] = "UpstreamTPIDTranslation"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["02"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["02"]["description"] = "DownstreamTPIDTranslation"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["03"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["03"]["description"] = "UpstreamSTPIDTranslation"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["04"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["04"]["description"] = "DownstreamSTPIDTranslation"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["05"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["05"]["description"] = "UpstreamBTPIDTranslation"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["05"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["06"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["06"]["description"] = "DownstreamBTPIDTranslation"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["06"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["07"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["07"]["description"] = "UpstreamITPIDTranslation"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["07"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["08"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["08"]["description"] = "DownstreamITPIDTranslation"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["14"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["15"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["15"]["description"] = "L2CPProcessing"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["15"]["hex"] = "0f"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["15"]["datatype"] = "aggregate"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["01"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["01"]["description"] = "L2CPTunnelMode"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["02"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["02"]["description"] = "L2CPDMACAddress"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["02"]["datatype"] = "ether"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["03"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["03"]["description"] = "L2CPOverwrotingDMACAddress"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["03"]["datatype"] = "ether"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["15"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["16"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["16"]["description"] = "DACDisableEnableConfiguration"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["16"]["hex"] = "10"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["16"]["datatype"] = "uchar"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["16"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["18"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["18"]["description"] = "PseudowireClass"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["18"]["hex"] = "12"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["18"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["18"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["19"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["19"]["description"] = "ServiceDelimiter"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["19"]["hex"] = "13"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["19"]["datatype"] = "aggregate"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["01"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["01"]["description"] = "CVIDDelimiter"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["02"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["02"]["description"] = "SVIDDelimiter"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["03"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["03"]["description"] = "ISIDDelimiter"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["04"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["04"]["description"] = "BVIDDelimiter"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["19"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["20"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["20"]["description"] = "VirtualSwitchInstanceEncoding"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["20"]["hex"] = "14"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["20"]["datatype"] = "aggregate"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["01"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["01"]["description"] = "VPLSClass"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["02"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["02"]["description"] = "ETreeRole"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["03"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["03"]["description"] = "ETreeRootVID"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["04"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["04"]["description"] = "ETreeLeafVID"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["20"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["description"] = "BGPAttribute"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["hex"] = "15"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["datatype"] = "aggregate"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["01"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["01"]["description"] = "BGPVPNID"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["01"]["datatype"] = "uint"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["02"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["02"]["description"] = "RouteDistinguisher"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["03"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["03"]["description"] = "RouteTargetImport"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["04"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["04"]["description"] = "RouteTargetExport"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["04"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["05"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["05"]["description"] = "CEIDVEID"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["05"]["datatype"] = "ushort"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["21"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["23"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["23"]["description"] = "PseudowireSignaling"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["23"]["hex"] = "17"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["23"]["datatype"] = "uchar"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["23"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["description"] = "SOAMSubtype"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["hex"] = "18"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["datatype"] = "aggregate"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["description"] = "MEPConfiguration"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["datatype"] = "aggregate"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["01"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["01"]["description"] = "MDLevel"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["02"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["02"]["description"] = "MDName"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["02"]["datatype"] = "string"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["03"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["03"]["description"] = "MAName"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["03"]["datatype"] = "string"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["04"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["04"]["description"] = "MEPID"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["04"]["datatype"] = "ushort"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["01"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["description"] = "RemoteMEPConfiguration"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["datatype"] = "aggregate"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["01"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["01"]["description"] = "RemoteMDLevel"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["02"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["02"]["description"] = "RemoteMDName"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["02"]["datatype"] = "string"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["03"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["03"]["description"] = "RemoteMAName"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["03"]["datatype"] = "string"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["04"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["04"]["description"] = "RemoteMEPID"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["04"]["datatype"] = "ushort"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["description"] = "FaultManagementConfiguration"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["datatype"] = "aggregate"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["01"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["01"]["description"] = "ContinuityCheckMessages"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["02"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["02"]["description"] = "LoopbackFunction"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["03"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["03"]["description"] = "LinktraceFunction"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["03"]["datatype"] = "uchar"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["03"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["description"] = "PerformanceManagementConfiguration"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["datatype"] = "aggregate"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["description"] = "FrameDelayMeasurement"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["datatype"] = "aggregate"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["01"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["01"]["description"] = "FrameDelayMeasurementEnable"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["02"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["02"]["description"] = "FrameDelayMeasurementOneWayTwoWay"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["03"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["03"]["description"] = "FrameDelayMeasurementTransmissionPeriodicity"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["03"]["datatype"] = "ushort"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["01"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["description"] = "FrameLossMeasurement"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["datatype"] = "aggregate"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["01"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["01"]["description"] = "FrameLossMeasurementEnable"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["02"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["02"]["description"] = "FrameLossMeasurementTransmissionPeriodicity"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["24"]["subTlvs"]["04"]["subTlvs"]["02"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["26"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["26"]["description"] = "L2VPNDSID"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["26"]["hex"] = "1a"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["26"]["datatype"] = "uint24"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["26"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["43"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["43"]["description"] = "VendorSpecificL2VPNSubtype"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["43"]["hex"] = "2b"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["43"]["datatype"] = "aggregate"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"]["08"] = {}
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"]["08"]["description"] = "VendorIdentifier"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["05"]["subTlvs"]["43"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["06"] = {}
DocsisTlvs["43"]["subTlvs"]["06"]["description"] = "ExtendedCMTSMICConfiguration"
DocsisTlvs["43"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["43"]["subTlvs"]["06"]["datatype"] = "aggregate"
DocsisTlvs["43"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["06"]["subTlvs"]["01"] = {}
DocsisTlvs["43"]["subTlvs"]["06"]["subTlvs"]["01"]["description"] = "ExtendedCMTSMICHMACDigestType"
DocsisTlvs["43"]["subTlvs"]["06"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["43"]["subTlvs"]["06"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["43"]["subTlvs"]["06"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["06"]["subTlvs"]["02"] = {}
DocsisTlvs["43"]["subTlvs"]["06"]["subTlvs"]["02"]["description"] = "ExtendedCMTSMICHMACBitmap"
DocsisTlvs["43"]["subTlvs"]["06"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["43"]["subTlvs"]["06"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["06"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["06"]["subTlvs"]["03"] = {}
DocsisTlvs["43"]["subTlvs"]["06"]["subTlvs"]["03"]["description"] = "ExtendedCMTSMICHMACDigest"
DocsisTlvs["43"]["subTlvs"]["06"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["43"]["subTlvs"]["06"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["06"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["07"] = {}
DocsisTlvs["43"]["subTlvs"]["07"]["description"] = "SAVAuthorizationEncoding"
DocsisTlvs["43"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["43"]["subTlvs"]["07"]["datatype"] = "aggregate"
DocsisTlvs["43"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["07"]["subTlvs"]["01"] = {}
DocsisTlvs["43"]["subTlvs"]["07"]["subTlvs"]["01"]["description"] = "SAVGroupName"
DocsisTlvs["43"]["subTlvs"]["07"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["43"]["subTlvs"]["07"]["subTlvs"]["01"]["datatype"] = "string"
DocsisTlvs["43"]["subTlvs"]["07"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["07"]["subTlvs"]["02"] = {}
DocsisTlvs["43"]["subTlvs"]["07"]["subTlvs"]["02"]["description"] = "SAVStaticPrefixRule"
DocsisTlvs["43"]["subTlvs"]["07"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["43"]["subTlvs"]["07"]["subTlvs"]["02"]["datatype"] = "aggregate"
DocsisTlvs["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["01"] = {}
DocsisTlvs["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["01"]["description"] = "SAVStaticPrefixAddress"
DocsisTlvs["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["01"]["datatype"] = "ip_ip6"
DocsisTlvs["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["02"] = {}
DocsisTlvs["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["02"]["description"] = "SAVStaticPrefixLength"
DocsisTlvs["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["43"]["subTlvs"]["07"]["subTlvs"]["02"]["subTlvs"]["02"]["subTlvs"] = {}



DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["01"] = {}
DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["01"]["description"] = "CMDownstreamRequiredAttributeMask"
DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["01"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["02"] = {}
DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["02"]["description"] = "CMDownstreamForbiddenAttributeMask"
DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["03"] = {}
DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["03"]["description"] = "CMUpstreamRequiredAttributeMask"
DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["03"]["subTlvs"] = {}


DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["05"] = {}
DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["05"]["description"] = "Unknown"
DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["05"]["datatype"] = "unknown"
DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["06"] = {}
DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["06"]["description"] = "Unknown"
DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["06"]["datatype"] = "unknown"
DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["07"] = {}
DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["07"]["description"] = "Unknown"
DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["07"]["hex"] = "07"
DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["07"]["datatype"] = "unknown"
DocsisTlvs["43"]["subTlvs"]["09"]["subTlvs"]["07"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["10"] = {}
DocsisTlvs["43"]["subTlvs"]["10"]["description"] = "IPMulticastJoinAuthorization"
DocsisTlvs["43"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["43"]["subTlvs"]["10"]["datatype"] = "aggregate"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["01"] = {}
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["01"]["description"] = "IPMulticastProfileName"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["01"]["datatype"] = "string"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"] = {}
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["description"] = "IPMulticastJoinAuthStaticSessionRule"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["datatype"] = "aggregate"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["01"] = {}
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["01"]["description"] = "MulticastRulePriority"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["02"] = {}
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["02"]["description"] = "AuthorizationAction"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["02"]["datatype"] = "uchar"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["03"] = {}
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["03"]["description"] = "SourcePrefixAddress"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["03"]["datatype"] = "ip_ip6"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["04"] = {}
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["04"]["description"] = "SourcePrefixLength"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["04"]["datatype"] = "uchar"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["05"] = {}
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["05"]["description"] = "GroupPrefixAddress"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["05"]["datatype"] = "ip_ip6"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["06"] = {}
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["06"]["description"] = "GroupPrefixLength"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["06"]["datatype"] = "uchar"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["03"] = {}
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["03"]["description"] = "MaximumMulticastSessions"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["03"]["datatype"] = "ushort"
DocsisTlvs["43"]["subTlvs"]["10"]["subTlvs"]["03"]["subTlvs"] = {}



DocsisTlvs["43"]["subTlvs"]["11"] = {}
DocsisTlvs["43"]["subTlvs"]["11"]["description"] = "ServiceTypeIdentifier"
DocsisTlvs["43"]["subTlvs"]["11"]["hex"] = "0b"
DocsisTlvs["43"]["subTlvs"]["11"]["datatype"] = "string"
DocsisTlvs["43"]["subTlvs"]["11"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["12"] = {}
DocsisTlvs["43"]["subTlvs"]["12"]["description"] = "DEMARCAutoConfiguration"
DocsisTlvs["43"]["subTlvs"]["12"]["hex"] = "0c"
DocsisTlvs["43"]["subTlvs"]["12"]["datatype"] = "aggregate"
DocsisTlvs["43"]["subTlvs"]["12"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["12"]["subTlvs"]["01"] = {}
DocsisTlvs["43"]["subTlvs"]["12"]["subTlvs"]["01"]["description"] = "DACDisableEnableConfig"
DocsisTlvs["43"]["subTlvs"]["12"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["43"]["subTlvs"]["12"]["subTlvs"]["01"]["datatype"] = "uchar"
DocsisTlvs["43"]["subTlvs"]["12"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["12"]["subTlvs"]["02"] = {}
DocsisTlvs["43"]["subTlvs"]["12"]["subTlvs"]["02"]["description"] = "CMIMEncoding"
DocsisTlvs["43"]["subTlvs"]["12"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["43"]["subTlvs"]["12"]["subTlvs"]["02"]["datatype"] = "hexstr"
DocsisTlvs["43"]["subTlvs"]["12"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["12"]["subTlvs"]["03"] = {}
DocsisTlvs["43"]["subTlvs"]["12"]["subTlvs"]["03"]["description"] = "UpstreamServiceClassName"
DocsisTlvs["43"]["subTlvs"]["12"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["43"]["subTlvs"]["12"]["subTlvs"]["03"]["datatype"] = "strzero"
DocsisTlvs["43"]["subTlvs"]["12"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["12"]["subTlvs"]["04"] = {}
DocsisTlvs["43"]["subTlvs"]["12"]["subTlvs"]["04"]["description"] = "DownstreamServiceClassName"
DocsisTlvs["43"]["subTlvs"]["12"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["43"]["subTlvs"]["12"]["subTlvs"]["04"]["datatype"] = "strzero"
DocsisTlvs["43"]["subTlvs"]["12"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["65"] = {}
DocsisTlvs["43"]["subTlvs"]["65"]["description"] = "dunno"
DocsisTlvs["43"]["subTlvs"]["65"]["hex"] = "41"
DocsisTlvs["43"]["subTlvs"]["65"]["datatype"] = "unknown"
DocsisTlvs["43"]["subTlvs"]["65"]["subTlvs"] = {}

DocsisTlvs["43"]["subTlvs"]["161"] = {}
DocsisTlvs["43"]["subTlvs"]["161"]["description"] = "vendor specif"
DocsisTlvs["43"]["subTlvs"]["161"]["hex"] = "a1"
DocsisTlvs["43"]["subTlvs"]["161"]["datatype"] = "unknown"
DocsisTlvs["43"]["subTlvs"]["161"]["subTlvs"] = {}

 */
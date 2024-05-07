use std::collections::HashMap;
use crate::tlv::TLV;

use crate::d4::DATATYPE;
use crate::d4::DOCSIS4TLV;
pub(crate) fn erouter() -> HashMap<u8, DOCSIS4TLV> {
    let mut d4_defs: HashMap<u8, DOCSIS4TLV> = HashMap::new();
    let sub_tlv = DOCSIS4TLV {
        t: 0x01,
        description: "InitializationMode".to_string(),
        data_type: DATATYPE::UCHAR,
        tlv: TLV { t: 0x01, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let mut sub_tlv = DOCSIS4TLV {
        t: 0x02,
        description: "TR69ManagementServer".to_string(),
        data_type: DATATYPE::AGGREGATE,
        tlv: TLV { t: 0x02, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    let sub_sub_tlv = DOCSIS4TLV {
        t: 0x01,
        description: "EnableCWMP".to_string(),
        data_type: DATATYPE::UCHAR,
        tlv: TLV { t: 0x01, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_tlv.sub_tlvs.insert(sub_sub_tlv.t, sub_sub_tlv);
    let sub_sub_tlv = DOCSIS4TLV {
        t: 0x02,
        description: "URL".to_string(),
        data_type: DATATYPE::STRING,
        tlv: TLV { t: 0x02, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_tlv.sub_tlvs.insert(sub_sub_tlv.t, sub_sub_tlv);

    let sub_sub_tlv = DOCSIS4TLV {
        t: 0x07,
        description: "ACSOverride".to_string(),
        data_type: DATATYPE::UCHAR,
        tlv: TLV { t: 0x07, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_tlv.sub_tlvs.insert(sub_sub_tlv.t, sub_sub_tlv);
    d4_defs.insert(sub_tlv.t, sub_tlv);
    d4_defs
}

/*



DocsisTlvs["202"]["subTlvs"]["02"]["subTlvs"]["03"] = {}
DocsisTlvs["202"]["subTlvs"]["02"]["subTlvs"]["03"]["description"] = "Username"
DocsisTlvs["202"]["subTlvs"]["02"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["202"]["subTlvs"]["02"]["subTlvs"]["03"]["datatype"] = "string"
DocsisTlvs["202"]["subTlvs"]["02"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["202"]["subTlvs"]["02"]["subTlvs"]["04"] = {}
DocsisTlvs["202"]["subTlvs"]["02"]["subTlvs"]["04"]["description"] = "Password"
DocsisTlvs["202"]["subTlvs"]["02"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["202"]["subTlvs"]["02"]["subTlvs"]["04"]["datatype"] = "string"
DocsisTlvs["202"]["subTlvs"]["02"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["202"]["subTlvs"]["02"]["subTlvs"]["05"] = {}
DocsisTlvs["202"]["subTlvs"]["02"]["subTlvs"]["05"]["description"] = "ConnectionRequestUsername"
DocsisTlvs["202"]["subTlvs"]["02"]["subTlvs"]["05"]["hex"] = "05"
DocsisTlvs["202"]["subTlvs"]["02"]["subTlvs"]["05"]["datatype"] = "string"
DocsisTlvs["202"]["subTlvs"]["02"]["subTlvs"]["05"]["subTlvs"] = {}

DocsisTlvs["202"]["subTlvs"]["02"]["subTlvs"]["06"] = {}
DocsisTlvs["202"]["subTlvs"]["02"]["subTlvs"]["06"]["description"] = "ConnectionRequestPassword"
DocsisTlvs["202"]["subTlvs"]["02"]["subTlvs"]["06"]["hex"] = "06"
DocsisTlvs["202"]["subTlvs"]["02"]["subTlvs"]["06"]["datatype"] = "string"
DocsisTlvs["202"]["subTlvs"]["02"]["subTlvs"]["06"]["subTlvs"] = {}

DocsisTlvs["202"]["subTlvs"]["03"] = {}
DocsisTlvs["202"]["subTlvs"]["03"]["description"] = "InitializationModeOverride"
DocsisTlvs["202"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["202"]["subTlvs"]["03"]["datatype"] = "uchar"
DocsisTlvs["202"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["202"]["subTlvs"]["10"] = {}
DocsisTlvs["202"]["subTlvs"]["10"]["description"] = "RATransmissionInterval"
DocsisTlvs["202"]["subTlvs"]["10"]["hex"] = "0a"
DocsisTlvs["202"]["subTlvs"]["10"]["datatype"] = "ushort"
DocsisTlvs["202"]["subTlvs"]["10"]["subTlvs"] = {}

DocsisTlvs["202"]["subTlvs"]["11"] = {}
DocsisTlvs["202"]["subTlvs"]["11"]["description"] = "SnmpMibObject"
DocsisTlvs["202"]["subTlvs"]["11"]["hex"] = "0b"
DocsisTlvs["202"]["subTlvs"]["11"]["datatype"] = "snmp_object"
DocsisTlvs["202"]["subTlvs"]["11"]["subTlvs"] = {}

DocsisTlvs["202"]["subTlvs"]["42"] = {}
DocsisTlvs["202"]["subTlvs"]["42"]["description"] = "TopologyModeEncoding"
DocsisTlvs["202"]["subTlvs"]["42"]["hex"] = "2a"
DocsisTlvs["202"]["subTlvs"]["42"]["datatype"] = "uchar"
DocsisTlvs["202"]["subTlvs"]["42"]["subTlvs"] = {}

DocsisTlvs["202"]["subTlvs"]["43"] = {}
DocsisTlvs["202"]["subTlvs"]["43"]["description"] = "VendorSpecific"
DocsisTlvs["202"]["subTlvs"]["43"]["hex"] = "2b"
DocsisTlvs["202"]["subTlvs"]["43"]["datatype"] = "aggregate"
DocsisTlvs["202"]["subTlvs"]["43"]["subTlvs"] = {}

DocsisTlvs["202"]["subTlvs"]["43"]["subTlvs"]["08"] = {}
DocsisTlvs["202"]["subTlvs"]["43"]["subTlvs"]["08"]["description"] = "VendorIdentifier"
DocsisTlvs["202"]["subTlvs"]["43"]["subTlvs"]["08"]["hex"] = "08"
DocsisTlvs["202"]["subTlvs"]["43"]["subTlvs"]["08"]["datatype"] = "hexstr"
DocsisTlvs["202"]["subTlvs"]["43"]["subTlvs"]["08"]["subTlvs"] = {}

DocsisTlvs["202"]["subTlvs"]["53"] = {}
DocsisTlvs["202"]["subTlvs"]["53"]["description"] = "SNMPv1v2cCoexistenceConfig"
DocsisTlvs["202"]["subTlvs"]["53"]["hex"] = "35"
DocsisTlvs["202"]["subTlvs"]["53"]["datatype"] = "aggregate"
DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"] = {}

DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["01"] = {}
DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["01"]["description"] = "SNMPv1v2cCommunityName"
DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["01"]["datatype"] = "string"
DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["02"] = {}
DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["02"]["description"] = "SNMPv1v2cTransportAddressAccess"
DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["02"]["datatype"] = "aggregate"
DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["02"]["subTlvs"]["01"] = {}
DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["02"]["subTlvs"]["01"]["description"] = "SNMPv1v2cTransportAddress"
DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["02"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["02"]["subTlvs"]["01"]["datatype"] = "(encode_ip_ip6_port)"
DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["02"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["02"]["subTlvs"]["02"] = {}
DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["02"]["subTlvs"]["02"]["description"] = "SNMPv1v2cTransportAddressMask"
DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["02"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["02"]["subTlvs"]["02"]["datatype"] = "(encode_ip_ip6_port)"
DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["02"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["03"] = {}
DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["03"]["description"] = "SNMPv1v2cAccessViewType"
DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["03"]["datatype"] = "uchar"
DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["04"] = {}
DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["04"]["description"] = "SNMPv1v2cAccessViewName"
DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["04"]["datatype"] = "string"
DocsisTlvs["202"]["subTlvs"]["53"]["subTlvs"]["04"]["subTlvs"] = {}

DocsisTlvs["202"]["subTlvs"]["54"] = {}
DocsisTlvs["202"]["subTlvs"]["54"]["description"] = "SNMPv3AccessViewConfiguration"
DocsisTlvs["202"]["subTlvs"]["54"]["hex"] = "36"
DocsisTlvs["202"]["subTlvs"]["54"]["datatype"] = "aggregate"
DocsisTlvs["202"]["subTlvs"]["54"]["subTlvs"] = {}

DocsisTlvs["202"]["subTlvs"]["54"]["subTlvs"]["01"] = {}
DocsisTlvs["202"]["subTlvs"]["54"]["subTlvs"]["01"]["description"] = "SNMPv3AccessViewName"
DocsisTlvs["202"]["subTlvs"]["54"]["subTlvs"]["01"]["hex"] = "01"
DocsisTlvs["202"]["subTlvs"]["54"]["subTlvs"]["01"]["datatype"] = "string"
DocsisTlvs["202"]["subTlvs"]["54"]["subTlvs"]["01"]["subTlvs"] = {}

DocsisTlvs["202"]["subTlvs"]["54"]["subTlvs"]["02"] = {}
DocsisTlvs["202"]["subTlvs"]["54"]["subTlvs"]["02"]["description"] = "SNMPv3AccessViewSubtree"
DocsisTlvs["202"]["subTlvs"]["54"]["subTlvs"]["02"]["hex"] = "02"
DocsisTlvs["202"]["subTlvs"]["54"]["subTlvs"]["02"]["datatype"] = "(encode_oid)"
DocsisTlvs["202"]["subTlvs"]["54"]["subTlvs"]["02"]["subTlvs"] = {}

DocsisTlvs["202"]["subTlvs"]["54"]["subTlvs"]["03"] = {}
DocsisTlvs["202"]["subTlvs"]["54"]["subTlvs"]["03"]["description"] = "SNMPv3AccessViewMask"
DocsisTlvs["202"]["subTlvs"]["54"]["subTlvs"]["03"]["hex"] = "03"
DocsisTlvs["202"]["subTlvs"]["54"]["subTlvs"]["03"]["datatype"] = "hexstr"
DocsisTlvs["202"]["subTlvs"]["54"]["subTlvs"]["03"]["subTlvs"] = {}

DocsisTlvs["202"]["subTlvs"]["54"]["subTlvs"]["04"] = {}
DocsisTlvs["202"]["subTlvs"]["54"]["subTlvs"]["04"]["description"] = "SNMPv3AccessViewType"
DocsisTlvs["202"]["subTlvs"]["54"]["subTlvs"]["04"]["hex"] = "04"
DocsisTlvs["202"]["subTlvs"]["54"]["subTlvs"]["04"]["datatype"] = "uchar"
DocsisTlvs["202"]["subTlvs"]["54"]["subTlvs"]["04"]["subTlvs"] = {}

 */
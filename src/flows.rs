use std::collections::HashMap;
use crate::tlv::TLV;

use crate::d4::DATATYPE;
use crate::d4::DOCSIS4TLV;
pub(crate) fn upstreams() -> HashMap<u8, DOCSIS4TLV> {
    let mut d4_defs: HashMap<u8, DOCSIS4TLV> = HashMap::new();
    let  sub_tlv = DOCSIS4TLV {
        t: 0x01,
        description: "ServiceFlowRef".to_string(),
        data_type: DATATYPE::USHORT,
        tlv: TLV { t: 0x01, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let  sub_tlv = DOCSIS4TLV {
        t: 0x02,
        description: "ServiceFlowId".to_string(),
        data_type: DATATYPE::UINT,
        tlv: TLV { t: 0x02, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let  sub_tlv = DOCSIS4TLV {
        t: 0x03,
        description: "ServiceIdentifier".to_string(),
        data_type: DATATYPE::USHORT,
        tlv: TLV { t: 0x03, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let  sub_tlv = DOCSIS4TLV {
        t: 0x04,
        description: "ServiceClassName".to_string(),
        data_type: DATATYPE::STRINGZERO,
        tlv: TLV { t: 0x04, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let  sub_tlv = DOCSIS4TLV {
        t: 0x06,
        description: "QosParamSetType".to_string(),
        data_type: DATATYPE::UCHAR,
        tlv: TLV { t: 0x06, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
        sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let  sub_tlv = DOCSIS4TLV {
            t: 0x07,
            description: "TrafficPriority".to_string(),
            data_type: DATATYPE::UCHAR,
            tlv: TLV { t: 0x07, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
            t: 0x08,
            description: "MaxRateSustained".to_string(),
            data_type: DATATYPE::UINT,
            tlv: TLV { t: 0x08, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
            t: 0x09,
            description: "MaxTrafficBurst".to_string(),
            data_type: DATATYPE::UINT,
            tlv: TLV { t: 0x09, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
            t: 0x0a,
            description: "MinReservedRate".to_string(),
            data_type: DATATYPE::UINT,
            tlv: TLV { t: 0x0a, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
            t: 0x0b,
            description: "MinResPacketSize".to_string(),
            data_type: DATATYPE::USHORT,
            tlv: TLV { t: 0x0b, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
            t: 0x0c,
            description: "ActQosParamsTimeout".to_string(),
            data_type: DATATYPE::USHORT,
            tlv: TLV { t: 0x0c, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
            t: 0x0d,
            description: "AdmQosParamsTimeout".to_string(),
            data_type: DATATYPE::USHORT,
            tlv: TLV { t: 0x0d, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
            t: 0x0e,
            description: "MaxConcatenatedBurst".to_string(),
            data_type: DATATYPE::USHORT,
            tlv: TLV { t: 0x0e, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let  sub_tlv = DOCSIS4TLV {
            t: 0x0f,
            description: "SchedulingType".to_string(),
            data_type: DATATYPE::UCHAR,
            tlv: TLV { t: 0x0f, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let  sub_tlv = DOCSIS4TLV {
            t: 0x10,
            description: "RequestOrTxPolicy".to_string(),
            data_type: DATATYPE::HEXSTR,
            tlv: TLV { t: 0x10, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
            t: 0x11,
            description: "NominalPollInterval".to_string(),
            data_type: DATATYPE::UINT,
            tlv: TLV { t: 0x11, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
            t: 0x12,
            description: "ToleratedPollJitter".to_string(),
            data_type: DATATYPE::UINT,
            tlv: TLV { t: 0x12, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
            t: 0x13,
            description: "UnsolicitedGrantSize".to_string(),
            data_type: DATATYPE::USHORT,
            tlv: TLV { t: 0x13, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
            t: 0x14,
            description: "NominalGrantInterval".to_string(),
            data_type: DATATYPE::UINT,
            tlv: TLV { t: 0x14, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
            t: 0x15,
            description: "ToleratedGrantJitter".to_string(),
            data_type: DATATYPE::UINT,
            tlv: TLV { t: 0x15, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
            t: 0x16,
            description: "GrantsPerInterval".to_string(),
            data_type: DATATYPE::UCHAR,
            tlv: TLV { t: 0x16, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let  sub_tlv = DOCSIS4TLV {
            t: 0x17,
            description: "IpTosOverwrite".to_string(),
            data_type: DATATYPE::HEXSTR,
            tlv: TLV { t: 0x17, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
            t: 0x1a,
            description: "MultipliertoNumberofBytesRequested".to_string(),
            data_type: DATATYPE::UCHAR,
            tlv: TLV { t: 0x1a, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
            t: 0x1b,
            description: "StreamPeakTrafficRate".to_string(),
            data_type: DATATYPE::UINT,
            tlv: TLV { t: 0x1b, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
            t: 0x1f,
            description: "ServiceFlowRequiredAttributeMask".to_string(),
            data_type: DATATYPE::HEXSTR,
            tlv: TLV { t: 0x1f, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
            t: 0x20,
            description: "ServiceFlowForbiddenAttributeMask".to_string(),
            data_type: DATATYPE::HEXSTR,
            tlv: TLV { t: 0x20, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
            t: 0x21,
            description: "ServiceFlowAttributeAggregationRuleMask".to_string(),
            data_type: DATATYPE::HEXSTR,
            tlv: TLV { t: 0x21, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
            t: 0x22,
            description: "ApplicationIdentifier".to_string(),
            data_type: DATATYPE::UINT,
            tlv: TLV { t: 0x22, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let mut sub_tlv = DOCSIS4TLV {
            t: 0x23,
            description: "BufferControl".to_string(),
            data_type: DATATYPE::AGGREGATE,
            tlv: TLV { t: 0x23, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    let sub_sub_tlv = DOCSIS4TLV {
            t: 0x01,
            description: "MinimumBuffer".to_string(),
            data_type: DATATYPE::UINT,
            tlv: TLV { t: 0x01, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_tlv.sub_tlvs.insert(sub_sub_tlv.t, sub_sub_tlv);
    let sub_sub_tlv = DOCSIS4TLV {
            t: 0x02,
            description: "TargetBuffer".to_string(),
            data_type: DATATYPE::UINT,
            tlv: TLV { t: 0x02, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_tlv.sub_tlvs.insert(sub_sub_tlv.t, sub_sub_tlv);
    let sub_sub_tlv = DOCSIS4TLV {
            t: 0x03,
            description: "MaximumBuffer".to_string(),
            data_type: DATATYPE::UINT,
            tlv: TLV { t: 0x03, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_tlv.sub_tlvs.insert(sub_sub_tlv.t, sub_sub_tlv);

    d4_defs.insert(sub_tlv.t, sub_tlv);

    let sub_tlv = DOCSIS4TLV {
            t: 0x24,
            description: "StreamAggregateServiceFlowReference".to_string(),
            data_type: DATATYPE::AGGREGATE,
            tlv: TLV { t: 0x24, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };

    //Sub Sub TLVs will go here

    d4_defs.insert(sub_tlv.t, sub_tlv);
    let sub_tlv = DOCSIS4TLV {
            t: 0x25,
            description: "StreamMESPReference".to_string(),
            data_type: DATATYPE::USHORT,
            tlv: TLV { t: 0x25, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);
    let mut sub_tlv = DOCSIS4TLV {
            t: 0x28,
            description: "AQMEncodings".to_string(),
            data_type: DATATYPE::AGGREGATE,
            tlv: TLV { t: 0x28, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    let sub_sub_tlv = DOCSIS4TLV {
            t: 0x01,
            description: "SFAQMDisable".to_string(),
            data_type: DATATYPE::UCHAR,
            tlv: TLV { t: 0x01, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_tlv.sub_tlvs.insert(sub_sub_tlv.t, sub_sub_tlv);
    let sub_sub_tlv = DOCSIS4TLV {
            t: 0x02,
            description: "SFAQMLatencyTarget".to_string(),
            data_type: DATATYPE::UCHAR,
            tlv: TLV { t: 0x02, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_tlv.sub_tlvs.insert(sub_sub_tlv.t, sub_sub_tlv);
    d4_defs.insert(sub_tlv.t, sub_tlv);

    let sub_tlv = DOCSIS4TLV {
            t: 0x29,
            description: "DataRateUnitSetting".to_string(),
            data_type: DATATYPE::USHORT,
            tlv: TLV { t: 0x29, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    d4_defs.insert(sub_tlv.t, sub_tlv);

    //VendorSpecific aggregate
    let mut sub_tlv = DOCSIS4TLV {
            t: 0x2b,
            description: "VendorSpecific".to_string(),
            data_type: DATATYPE::AGGREGATE,
            tlv: TLV { t: 0x2b, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    let sub_sub_tlv = DOCSIS4TLV {
            t: 0x01,
            description: "CMLoadBalancingPolicyID".to_string(),
            data_type: DATATYPE::UINT,
            tlv: TLV { t: 0x01, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_tlv.sub_tlvs.insert(sub_sub_tlv.t, sub_sub_tlv);
    let  sub_sub_tlv = DOCSIS4TLV {
            t: 0x02,
            description: "CMLoadBalancingPriority".to_string(),
            data_type: DATATYPE::UINT,
            tlv: TLV { t: 0x02, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_tlv.sub_tlvs.insert(sub_sub_tlv.t, sub_sub_tlv);
    let  sub_sub_tlv = DOCSIS4TLV {
            t: 0x03,
            description: "CMLoadBalancingGroupID".to_string(),
            data_type: DATATYPE::UINT,
            tlv: TLV { t: 0x03, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_tlv.sub_tlvs.insert(sub_sub_tlv.t, sub_sub_tlv);
    let  sub_sub_tlv = DOCSIS4TLV {
            t: 0x04,
            description: "CMRangingClassIDExtension".to_string(),
            data_type: DATATYPE::USHORT,
            tlv: TLV { t: 0x04, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_tlv.sub_tlvs.insert(sub_sub_tlv.t, sub_sub_tlv);
    let mut sub_sub_tlv = DOCSIS4TLV {
            t: 0x05,
            description: "L2VPNEncoding".to_string(),
            data_type: DATATYPE::AGGREGATE,
            tlv: TLV { t: 0x05, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    let  sub_sub_sub_tlv = DOCSIS4TLV {
            t: 0x01,
            description: "VPNIdentifier".to_string(),
            data_type: DATATYPE::HEXSTR,
            tlv: TLV { t: 0x01, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_sub_tlv.sub_tlvs.insert(sub_sub_sub_tlv.t, sub_sub_sub_tlv);
    let mut sub_sub_sub_tlv = DOCSIS4TLV {
            t: 0x02,
            description: "NSIEncapsulation".to_string(),
            data_type: DATATYPE::AGGREGATE,
            tlv: TLV { t: 0x02, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    let  sub_sub_sub_sub_tlv = DOCSIS4TLV {
            t: 0x01,
            description: "ServiceMultiplexingValueOther".to_string(),
            data_type: DATATYPE::LENZERO,
            tlv: TLV { t: 0x01, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_sub_sub_tlv.sub_tlvs.insert(sub_sub_sub_sub_tlv.t, sub_sub_sub_sub_tlv);
    let  sub_sub_sub_sub_tlv = DOCSIS4TLV {
            t: 0x02,
            description: "NSIEncapsulationSingleQTag".to_string(),
            data_type: DATATYPE::USHORT,
            tlv: TLV { t: 0x02, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_sub_sub_tlv.sub_tlvs.insert(sub_sub_sub_sub_tlv.t, sub_sub_sub_sub_tlv);
    let  sub_sub_sub_sub_tlv = DOCSIS4TLV {
            t: 0x03,
            description: "NSIEncapsulationDualQTag".to_string(),
            data_type: DATATYPE::DUAL_QTAG,
            tlv: TLV { t: 0x03, l: 0, v: Vec::new(), sub_tlvs: Vec::new() },
            sub_tlvs: HashMap::new(),
        mib: None,
    };
    sub_sub_sub_tlv.sub_tlvs.insert(sub_sub_sub_sub_tlv.t, sub_sub_sub_sub_tlv);

    //TODO: Add more sub sub sub tlvs for TLV 24.43

    sub_sub_tlv.sub_tlvs.insert(sub_sub_sub_tlv.t, sub_sub_sub_tlv);

    sub_tlv.sub_tlvs.insert(sub_sub_tlv.t, sub_sub_tlv);

    d4_defs.insert(sub_tlv.t, sub_tlv);
    // end of VendorSpecific

    d4_defs
}

/*


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
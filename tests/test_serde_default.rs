#[cfg(feature = "serde")]
#[test]
fn test_serde_can_deserialize_defaults() {
    use ibc_proto::ibc::core::client::v1::Height;

    let data = r#"{}"#;
    let height: Height = serde_json::from_str(data).unwrap();
    assert_eq!(height.revision_number, 0u64);
    assert_eq!(height.revision_height, 0u64);
}

// This enables ibc-rs to keep supporting chains on different proto versions
// where some fields may or may not be present
#[cfg(feature = "serde")]
#[test]
fn test_serde_can_serialize_defaults() {
    use ibc_proto::ibc::core::client::v1::Height;

    let height = Height {
        revision_number: 0u64,
        revision_height: 0u64,
    };
    let str = serde_json::to_string(&height).unwrap();
    assert_eq!(str, r#"{"revisionNumber":"0","revisionHeight":"0"}"#);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_can_serialize_packet_data() {
    use ibc_proto::ibc::applications::transfer::v2::FungibleTokenPacketData;

    let packet_data = FungibleTokenPacketData {
        denom: "uatom".to_string(),
        amount: "10".to_string(),
        sender: "cosmos1wxeyh7zgn4tctjzs0vtqpc6p5cxq5t2muzl7ng".to_string(),
        receiver: "cosmos1wxeyh7zgn4tctjzs0vtqpc6p5cxq5t2muzl7ng".to_string(),
        memo: "".to_string(),
    };
    let str = serde_json::to_string(&packet_data).unwrap();
    assert_eq!(
        str,
        r#"{"denom":"uatom","amount":"10","sender":"cosmos1wxeyh7zgn4tctjzs0vtqpc6p5cxq5t2muzl7ng","receiver":"cosmos1wxeyh7zgn4tctjzs0vtqpc6p5cxq5t2muzl7ng","memo":""}"#
    );
}

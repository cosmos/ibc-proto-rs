#[cfg(feature = "serde")]
use ibc_proto::ibc::core::client::v1::Height;

#[cfg(feature = "serde")]
#[test]
fn test_serde_can_deserialize_defaults() {
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
    let height = Height {
        revision_number: 0u64,
        revision_height: 0u64,
    };
    let str = serde_json::to_string(&height).unwrap();
    assert_eq!(str, r#"{"revisionNumber":"0","revisionHeight":"0"}"#);
}

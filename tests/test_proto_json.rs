#[cfg(feature = "serde")]
use ibc_proto::ibc::core::client::v1::Height;

#[cfg(feature = "serde")]
#[test]
fn test_proto_json_deserialization() {
    let data = r#"
        { "revisionNumber": "5", "revisionHeight": "3928271" }
    "#;
    let height: Height = serde_json::from_str(data).unwrap();
    assert_eq!(height.revision_number, 5u64);
    assert_eq!(height.revision_height, 3928271u64);
}

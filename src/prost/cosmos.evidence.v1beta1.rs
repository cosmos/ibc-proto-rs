// This file is @generated by prost-build.
/// Equivocation implements the Evidence interface and defines evidence of double
/// signing misbehavior.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Equivocation {
    /// height is the equivocation height.
    #[prost(int64, tag = "1")]
    pub height: i64,
    /// time is the equivocation time.
    #[prost(message, optional, tag = "2")]
    pub time: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    /// power is the equivocation validator power.
    #[prost(int64, tag = "3")]
    pub power: i64,
    /// consensus_address is the equivocation validator consensus address.
    #[prost(string, tag = "4")]
    pub consensus_address: ::prost::alloc::string::String,
}
impl ::prost::Name for Equivocation {
    const NAME: &'static str = "Equivocation";
    const PACKAGE: &'static str = "cosmos.evidence.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "cosmos.evidence.v1beta1.Equivocation".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/cosmos.evidence.v1beta1.Equivocation".into()
    }
}

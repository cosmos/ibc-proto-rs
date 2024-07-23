// This file is @generated by prost-build.
/// ClientState defines the 09-localhost client state
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientState {
    /// the latest block height
    #[prost(message, optional, tag = "1")]
    pub latest_height: ::core::option::Option<
        super::super::super::core::client::v1::Height,
    >,
}
impl ::prost::Name for ClientState {
    const NAME: &'static str = "ClientState";
    const PACKAGE: &'static str = "ibc.lightclients.localhost.v2";
    fn full_name() -> ::prost::alloc::string::String {
        "ibc.lightclients.localhost.v2.ClientState".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/ibc.lightclients.localhost.v2.ClientState".into()
    }
}

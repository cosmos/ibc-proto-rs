/// ClientState defines a Rollkit rollup client that tracks the
/// consensus state of the counterparty rollup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientState {
    #[prost(message, optional, tag = "1")]
    pub tendermint_client_state: ::core::option::Option<
        super::super::tendermint::v1::ClientState,
    >,
    #[prost(message, optional, tag = "2")]
    pub da_params: ::core::option::Option<DaParams>,
}
impl ::prost::Name for ClientState {
    const NAME: &'static str = "ClientState";
    const PACKAGE: &'static str = "ibc.lightclients.rollkit.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.lightclients.rollkit.v1.{}", Self::NAME)
    }
}
/// DaParams defines the data-availability related parameters of a Rollkit rollup light.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DaParams {
    /// client ID of the DA light client
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// fraud dispute period window in nanoseconds
    #[prost(uint64, tag = "2")]
    pub fraud_period_window: u64,
}
impl ::prost::Name for DaParams {
    const NAME: &'static str = "DaParams";
    const PACKAGE: &'static str = "ibc.lightclients.rollkit.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.lightclients.rollkit.v1.{}", Self::NAME)
    }
}
/// Header defines the structure of the header for Rollkit light clients
/// operating on a Data Availability layer. It encapsulates all the information
/// necessary to update a client from a trusted Rollkit rollup ConsensusState.
/// Rollkit headers are essentially the same as Tendermint headers, but require
/// additional data in order to query the VerifyMembership RPC on the DA light client.
/// For that reason the Header structure comprises of a Tendermint header and
/// the extra data required to query the RPC endpoint on update state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    /// the Tendermint header
    #[prost(message, optional, tag = "1")]
    pub tendermint_header: ::core::option::Option<super::super::tendermint::v1::Header>,
    /// additional data needed to query the VerifyMembership RPC on the DA light client
    #[prost(message, optional, tag = "2")]
    pub da_data: ::core::option::Option<DaData>,
}
impl ::prost::Name for Header {
    const NAME: &'static str = "Header";
    const PACKAGE: &'static str = "ibc.lightclients.rollkit.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.lightclients.rollkit.v1.{}", Self::NAME)
    }
}
/// DaData defines the information needed by Rollkit rollup light
/// client to query the VerifyMembership RPC of the DA light client.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DaData {
    /// the proof of inclusion of Rollkit rollup block data in DA block
    #[prost(bytes = "vec", tag = "2")]
    pub shared_proof: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for DaData {
    const NAME: &'static str = "DaData";
    const PACKAGE: &'static str = "ibc.lightclients.rollkit.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.lightclients.rollkit.v1.{}", Self::NAME)
    }
}

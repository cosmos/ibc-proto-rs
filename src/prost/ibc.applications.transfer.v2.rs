/// FungibleTokenPacketData defines a struct for the packet payload
/// See FungibleTokenPacketData spec:
/// <https://github.com/cosmos/ibc/tree/master/spec/app/ics-020-fungible-token-transfer#data-structures>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FungibleTokenPacketData {
    /// the token denomination to be transferred
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// the token amount to be transferred
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    /// the sender address
    #[prost(string, tag = "3")]
    pub sender: ::prost::alloc::string::String,
    /// the recipient address on the destination chain
    #[prost(string, tag = "4")]
    pub receiver: ::prost::alloc::string::String,
    /// optional memo
    #[prost(string, tag = "5")]
    pub memo: ::prost::alloc::string::String,
}
impl ::prost::Name for FungibleTokenPacketData {
    const NAME: &'static str = "FungibleTokenPacketData";
    const PACKAGE: &'static str = "ibc.applications.transfer.v2";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.applications.transfer.v2.{}", Self::NAME)
    }
}
/// FungibleTokenPacketDataV2 defines a struct for the packet payload
/// See FungibleTokenPacketDataV2 spec:
/// <https://github.com/cosmos/ibc/tree/master/spec/app/ics-020-fungible-token-transfer#data-structures>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FungibleTokenPacketDataV2 {
    /// the tokens to be transferred
    #[prost(message, repeated, tag = "1")]
    pub tokens: ::prost::alloc::vec::Vec<Token>,
    /// the sender address
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    /// the recipient address on the destination chain
    #[prost(string, tag = "3")]
    pub receiver: ::prost::alloc::string::String,
    /// optional memo
    #[prost(string, tag = "4")]
    pub memo: ::prost::alloc::string::String,
}
impl ::prost::Name for FungibleTokenPacketDataV2 {
    const NAME: &'static str = "FungibleTokenPacketDataV2";
    const PACKAGE: &'static str = "ibc.applications.transfer.v2";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.applications.transfer.v2.{}", Self::NAME)
    }
}
/// Token defines a struct which represents a token to be transferred.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Token {
    /// the base token denomination to be transferred
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// the token amount to be transferred
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    /// the trace of the token
    #[prost(string, repeated, tag = "3")]
    pub trace: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for Token {
    const NAME: &'static str = "Token";
    const PACKAGE: &'static str = "ibc.applications.transfer.v2";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.applications.transfer.v2.{}", Self::NAME)
    }
}

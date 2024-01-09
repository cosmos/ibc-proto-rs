/// ClientState defines the client state for the Sovereign SDK rollups operating
/// on a Tendermint-based data availability layer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientState {
    /// the rollup identifier
    #[prost(string, tag = "1")]
    pub rollup_id: ::prost::alloc::string::String,
    /// the latest height the client was updated to
    #[prost(message, optional, tag = "2")]
    pub latest_height: ::core::option::Option<
        super::super::super::super::core::client::v1::Height,
    >,
    /// the height when the client was frozen due to a misbehaviour
    #[prost(message, optional, tag = "3")]
    pub frozen_height: ::core::option::Option<
        super::super::super::super::core::client::v1::Height,
    >,
    /// the path at which next upgraded client will be committed. Each element
    /// corresponds to the key for a single CommitmentProof in the chained proof.
    /// NOTE: ClientState must stored under
    /// `{upgradePath}/{upgradeHeight}/clientState` ConsensusState must be stored
    /// under `{upgradepath}/{upgradeHeight}/consensusState`
    #[prost(string, repeated, tag = "4")]
    pub upgrade_path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// the tendermint params
    #[prost(message, optional, tag = "5")]
    pub tendermint_params: ::core::option::Option<TendermintParams>,
}
impl ::prost::Name for ClientState {
    const NAME: &'static str = "ClientState";
    const PACKAGE: &'static str = "ibc.lightclients.sovereign.tendermint.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!(
            "ibc.lightclients.sovereign.tendermint.v1.{}", Self::NAME
        )
    }
}
/// TendermintParams contains the data necessary to verify Tendermint headers
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TendermintParams {
    /// the identifier of the chain hosting the Tendermint consensus
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    /// the trust level of the chain
    #[prost(message, optional, tag = "2")]
    pub trust_level: ::core::option::Option<Fraction>,
    /// the duration of the period since the LastestTimestamp during which the
    /// submitted headers are valid for upgrade
    #[prost(message, optional, tag = "3")]
    pub trusting_period: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Duration,
    >,
    /// the duration of the staking unbonding period
    #[prost(message, optional, tag = "4")]
    pub unbonding_period: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Duration,
    >,
    /// the duration of new (untrusted) header's Time can drift into the future.
    #[prost(message, optional, tag = "5")]
    pub max_clock_drift: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Duration,
    >,
}
impl ::prost::Name for TendermintParams {
    const NAME: &'static str = "TendermintParams";
    const PACKAGE: &'static str = "ibc.lightclients.sovereign.tendermint.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!(
            "ibc.lightclients.sovereign.tendermint.v1.{}", Self::NAME
        )
    }
}
/// Fraction defines the protobuf message type for tmmath.Fraction that only
/// supports positive values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fraction {
    /// the numerator
    #[prost(uint64, tag = "1")]
    pub numerator: u64,
    /// the denominator
    #[prost(uint64, tag = "2")]
    pub denominator: u64,
}
impl ::prost::Name for Fraction {
    const NAME: &'static str = "Fraction";
    const PACKAGE: &'static str = "ibc.lightclients.sovereign.tendermint.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!(
            "ibc.lightclients.sovereign.tendermint.v1.{}", Self::NAME
        )
    }
}
/// ConsensusState defines the consensus state for the Sovereign SDK rollups
/// operating on a Tendermint-based data availability layer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusState {
    /// the timestamp that corresponds to the rollup height in which the
    /// ConsensusState was stored.
    #[prost(message, optional, tag = "1")]
    pub timestamp: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Timestamp,
    >,
    /// the root of rollup state at the ConsensusState height
    #[prost(message, optional, tag = "2")]
    pub root: ::core::option::Option<
        super::super::super::super::core::commitment::v1::MerkleRoot,
    >,
    /// the hash of the next validator set
    #[prost(bytes = "vec", tag = "3")]
    pub next_validators_hash: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for ConsensusState {
    const NAME: &'static str = "ConsensusState";
    const PACKAGE: &'static str = "ibc.lightclients.sovereign.tendermint.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!(
            "ibc.lightclients.sovereign.tendermint.v1.{}", Self::NAME
        )
    }
}
/// Header defines the structure of the header for the Sovereign SDK light
/// clients operating on a Tendermint-based data availability layer.
/// It encapsulates all the information necessary to update client from a trusted
/// rollup ConsensusState.
/// The TrustedHeight is the height of a stored ConsensusState on the client that
/// will be used to verify the new untrusted header. The Trusted ConsensusState
/// must be within the unbonding period of current time in order to correctly
/// verify, and the TrustedValidators must hash to
/// TrustedConsensusState.NextValidatorsHash since that is the last trusted
/// validator set at the TrustedHeight.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    /// the Tendermint header
    #[prost(message, optional, tag = "1")]
    pub da_header: ::core::option::Option<super::super::super::tendermint::v1::Header>,
    /// the rollup header
    #[prost(message, optional, tag = "2")]
    pub aggregated_proof_data: ::core::option::Option<
        super::super::super::super::super::sovereign::types::v1::AggregatedProofData,
    >,
}
impl ::prost::Name for Header {
    const NAME: &'static str = "Header";
    const PACKAGE: &'static str = "ibc.lightclients.sovereign.tendermint.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!(
            "ibc.lightclients.sovereign.tendermint.v1.{}", Self::NAME
        )
    }
}
/// SovTendermintMisbehaviour defines the misbehaviour for the Sovereign SDK
/// rollups with Tendermint-based data availability layer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Misbehaviour {
    /// the client identifier
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// the header_1 of the Sovereign SDK rollup with Tendermint-based DA layer
    #[prost(message, optional, tag = "2")]
    pub header_1: ::core::option::Option<Header>,
    /// the header_2 of the Sovereign SDK rollup with Tendermint-based DA layer
    #[prost(message, optional, tag = "3")]
    pub header_2: ::core::option::Option<Header>,
}
impl ::prost::Name for Misbehaviour {
    const NAME: &'static str = "Misbehaviour";
    const PACKAGE: &'static str = "ibc.lightclients.sovereign.tendermint.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!(
            "ibc.lightclients.sovereign.tendermint.v1.{}", Self::NAME
        )
    }
}

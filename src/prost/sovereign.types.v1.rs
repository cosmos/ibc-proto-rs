/// AggregatedProofData is the overarching structure, encompassing public input,
/// proof data information, and aggregated proof bytes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregatedProofData {
    /// the public input
    #[prost(message, optional, tag = "1")]
    pub public_input: ::core::option::Option<PublicInput>,
    /// the proof data info
    #[prost(message, optional, tag = "2")]
    pub proof_data_info: ::core::option::Option<ProofDataInfo>,
    /// the aggregated proof bytes
    #[prost(message, optional, tag = "3")]
    pub aggregated_proof: ::core::option::Option<AggregatedProof>,
}
impl ::prost::Name for AggregatedProofData {
    const NAME: &'static str = "AggregatedProofData";
    const PACKAGE: &'static str = "sovereign.types.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("sovereign.types.v1.{}", Self::NAME)
    }
}
/// PublicInput defines the public properties of the AggregatedProof for the
/// Sovereign SDK rollups, utilized for verifying the proof.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicInput {
    /// the initial DA block hash
    #[prost(bytes = "vec", tag = "1")]
    pub initial_da_block_hash: ::prost::alloc::vec::Vec<u8>,
    /// the final DA block hash
    #[prost(bytes = "vec", tag = "2")]
    pub final_da_block_hash: ::prost::alloc::vec::Vec<u8>,
    /// the initial state root
    #[prost(bytes = "vec", tag = "3")]
    pub initial_state_root: ::prost::alloc::vec::Vec<u8>,
    /// the final state root
    #[prost(bytes = "vec", tag = "4")]
    pub final_state_root: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for PublicInput {
    const NAME: &'static str = "PublicInput";
    const PACKAGE: &'static str = "sovereign.types.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("sovereign.types.v1.{}", Self::NAME)
    }
}
/// ProofDataInfo contains additional information about the proof.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProofDataInfo {
    /// the initial height of the DA block from which the proof covers the updating
    /// of the initial state root.
    #[prost(uint64, tag = "1")]
    pub initial_state_height: u64,
    /// the last height of the DA block, up to which the proof captures the
    /// rollup's transition to the final state root.
    #[prost(uint64, tag = "2")]
    pub final_state_height: u64,
}
impl ::prost::Name for ProofDataInfo {
    const NAME: &'static str = "ProofDataInfo";
    const PACKAGE: &'static str = "sovereign.types.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("sovereign.types.v1.{}", Self::NAME)
    }
}
/// AggregatedProof defines the aggregated proof structure for the Sovereign SDK
/// rollups.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregatedProof {
    /// the rollup aggregated proof bytes covering a range of DA blocks
    #[prost(bytes = "vec", tag = "1")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for AggregatedProof {
    const NAME: &'static str = "AggregatedProof";
    const PACKAGE: &'static str = "sovereign.types.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("sovereign.types.v1.{}", Self::NAME)
    }
}

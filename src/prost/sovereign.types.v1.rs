/// AggregatedProofData is the overarching structure, encompassing public input,
/// proof data information, and aggregated proof bytes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregatedProofData {
    /// the public input of the aggregated proof
    #[prost(message, optional, tag = "1")]
    pub public_input: ::core::option::Option<AggregatedProofPublicInput>,
    /// the aggregated proof bytes
    #[prost(message, optional, tag = "2")]
    pub aggregated_proof: ::core::option::Option<AggregatedProof>,
}
impl ::prost::Name for AggregatedProofData {
    const NAME: &'static str = "AggregatedProofData";
    const PACKAGE: &'static str = "sovereign.types.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("sovereign.types.v1.{}", Self::NAME)
    }
}
/// AggregatedProofPublicInput defines the public properties of the
/// AggregatedProof for the Sovereign SDK rollups, utilized for verifying the
/// proof.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregatedProofPublicInput {
    /// The set of validity conditions for each block of the aggregated proof.
    #[prost(message, repeated, tag = "1")]
    pub validity_conditions: ::prost::alloc::vec::Vec<ValidityCondition>,
    /// The initial slot number of the rollup from which the proof captures the
    /// rollup's transition from the initial state root.
    #[prost(uint64, tag = "2")]
    pub initial_slot_number: u64,
    /// the final slot number of the rollup, up to which the proof captures the
    /// rollup's transition to the final state root.
    #[prost(uint64, tag = "3")]
    pub final_slot_number: u64,
    /// the genesis state root
    #[prost(bytes = "vec", tag = "4")]
    pub genesis_state_root: ::prost::alloc::vec::Vec<u8>,
    /// the initial state root
    #[prost(bytes = "vec", tag = "5")]
    pub initial_state_root: ::prost::alloc::vec::Vec<u8>,
    /// the final state root
    #[prost(bytes = "vec", tag = "6")]
    pub final_state_root: ::prost::alloc::vec::Vec<u8>,
    /// the initial DA block hash
    #[prost(bytes = "vec", tag = "7")]
    pub initial_da_block_hash: ::prost::alloc::vec::Vec<u8>,
    /// the final DA block hash
    #[prost(bytes = "vec", tag = "8")]
    pub final_da_block_hash: ::prost::alloc::vec::Vec<u8>,
    /// the code commitment of the aggregated proof circuit
    #[prost(message, optional, tag = "9")]
    pub code_commitment: ::core::option::Option<CodeCommitment>,
}
impl ::prost::Name for AggregatedProofPublicInput {
    const NAME: &'static str = "AggregatedProofPublicInput";
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
/// ValidityCondition defines the validity condition for each block of the
/// aggregated proof
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidityCondition {
    /// the validity condition
    #[prost(bytes = "vec", tag = "1")]
    pub validity_condition: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for ValidityCondition {
    const NAME: &'static str = "ValidityCondition";
    const PACKAGE: &'static str = "sovereign.types.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("sovereign.types.v1.{}", Self::NAME)
    }
}
/// CodeCommitment defines the code commitment of the aggregated proof circuit
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CodeCommitment {
    /// the code commitment
    #[prost(bytes = "vec", tag = "1")]
    pub code_commitment: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for CodeCommitment {
    const NAME: &'static str = "CodeCommitment";
    const PACKAGE: &'static str = "sovereign.types.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("sovereign.types.v1.{}", Self::NAME)
    }
}

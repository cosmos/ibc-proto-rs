/// This packet is sent from provider chain to consumer chain if the validator
/// set for consumer chain changes (due to new bonding/unbonding messages or
/// slashing events) A VSCMatured packet from consumer chain will be sent
/// asynchronously once unbonding period is over, and this will function as
/// `UnbondingOver` message for this packet.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSetChangePacketData {
    #[prost(message, repeated, tag = "1")]
    pub validator_updates: ::prost::alloc::vec::Vec<
        ::tendermint_proto::abci::ValidatorUpdate,
    >,
    #[prost(uint64, tag = "2")]
    pub valset_update_id: u64,
    /// consensus address of consumer chain validators
    /// successfully slashed on the provider chain
    #[prost(string, repeated, tag = "3")]
    pub slash_acks: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// List of ccv.ValidatorSetChangePacketData.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSetChangePackets {
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<ValidatorSetChangePacketData>,
}
/// This packet is sent from the consumer chain to the provider chain
/// to notify that a VSC packet reached maturity on the consumer chain.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VscMaturedPacketData {
    /// the id of the VSC packet that reached maturity
    #[prost(uint64, tag = "1")]
    pub valset_update_id: u64,
}
/// This packet is sent from the consumer chain to the provider chain
/// to request the slashing of a validator as a result of an infraction
/// committed on the consumer chain.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SlashPacketData {
    #[prost(message, optional, tag = "1")]
    pub validator: ::core::option::Option<::tendermint_proto::abci::Validator>,
    /// map to the infraction block height on the provider
    #[prost(uint64, tag = "2")]
    pub valset_update_id: u64,
    /// tell if the slashing is for a downtime or a double-signing infraction
    #[prost(
        enumeration = "super::super::super::cosmos::staking::v1beta1::Infraction",
        tag = "3"
    )]
    pub infraction: i32,
}
/// MaturedUnbondingOps defines a list of ids corresponding to ids of matured
/// unbonding operations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaturedUnbondingOps {
    #[prost(uint64, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<u64>,
}
/// ConsumerPacketData contains a consumer packet data, type tag, and index for storage.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsumerPacketData {
    #[prost(enumeration = "ConsumerPacketDataType", tag = "1")]
    pub r#type: i32,
    #[prost(oneof = "consumer_packet_data::Data", tags = "2, 3")]
    pub data: ::core::option::Option<consumer_packet_data::Data>,
}
/// Nested message and enum types in `ConsumerPacketData`.
pub mod consumer_packet_data {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag = "2")]
        SlashPacketData(super::SlashPacketData),
        #[prost(message, tag = "3")]
        VscMaturedPacketData(super::VscMaturedPacketData),
    }
}
/// ConsumerPacketDataList is a list of consumer packet data packets.
/// NOTE: It is only used for exporting / importing state in InitGenesis and ExportGenesis.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsumerPacketDataList {
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<ConsumerPacketData>,
}
/// ConsumerPacketData contains a consumer packet data and a type tag
/// that is compatible with ICS v1 and v2 over the wire. It is not used for internal storage.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsumerPacketDataV1 {
    #[prost(enumeration = "ConsumerPacketDataType", tag = "1")]
    pub r#type: i32,
    #[prost(oneof = "consumer_packet_data_v1::Data", tags = "2, 3")]
    pub data: ::core::option::Option<consumer_packet_data_v1::Data>,
}
/// Nested message and enum types in `ConsumerPacketDataV1`.
pub mod consumer_packet_data_v1 {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag = "2")]
        SlashPacketData(super::SlashPacketDataV1),
        #[prost(message, tag = "3")]
        VscMaturedPacketData(super::VscMaturedPacketData),
    }
}
/// This packet is sent from the consumer chain to the provider chain
/// It is backward compatible with the ICS v1 and v2 version of the packet.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SlashPacketDataV1 {
    #[prost(message, optional, tag = "1")]
    pub validator: ::core::option::Option<::tendermint_proto::abci::Validator>,
    /// map to the infraction block height on the provider
    #[prost(uint64, tag = "2")]
    pub valset_update_id: u64,
    /// tell if the slashing is for a downtime or a double-signing infraction
    #[prost(enumeration = "InfractionType", tag = "3")]
    pub infraction: i32,
}
/// ConsumerPacketType indicates interchain security specific packet types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConsumerPacketDataType {
    /// UNSPECIFIED packet type
    ConsumerPacketTypeUnspecified = 0,
    /// Slash packet
    ConsumerPacketTypeSlash = 1,
    /// VSCMatured packet
    ConsumerPacketTypeVscm = 2,
}
impl ConsumerPacketDataType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConsumerPacketDataType::ConsumerPacketTypeUnspecified => {
                "CONSUMER_PACKET_TYPE_UNSPECIFIED"
            }
            ConsumerPacketDataType::ConsumerPacketTypeSlash => {
                "CONSUMER_PACKET_TYPE_SLASH"
            }
            ConsumerPacketDataType::ConsumerPacketTypeVscm => "CONSUMER_PACKET_TYPE_VSCM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CONSUMER_PACKET_TYPE_UNSPECIFIED" => {
                Some(Self::ConsumerPacketTypeUnspecified)
            }
            "CONSUMER_PACKET_TYPE_SLASH" => Some(Self::ConsumerPacketTypeSlash),
            "CONSUMER_PACKET_TYPE_VSCM" => Some(Self::ConsumerPacketTypeVscm),
            _ => None,
        }
    }
}
/// InfractionType indicates the infraction type a validator commited.
/// NOTE: ccv.InfractionType to maintain compatibility between ICS versions
/// using different versions of the cosmos-sdk and ibc-go modules.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InfractionType {
    /// UNSPECIFIED defines an empty infraction type.
    Unspecified = 0,
    /// DOUBLE_SIGN defines a validator that double-signs a block.
    DoubleSign = 1,
    /// DOWNTIME defines a validator that missed signing too many blocks.
    Downtime = 2,
}
impl InfractionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InfractionType::Unspecified => "INFRACTION_TYPE_UNSPECIFIED",
            InfractionType::DoubleSign => "INFRACTION_TYPE_DOUBLE_SIGN",
            InfractionType::Downtime => "INFRACTION_TYPE_DOWNTIME",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INFRACTION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "INFRACTION_TYPE_DOUBLE_SIGN" => Some(Self::DoubleSign),
            "INFRACTION_TYPE_DOWNTIME" => Some(Self::Downtime),
            _ => None,
        }
    }
}

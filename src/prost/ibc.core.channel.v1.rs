/// Channel defines pipeline for exactly-once packet delivery between specific
/// modules on separate blockchains, which has at least one end capable of
/// sending packets and one end capable of receiving packets.
#[cfg_attr(
    all(feature = "json-schema", feature = "serde"),
    derive(::schemars::JsonSchema)
)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Channel {
    /// current state of the channel end
    #[prost(enumeration = "State", tag = "1")]
    pub state: i32,
    /// whether the channel is ordered or unordered
    #[prost(enumeration = "Order", tag = "2")]
    pub ordering: i32,
    /// counterparty channel end
    #[prost(message, optional, tag = "3")]
    pub counterparty: ::core::option::Option<Counterparty>,
    /// list of connection identifiers, in order, along which packets sent on
    /// this channel will travel
    #[prost(string, repeated, tag = "4")]
    pub connection_hops: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// opaque channel version, which is agreed upon during the handshake
    #[prost(string, tag = "5")]
    pub version: ::prost::alloc::string::String,
    /// upgrade sequence indicates the latest upgrade attempt performed by this channel
    /// the value of 0 indicates the channel has never been upgraded
    #[prost(uint64, tag = "6")]
    pub upgrade_sequence: u64,
}
impl ::prost::Name for Channel {
    const NAME: &'static str = "Channel";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// IdentifiedChannel defines a channel with additional port and channel
/// identifier fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentifiedChannel {
    /// current state of the channel end
    #[prost(enumeration = "State", tag = "1")]
    pub state: i32,
    /// whether the channel is ordered or unordered
    #[prost(enumeration = "Order", tag = "2")]
    pub ordering: i32,
    /// counterparty channel end
    #[prost(message, optional, tag = "3")]
    pub counterparty: ::core::option::Option<Counterparty>,
    /// list of connection identifiers, in order, along which packets sent on
    /// this channel will travel
    #[prost(string, repeated, tag = "4")]
    pub connection_hops: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// opaque channel version, which is agreed upon during the handshake
    #[prost(string, tag = "5")]
    pub version: ::prost::alloc::string::String,
    /// port identifier
    #[prost(string, tag = "6")]
    pub port_id: ::prost::alloc::string::String,
    /// channel identifier
    #[prost(string, tag = "7")]
    pub channel_id: ::prost::alloc::string::String,
    /// upgrade sequence indicates the latest upgrade attempt performed by this channel
    /// the value of 0 indicates the channel has never been upgraded
    #[prost(uint64, tag = "8")]
    pub upgrade_sequence: u64,
}
impl ::prost::Name for IdentifiedChannel {
    const NAME: &'static str = "IdentifiedChannel";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// Counterparty defines a channel end counterparty
#[cfg_attr(
    all(feature = "json-schema", feature = "serde"),
    derive(::schemars::JsonSchema)
)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Counterparty {
    /// port on the counterparty chain which owns the other end of the channel.
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel end on the counterparty chain
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
}
impl ::prost::Name for Counterparty {
    const NAME: &'static str = "Counterparty";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// Packet defines a type that carries data across different chains through IBC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Packet {
    /// number corresponds to the order of sends and receives, where a Packet
    /// with an earlier sequence number must be sent and received before a Packet
    /// with a later sequence number.
    #[prost(uint64, tag = "1")]
    pub sequence: u64,
    /// identifies the port on the sending chain.
    #[prost(string, tag = "2")]
    pub source_port: ::prost::alloc::string::String,
    /// identifies the channel end on the sending chain.
    #[prost(string, tag = "3")]
    pub source_channel: ::prost::alloc::string::String,
    /// identifies the port on the receiving chain.
    #[prost(string, tag = "4")]
    pub destination_port: ::prost::alloc::string::String,
    /// identifies the channel end on the receiving chain.
    #[prost(string, tag = "5")]
    pub destination_channel: ::prost::alloc::string::String,
    /// actual opaque bytes transferred directly to the application module
    #[prost(bytes = "vec", tag = "6")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// block height after which the packet times out
    #[prost(message, optional, tag = "7")]
    pub timeout_height: ::core::option::Option<super::super::client::v1::Height>,
    /// block timestamp (in nanoseconds) after which the packet times out
    #[prost(uint64, tag = "8")]
    pub timeout_timestamp: u64,
}
impl ::prost::Name for Packet {
    const NAME: &'static str = "Packet";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// PacketState defines the generic type necessary to retrieve and store
/// packet commitments, acknowledgements, and receipts.
/// Caller is responsible for knowing the context necessary to interpret this
/// state as a commitment, acknowledgement, or a receipt.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PacketState {
    /// channel port identifier.
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier.
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    /// packet sequence.
    #[prost(uint64, tag = "3")]
    pub sequence: u64,
    /// embedded data that represents packet state.
    #[prost(bytes = "vec", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for PacketState {
    const NAME: &'static str = "PacketState";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// PacketId is an identifer for a unique Packet
/// Source chains refer to packets by source port/channel
/// Destination chains refer to packets by destination port/channel
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PacketId {
    /// channel port identifier
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    /// packet sequence
    #[prost(uint64, tag = "3")]
    pub sequence: u64,
}
impl ::prost::Name for PacketId {
    const NAME: &'static str = "PacketId";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// Acknowledgement is the recommended acknowledgement format to be used by
/// app-specific protocols.
/// NOTE: The field numbers 21 and 22 were explicitly chosen to avoid accidental
/// conflicts with other protobuf message formats used for acknowledgements.
/// The first byte of any message with this format will be the non-ASCII values
/// `0xaa` (result) or `0xb2` (error). Implemented as defined by ICS:
/// <https://github.com/cosmos/ibc/tree/master/spec/core/ics-004-channel-and-packet-semantics#acknowledgement-envelope>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Acknowledgement {
    /// response contains either a result or an error and must be non-empty
    #[prost(oneof = "acknowledgement::Response", tags = "21, 22")]
    pub response: ::core::option::Option<acknowledgement::Response>,
}
/// Nested message and enum types in `Acknowledgement`.
pub mod acknowledgement {
    /// response contains either a result or an error and must be non-empty
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(bytes, tag = "21")]
        Result(::prost::alloc::vec::Vec<u8>),
        #[prost(string, tag = "22")]
        Error(::prost::alloc::string::String),
    }
}
impl ::prost::Name for Acknowledgement {
    const NAME: &'static str = "Acknowledgement";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// Timeout defines an execution deadline structure for 04-channel handlers.
/// This includes packet lifecycle handlers as well as the upgrade handshake handlers.
/// A valid Timeout contains either one or both of a timestamp and block height (sequence).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Timeout {
    /// block height after which the packet or upgrade times out
    #[prost(message, optional, tag = "1")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
    /// block timestamp (in nanoseconds) after which the packet or upgrade times out
    #[prost(uint64, tag = "2")]
    pub timestamp: u64,
}
impl ::prost::Name for Timeout {
    const NAME: &'static str = "Timeout";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// Params defines the set of IBC channel parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// the relative timeout after which channel upgrades will time out.
    #[prost(message, optional, tag = "1")]
    pub upgrade_timeout: ::core::option::Option<Timeout>,
}
impl ::prost::Name for Params {
    const NAME: &'static str = "Params";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// State defines if a channel is in one of the following states:
/// CLOSED, INIT, TRYOPEN, OPEN, FLUSHING, FLUSHCOMPLETE or UNINITIALIZED.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum State {
    /// Default State
    UninitializedUnspecified = 0,
    /// A channel has just started the opening handshake.
    Init = 1,
    /// A channel has acknowledged the handshake step on the counterparty chain.
    Tryopen = 2,
    /// A channel has completed the handshake. Open channels are
    /// ready to send and receive packets.
    Open = 3,
    /// A channel has been closed and can no longer be used to send or receive
    /// packets.
    Closed = 4,
    /// A channel has been forced closed due to a frozen client in the connection path.
    Frozen = 5,
    /// A channel has just accepted the upgrade handshake attempt and is flushing in-flight packets.
    Flushing = 6,
    /// A channel has just completed flushing any in-flight packets.
    Flushcomplete = 7,
}
impl State {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            State::UninitializedUnspecified => "STATE_UNINITIALIZED_UNSPECIFIED",
            State::Init => "STATE_INIT",
            State::Tryopen => "STATE_TRYOPEN",
            State::Open => "STATE_OPEN",
            State::Closed => "STATE_CLOSED",
            State::Frozen => "STATE_FROZEN",
            State::Flushing => "STATE_FLUSHING",
            State::Flushcomplete => "STATE_FLUSHCOMPLETE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STATE_UNINITIALIZED_UNSPECIFIED" => Some(Self::UninitializedUnspecified),
            "STATE_INIT" => Some(Self::Init),
            "STATE_TRYOPEN" => Some(Self::Tryopen),
            "STATE_OPEN" => Some(Self::Open),
            "STATE_CLOSED" => Some(Self::Closed),
            "STATE_FROZEN" => Some(Self::Frozen),
            "STATE_FLUSHING" => Some(Self::Flushing),
            "STATE_FLUSHCOMPLETE" => Some(Self::Flushcomplete),
            _ => None,
        }
    }
}
/// Order defines if a channel is ORDERED or UNORDERED
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Order {
    /// zero-value for channel ordering
    NoneUnspecified = 0,
    /// packets can be delivered in any order, which may differ from the order in
    /// which they were sent.
    Unordered = 1,
    /// packets are delivered exactly in the order which they were sent
    Ordered = 2,
}
impl Order {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Order::NoneUnspecified => "ORDER_NONE_UNSPECIFIED",
            Order::Unordered => "ORDER_UNORDERED",
            Order::Ordered => "ORDER_ORDERED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ORDER_NONE_UNSPECIFIED" => Some(Self::NoneUnspecified),
            "ORDER_UNORDERED" => Some(Self::Unordered),
            "ORDER_ORDERED" => Some(Self::Ordered),
            _ => None,
        }
    }
}
/// GenesisState defines the ibc channel submodule's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub channels: ::prost::alloc::vec::Vec<IdentifiedChannel>,
    #[prost(message, repeated, tag = "2")]
    pub acknowledgements: ::prost::alloc::vec::Vec<PacketState>,
    #[prost(message, repeated, tag = "3")]
    pub commitments: ::prost::alloc::vec::Vec<PacketState>,
    #[prost(message, repeated, tag = "4")]
    pub receipts: ::prost::alloc::vec::Vec<PacketState>,
    #[prost(message, repeated, tag = "5")]
    pub send_sequences: ::prost::alloc::vec::Vec<PacketSequence>,
    #[prost(message, repeated, tag = "6")]
    pub recv_sequences: ::prost::alloc::vec::Vec<PacketSequence>,
    #[prost(message, repeated, tag = "7")]
    pub ack_sequences: ::prost::alloc::vec::Vec<PacketSequence>,
    /// the sequence for the next generated channel identifier
    #[prost(uint64, tag = "8")]
    pub next_channel_sequence: u64,
    #[prost(message, optional, tag = "9")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// PacketSequence defines the genesis type necessary to retrieve and store
/// next send and receive sequences.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PacketSequence {
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub sequence: u64,
}
impl ::prost::Name for PacketSequence {
    const NAME: &'static str = "PacketSequence";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// Upgrade is a verifiable type which contains the relevant information
/// for an attempted upgrade. It provides the proposed changes to the channel
/// end, the timeout for this upgrade attempt and the next packet sequence
/// which allows the counterparty to efficiently know the highest sequence it has received.
/// The next sequence send is used for pruning and upgrading from unordered to ordered channels.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Upgrade {
    #[prost(message, optional, tag = "1")]
    pub fields: ::core::option::Option<UpgradeFields>,
    #[prost(message, optional, tag = "2")]
    pub timeout: ::core::option::Option<Timeout>,
    #[prost(uint64, tag = "3")]
    pub next_sequence_send: u64,
}
impl ::prost::Name for Upgrade {
    const NAME: &'static str = "Upgrade";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// UpgradeFields are the fields in a channel end which may be changed
/// during a channel upgrade.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeFields {
    #[prost(enumeration = "Order", tag = "1")]
    pub ordering: i32,
    #[prost(string, repeated, tag = "2")]
    pub connection_hops: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
}
impl ::prost::Name for UpgradeFields {
    const NAME: &'static str = "UpgradeFields";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// ErrorReceipt defines a type which encapsulates the upgrade sequence and error associated with the
/// upgrade handshake failure. When a channel upgrade handshake is aborted both chains are expected to increment to the
/// next sequence.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorReceipt {
    /// the channel upgrade sequence
    #[prost(uint64, tag = "1")]
    pub sequence: u64,
    /// the error message detailing the cause of failure
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
impl ::prost::Name for ErrorReceipt {
    const NAME: &'static str = "ErrorReceipt";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelOpenInit defines an sdk.Msg to initialize a channel handshake. It
/// is called by a relayer on Chain A.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelOpenInit {
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub channel: ::core::option::Option<Channel>,
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgChannelOpenInit {
    const NAME: &'static str = "MsgChannelOpenInit";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelOpenInitResponse defines the Msg/ChannelOpenInit response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelOpenInitResponse {
    #[prost(string, tag = "1")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgChannelOpenInitResponse {
    const NAME: &'static str = "MsgChannelOpenInitResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelOpenInit defines a msg sent by a Relayer to try to open a channel
/// on Chain B. The version field within the Channel field has been deprecated. Its
/// value will be ignored by core IBC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelOpenTry {
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    /// Deprecated: this field is unused. Crossing hello's are no longer supported in core IBC.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub previous_channel_id: ::prost::alloc::string::String,
    /// NOTE: the version field within the channel has been deprecated. Its value will be ignored by core IBC.
    #[prost(message, optional, tag = "3")]
    pub channel: ::core::option::Option<Channel>,
    #[prost(string, tag = "4")]
    pub counterparty_version: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "5")]
    pub proof_init: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "6")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "7")]
    pub signer: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgChannelOpenTry {
    const NAME: &'static str = "MsgChannelOpenTry";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelOpenTryResponse defines the Msg/ChannelOpenTry response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelOpenTryResponse {
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgChannelOpenTryResponse {
    const NAME: &'static str = "MsgChannelOpenTryResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelOpenAck defines a msg sent by a Relayer to Chain A to acknowledge
/// the change of channel state to TRYOPEN on Chain B.
/// WARNING: a channel upgrade MUST NOT initialize an upgrade for this channel
/// in the same block as executing this message otherwise the counterparty will
/// be incapable of opening.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelOpenAck {
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub counterparty_channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub counterparty_version: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "5")]
    pub proof_try: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "6")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "7")]
    pub signer: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgChannelOpenAck {
    const NAME: &'static str = "MsgChannelOpenAck";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelOpenAckResponse defines the Msg/ChannelOpenAck response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelOpenAckResponse {}
impl ::prost::Name for MsgChannelOpenAckResponse {
    const NAME: &'static str = "MsgChannelOpenAckResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelOpenConfirm defines a msg sent by a Relayer to Chain B to
/// acknowledge the change of channel state to OPEN on Chain A.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelOpenConfirm {
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub proof_ack: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "4")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "5")]
    pub signer: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgChannelOpenConfirm {
    const NAME: &'static str = "MsgChannelOpenConfirm";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelOpenConfirmResponse defines the Msg/ChannelOpenConfirm response
/// type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelOpenConfirmResponse {}
impl ::prost::Name for MsgChannelOpenConfirmResponse {
    const NAME: &'static str = "MsgChannelOpenConfirmResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelCloseInit defines a msg sent by a Relayer to Chain A
/// to close a channel with Chain B.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelCloseInit {
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgChannelCloseInit {
    const NAME: &'static str = "MsgChannelCloseInit";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelCloseInitResponse defines the Msg/ChannelCloseInit response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelCloseInitResponse {}
impl ::prost::Name for MsgChannelCloseInitResponse {
    const NAME: &'static str = "MsgChannelCloseInitResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelCloseConfirm defines a msg sent by a Relayer to Chain B
/// to acknowledge the change of channel state to CLOSED on Chain A.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelCloseConfirm {
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub proof_init: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "4")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "5")]
    pub signer: ::prost::alloc::string::String,
    #[prost(uint64, tag = "6")]
    pub counterparty_upgrade_sequence: u64,
}
impl ::prost::Name for MsgChannelCloseConfirm {
    const NAME: &'static str = "MsgChannelCloseConfirm";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelCloseConfirmResponse defines the Msg/ChannelCloseConfirm response
/// type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelCloseConfirmResponse {}
impl ::prost::Name for MsgChannelCloseConfirmResponse {
    const NAME: &'static str = "MsgChannelCloseConfirmResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelCloseFrozen defines a msg sent by a Relayer to force close
/// a channel due to a frozen client in a multi-hop channel path.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelCloseFrozen {
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub proof_connection: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub proof_client_state: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "5")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "6")]
    pub signer: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgChannelCloseFrozen {
    const NAME: &'static str = "MsgChannelCloseFrozen";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelCloseFrozenResponse defines the Msg/ChannelFrozenConfirm response
/// type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelCloseFrozenResponse {}
impl ::prost::Name for MsgChannelCloseFrozenResponse {
    const NAME: &'static str = "MsgChannelCloseFrozenResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgRecvPacket receives incoming IBC packet
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRecvPacket {
    #[prost(message, optional, tag = "1")]
    pub packet: ::core::option::Option<Packet>,
    #[prost(bytes = "vec", tag = "2")]
    pub proof_commitment: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "4")]
    pub signer: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgRecvPacket {
    const NAME: &'static str = "MsgRecvPacket";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgRecvPacketResponse defines the Msg/RecvPacket response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRecvPacketResponse {
    #[prost(enumeration = "ResponseResultType", tag = "1")]
    pub result: i32,
}
impl ::prost::Name for MsgRecvPacketResponse {
    const NAME: &'static str = "MsgRecvPacketResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgTimeout receives timed-out packet
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTimeout {
    #[prost(message, optional, tag = "1")]
    pub packet: ::core::option::Option<Packet>,
    #[prost(bytes = "vec", tag = "2")]
    pub proof_unreceived: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(uint64, tag = "4")]
    pub next_sequence_recv: u64,
    #[prost(string, tag = "5")]
    pub signer: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgTimeout {
    const NAME: &'static str = "MsgTimeout";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgTimeoutResponse defines the Msg/Timeout response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTimeoutResponse {
    #[prost(enumeration = "ResponseResultType", tag = "1")]
    pub result: i32,
}
impl ::prost::Name for MsgTimeoutResponse {
    const NAME: &'static str = "MsgTimeoutResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgTimeoutOnClose timed-out packet upon counterparty channel closure.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTimeoutOnClose {
    #[prost(message, optional, tag = "1")]
    pub packet: ::core::option::Option<Packet>,
    #[prost(bytes = "vec", tag = "2")]
    pub proof_unreceived: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub proof_close: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "4")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(uint64, tag = "5")]
    pub next_sequence_recv: u64,
    #[prost(string, tag = "6")]
    pub signer: ::prost::alloc::string::String,
    #[prost(uint64, tag = "7")]
    pub counterparty_upgrade_sequence: u64,
}
impl ::prost::Name for MsgTimeoutOnClose {
    const NAME: &'static str = "MsgTimeoutOnClose";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgTimeoutOnCloseResponse defines the Msg/TimeoutOnClose response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTimeoutOnCloseResponse {
    #[prost(enumeration = "ResponseResultType", tag = "1")]
    pub result: i32,
}
impl ::prost::Name for MsgTimeoutOnCloseResponse {
    const NAME: &'static str = "MsgTimeoutOnCloseResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgAcknowledgement receives incoming IBC acknowledgement
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAcknowledgement {
    #[prost(message, optional, tag = "1")]
    pub packet: ::core::option::Option<Packet>,
    #[prost(bytes = "vec", tag = "2")]
    pub acknowledgement: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub proof_acked: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "4")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "5")]
    pub signer: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgAcknowledgement {
    const NAME: &'static str = "MsgAcknowledgement";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgAcknowledgementResponse defines the Msg/Acknowledgement response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAcknowledgementResponse {
    #[prost(enumeration = "ResponseResultType", tag = "1")]
    pub result: i32,
}
impl ::prost::Name for MsgAcknowledgementResponse {
    const NAME: &'static str = "MsgAcknowledgementResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelUpgradeInit defines the request type for the ChannelUpgradeInit rpc
/// WARNING: Initializing a channel upgrade in the same block as opening the channel
/// may result in the counterparty being incapable of opening.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelUpgradeInit {
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub fields: ::core::option::Option<UpgradeFields>,
    #[prost(string, tag = "4")]
    pub signer: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgChannelUpgradeInit {
    const NAME: &'static str = "MsgChannelUpgradeInit";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelUpgradeInitResponse defines the MsgChannelUpgradeInit response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelUpgradeInitResponse {
    #[prost(message, optional, tag = "1")]
    pub upgrade: ::core::option::Option<Upgrade>,
    #[prost(uint64, tag = "2")]
    pub upgrade_sequence: u64,
}
impl ::prost::Name for MsgChannelUpgradeInitResponse {
    const NAME: &'static str = "MsgChannelUpgradeInitResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelUpgradeTry defines the request type for the ChannelUpgradeTry rpc
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelUpgradeTry {
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub proposed_upgrade_connection_hops: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(message, optional, tag = "4")]
    pub counterparty_upgrade_fields: ::core::option::Option<UpgradeFields>,
    #[prost(uint64, tag = "5")]
    pub counterparty_upgrade_sequence: u64,
    #[prost(bytes = "vec", tag = "6")]
    pub proof_channel: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "7")]
    pub proof_upgrade: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "8")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "9")]
    pub signer: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgChannelUpgradeTry {
    const NAME: &'static str = "MsgChannelUpgradeTry";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelUpgradeTryResponse defines the MsgChannelUpgradeTry response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelUpgradeTryResponse {
    #[prost(message, optional, tag = "1")]
    pub upgrade: ::core::option::Option<Upgrade>,
    #[prost(uint64, tag = "2")]
    pub upgrade_sequence: u64,
    #[prost(enumeration = "ResponseResultType", tag = "3")]
    pub result: i32,
}
impl ::prost::Name for MsgChannelUpgradeTryResponse {
    const NAME: &'static str = "MsgChannelUpgradeTryResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelUpgradeAck defines the request type for the ChannelUpgradeAck rpc
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelUpgradeAck {
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub counterparty_upgrade: ::core::option::Option<Upgrade>,
    #[prost(bytes = "vec", tag = "4")]
    pub proof_channel: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    pub proof_upgrade: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "6")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "7")]
    pub signer: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgChannelUpgradeAck {
    const NAME: &'static str = "MsgChannelUpgradeAck";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelUpgradeAckResponse defines MsgChannelUpgradeAck response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelUpgradeAckResponse {
    #[prost(enumeration = "ResponseResultType", tag = "1")]
    pub result: i32,
}
impl ::prost::Name for MsgChannelUpgradeAckResponse {
    const NAME: &'static str = "MsgChannelUpgradeAckResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelUpgradeConfirm defines the request type for the ChannelUpgradeConfirm rpc
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelUpgradeConfirm {
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(enumeration = "State", tag = "3")]
    pub counterparty_channel_state: i32,
    #[prost(message, optional, tag = "4")]
    pub counterparty_upgrade: ::core::option::Option<Upgrade>,
    #[prost(bytes = "vec", tag = "5")]
    pub proof_channel: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "6")]
    pub proof_upgrade: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "7")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "8")]
    pub signer: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgChannelUpgradeConfirm {
    const NAME: &'static str = "MsgChannelUpgradeConfirm";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelUpgradeConfirmResponse defines MsgChannelUpgradeConfirm response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelUpgradeConfirmResponse {
    #[prost(enumeration = "ResponseResultType", tag = "1")]
    pub result: i32,
}
impl ::prost::Name for MsgChannelUpgradeConfirmResponse {
    const NAME: &'static str = "MsgChannelUpgradeConfirmResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelUpgradeOpen defines the request type for the ChannelUpgradeOpen rpc
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelUpgradeOpen {
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(enumeration = "State", tag = "3")]
    pub counterparty_channel_state: i32,
    #[prost(uint64, tag = "4")]
    pub counterparty_upgrade_sequence: u64,
    #[prost(bytes = "vec", tag = "5")]
    pub proof_channel: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "6")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "7")]
    pub signer: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgChannelUpgradeOpen {
    const NAME: &'static str = "MsgChannelUpgradeOpen";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelUpgradeOpenResponse defines the MsgChannelUpgradeOpen response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelUpgradeOpenResponse {}
impl ::prost::Name for MsgChannelUpgradeOpenResponse {
    const NAME: &'static str = "MsgChannelUpgradeOpenResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelUpgradeTimeout defines the request type for the ChannelUpgradeTimeout rpc
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelUpgradeTimeout {
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub counterparty_channel: ::core::option::Option<Channel>,
    #[prost(bytes = "vec", tag = "4")]
    pub proof_channel: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "5")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "6")]
    pub signer: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgChannelUpgradeTimeout {
    const NAME: &'static str = "MsgChannelUpgradeTimeout";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelUpgradeTimeoutRepsonse defines the MsgChannelUpgradeTimeout response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelUpgradeTimeoutResponse {}
impl ::prost::Name for MsgChannelUpgradeTimeoutResponse {
    const NAME: &'static str = "MsgChannelUpgradeTimeoutResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelUpgradeCancel defines the request type for the ChannelUpgradeCancel rpc
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelUpgradeCancel {
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub error_receipt: ::core::option::Option<ErrorReceipt>,
    #[prost(bytes = "vec", tag = "4")]
    pub proof_error_receipt: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "5")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "6")]
    pub signer: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgChannelUpgradeCancel {
    const NAME: &'static str = "MsgChannelUpgradeCancel";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgChannelUpgradeCancelResponse defines the MsgChannelUpgradeCancel response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelUpgradeCancelResponse {}
impl ::prost::Name for MsgChannelUpgradeCancelResponse {
    const NAME: &'static str = "MsgChannelUpgradeCancelResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgUpdateParams is the MsgUpdateParams request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the channel parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for MsgUpdateParams {
    const NAME: &'static str = "MsgUpdateParams";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgUpdateParamsResponse defines the MsgUpdateParams response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
impl ::prost::Name for MsgUpdateParamsResponse {
    const NAME: &'static str = "MsgUpdateParamsResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgPruneAcknowledgements defines the request type for the PruneAcknowledgements rpc.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPruneAcknowledgements {
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub limit: u64,
    #[prost(string, tag = "4")]
    pub signer: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgPruneAcknowledgements {
    const NAME: &'static str = "MsgPruneAcknowledgements";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgPruneAcknowledgementsResponse defines the response type for the PruneAcknowledgements rpc.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPruneAcknowledgementsResponse {
    /// Number of sequences pruned (includes both packet acknowledgements and packet receipts where appropriate).
    #[prost(uint64, tag = "1")]
    pub total_pruned_sequences: u64,
    /// Number of sequences left after pruning.
    #[prost(uint64, tag = "2")]
    pub total_remaining_sequences: u64,
}
impl ::prost::Name for MsgPruneAcknowledgementsResponse {
    const NAME: &'static str = "MsgPruneAcknowledgementsResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MultihopProof holds the information necessary to prove a multihop message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultihopProof {
    #[prost(bytes = "vec", tag = "2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "4")]
    pub prefixed_key: ::core::option::Option<super::super::commitment::v1::MerklePath>,
}
impl ::prost::Name for MultihopProof {
    const NAME: &'static str = "MultihopProof";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// MsgMultihopProofs holds the proof information for each intermediary hop for a multihop message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMultihopProofs {
    #[prost(message, optional, tag = "2")]
    pub key_proof: ::core::option::Option<MultihopProof>,
    #[prost(message, repeated, tag = "3")]
    pub connection_proofs: ::prost::alloc::vec::Vec<MultihopProof>,
    #[prost(message, repeated, tag = "4")]
    pub consensus_proofs: ::prost::alloc::vec::Vec<MultihopProof>,
}
impl ::prost::Name for MsgMultihopProofs {
    const NAME: &'static str = "MsgMultihopProofs";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// ResponseResultType defines the possible outcomes of the execution of a message
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResponseResultType {
    /// Default zero value enumeration
    Unspecified = 0,
    /// The message did not call the IBC application callbacks (because, for example, the packet had already been relayed)
    Noop = 1,
    /// The message was executed successfully
    Success = 2,
    /// The message was executed unsuccessfully
    Failure = 3,
}
impl ResponseResultType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResponseResultType::Unspecified => "RESPONSE_RESULT_TYPE_UNSPECIFIED",
            ResponseResultType::Noop => "RESPONSE_RESULT_TYPE_NOOP",
            ResponseResultType::Success => "RESPONSE_RESULT_TYPE_SUCCESS",
            ResponseResultType::Failure => "RESPONSE_RESULT_TYPE_FAILURE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RESPONSE_RESULT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "RESPONSE_RESULT_TYPE_NOOP" => Some(Self::Noop),
            "RESPONSE_RESULT_TYPE_SUCCESS" => Some(Self::Success),
            "RESPONSE_RESULT_TYPE_FAILURE" => Some(Self::Failure),
            _ => None,
        }
    }
}
/// Generated client implementations.
#[cfg(feature = "client")]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Msg defines the ibc/channel Msg service.
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MsgClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MsgClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MsgClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// ChannelOpenInit defines a rpc handler method for MsgChannelOpenInit.
        pub async fn channel_open_init(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgChannelOpenInit>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelOpenInitResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/ChannelOpenInit",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.channel.v1.Msg", "ChannelOpenInit"));
            self.inner.unary(req, path, codec).await
        }
        /// ChannelOpenTry defines a rpc handler method for MsgChannelOpenTry.
        pub async fn channel_open_try(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgChannelOpenTry>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelOpenTryResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/ChannelOpenTry",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.channel.v1.Msg", "ChannelOpenTry"));
            self.inner.unary(req, path, codec).await
        }
        /// ChannelOpenAck defines a rpc handler method for MsgChannelOpenAck.
        pub async fn channel_open_ack(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgChannelOpenAck>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelOpenAckResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/ChannelOpenAck",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.channel.v1.Msg", "ChannelOpenAck"));
            self.inner.unary(req, path, codec).await
        }
        /// ChannelOpenConfirm defines a rpc handler method for MsgChannelOpenConfirm.
        pub async fn channel_open_confirm(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgChannelOpenConfirm>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelOpenConfirmResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/ChannelOpenConfirm",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ibc.core.channel.v1.Msg", "ChannelOpenConfirm"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ChannelCloseInit defines a rpc handler method for MsgChannelCloseInit.
        pub async fn channel_close_init(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgChannelCloseInit>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelCloseInitResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/ChannelCloseInit",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.channel.v1.Msg", "ChannelCloseInit"));
            self.inner.unary(req, path, codec).await
        }
        /// ChannelCloseConfirm defines a rpc handler method for
        /// MsgChannelCloseConfirm.
        pub async fn channel_close_confirm(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgChannelCloseConfirm>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelCloseConfirmResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/ChannelCloseConfirm",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ibc.core.channel.v1.Msg", "ChannelCloseConfirm"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ChannelCloseFrozen defines a rpc handler method for MsgChannelCloseFrozen.
        pub async fn channel_close_frozen(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgChannelCloseFrozen>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelCloseFrozenResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/ChannelCloseFrozen",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ibc.core.channel.v1.Msg", "ChannelCloseFrozen"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// RecvPacket defines a rpc handler method for MsgRecvPacket.
        pub async fn recv_packet(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgRecvPacket>,
        ) -> std::result::Result<
            tonic::Response<super::MsgRecvPacketResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/RecvPacket",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.channel.v1.Msg", "RecvPacket"));
            self.inner.unary(req, path, codec).await
        }
        /// Timeout defines a rpc handler method for MsgTimeout.
        pub async fn timeout(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgTimeout>,
        ) -> std::result::Result<
            tonic::Response<super::MsgTimeoutResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/Timeout",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.channel.v1.Msg", "Timeout"));
            self.inner.unary(req, path, codec).await
        }
        /// TimeoutOnClose defines a rpc handler method for MsgTimeoutOnClose.
        pub async fn timeout_on_close(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgTimeoutOnClose>,
        ) -> std::result::Result<
            tonic::Response<super::MsgTimeoutOnCloseResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/TimeoutOnClose",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.channel.v1.Msg", "TimeoutOnClose"));
            self.inner.unary(req, path, codec).await
        }
        /// Acknowledgement defines a rpc handler method for MsgAcknowledgement.
        pub async fn acknowledgement(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAcknowledgement>,
        ) -> std::result::Result<
            tonic::Response<super::MsgAcknowledgementResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/Acknowledgement",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.channel.v1.Msg", "Acknowledgement"));
            self.inner.unary(req, path, codec).await
        }
        /// ChannelUpgradeInit defines a rpc handler method for MsgChannelUpgradeInit.
        pub async fn channel_upgrade_init(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgChannelUpgradeInit>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelUpgradeInitResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/ChannelUpgradeInit",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ibc.core.channel.v1.Msg", "ChannelUpgradeInit"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ChannelUpgradeTry defines a rpc handler method for MsgChannelUpgradeTry.
        pub async fn channel_upgrade_try(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgChannelUpgradeTry>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelUpgradeTryResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/ChannelUpgradeTry",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.channel.v1.Msg", "ChannelUpgradeTry"));
            self.inner.unary(req, path, codec).await
        }
        /// ChannelUpgradeAck defines a rpc handler method for MsgChannelUpgradeAck.
        pub async fn channel_upgrade_ack(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgChannelUpgradeAck>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelUpgradeAckResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/ChannelUpgradeAck",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.channel.v1.Msg", "ChannelUpgradeAck"));
            self.inner.unary(req, path, codec).await
        }
        /// ChannelUpgradeConfirm defines a rpc handler method for MsgChannelUpgradeConfirm.
        pub async fn channel_upgrade_confirm(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgChannelUpgradeConfirm>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelUpgradeConfirmResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/ChannelUpgradeConfirm",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ibc.core.channel.v1.Msg", "ChannelUpgradeConfirm"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ChannelUpgradeOpen defines a rpc handler method for MsgChannelUpgradeOpen.
        pub async fn channel_upgrade_open(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgChannelUpgradeOpen>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelUpgradeOpenResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/ChannelUpgradeOpen",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ibc.core.channel.v1.Msg", "ChannelUpgradeOpen"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ChannelUpgradeTimeout defines a rpc handler method for MsgChannelUpgradeTimeout.
        pub async fn channel_upgrade_timeout(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgChannelUpgradeTimeout>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelUpgradeTimeoutResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/ChannelUpgradeTimeout",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ibc.core.channel.v1.Msg", "ChannelUpgradeTimeout"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ChannelUpgradeCancel defines a rpc handler method for MsgChannelUpgradeCancel.
        pub async fn channel_upgrade_cancel(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgChannelUpgradeCancel>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelUpgradeCancelResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/ChannelUpgradeCancel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ibc.core.channel.v1.Msg", "ChannelUpgradeCancel"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpdateChannelParams defines a rpc handler method for MsgUpdateParams.
        pub async fn update_channel_params(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateParams>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateParamsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/UpdateChannelParams",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ibc.core.channel.v1.Msg", "UpdateChannelParams"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// PruneAcknowledgements defines a rpc handler method for MsgPruneAcknowledgements.
        pub async fn prune_acknowledgements(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgPruneAcknowledgements>,
        ) -> std::result::Result<
            tonic::Response<super::MsgPruneAcknowledgementsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/PruneAcknowledgements",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ibc.core.channel.v1.Msg", "PruneAcknowledgements"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "server")]
pub mod msg_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MsgServer.
    #[async_trait]
    pub trait Msg: Send + Sync + 'static {
        /// ChannelOpenInit defines a rpc handler method for MsgChannelOpenInit.
        async fn channel_open_init(
            &self,
            request: tonic::Request<super::MsgChannelOpenInit>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelOpenInitResponse>,
            tonic::Status,
        >;
        /// ChannelOpenTry defines a rpc handler method for MsgChannelOpenTry.
        async fn channel_open_try(
            &self,
            request: tonic::Request<super::MsgChannelOpenTry>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelOpenTryResponse>,
            tonic::Status,
        >;
        /// ChannelOpenAck defines a rpc handler method for MsgChannelOpenAck.
        async fn channel_open_ack(
            &self,
            request: tonic::Request<super::MsgChannelOpenAck>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelOpenAckResponse>,
            tonic::Status,
        >;
        /// ChannelOpenConfirm defines a rpc handler method for MsgChannelOpenConfirm.
        async fn channel_open_confirm(
            &self,
            request: tonic::Request<super::MsgChannelOpenConfirm>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelOpenConfirmResponse>,
            tonic::Status,
        >;
        /// ChannelCloseInit defines a rpc handler method for MsgChannelCloseInit.
        async fn channel_close_init(
            &self,
            request: tonic::Request<super::MsgChannelCloseInit>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelCloseInitResponse>,
            tonic::Status,
        >;
        /// ChannelCloseConfirm defines a rpc handler method for
        /// MsgChannelCloseConfirm.
        async fn channel_close_confirm(
            &self,
            request: tonic::Request<super::MsgChannelCloseConfirm>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelCloseConfirmResponse>,
            tonic::Status,
        >;
        /// ChannelCloseFrozen defines a rpc handler method for MsgChannelCloseFrozen.
        async fn channel_close_frozen(
            &self,
            request: tonic::Request<super::MsgChannelCloseFrozen>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelCloseFrozenResponse>,
            tonic::Status,
        >;
        /// RecvPacket defines a rpc handler method for MsgRecvPacket.
        async fn recv_packet(
            &self,
            request: tonic::Request<super::MsgRecvPacket>,
        ) -> std::result::Result<
            tonic::Response<super::MsgRecvPacketResponse>,
            tonic::Status,
        >;
        /// Timeout defines a rpc handler method for MsgTimeout.
        async fn timeout(
            &self,
            request: tonic::Request<super::MsgTimeout>,
        ) -> std::result::Result<
            tonic::Response<super::MsgTimeoutResponse>,
            tonic::Status,
        >;
        /// TimeoutOnClose defines a rpc handler method for MsgTimeoutOnClose.
        async fn timeout_on_close(
            &self,
            request: tonic::Request<super::MsgTimeoutOnClose>,
        ) -> std::result::Result<
            tonic::Response<super::MsgTimeoutOnCloseResponse>,
            tonic::Status,
        >;
        /// Acknowledgement defines a rpc handler method for MsgAcknowledgement.
        async fn acknowledgement(
            &self,
            request: tonic::Request<super::MsgAcknowledgement>,
        ) -> std::result::Result<
            tonic::Response<super::MsgAcknowledgementResponse>,
            tonic::Status,
        >;
        /// ChannelUpgradeInit defines a rpc handler method for MsgChannelUpgradeInit.
        async fn channel_upgrade_init(
            &self,
            request: tonic::Request<super::MsgChannelUpgradeInit>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelUpgradeInitResponse>,
            tonic::Status,
        >;
        /// ChannelUpgradeTry defines a rpc handler method for MsgChannelUpgradeTry.
        async fn channel_upgrade_try(
            &self,
            request: tonic::Request<super::MsgChannelUpgradeTry>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelUpgradeTryResponse>,
            tonic::Status,
        >;
        /// ChannelUpgradeAck defines a rpc handler method for MsgChannelUpgradeAck.
        async fn channel_upgrade_ack(
            &self,
            request: tonic::Request<super::MsgChannelUpgradeAck>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelUpgradeAckResponse>,
            tonic::Status,
        >;
        /// ChannelUpgradeConfirm defines a rpc handler method for MsgChannelUpgradeConfirm.
        async fn channel_upgrade_confirm(
            &self,
            request: tonic::Request<super::MsgChannelUpgradeConfirm>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelUpgradeConfirmResponse>,
            tonic::Status,
        >;
        /// ChannelUpgradeOpen defines a rpc handler method for MsgChannelUpgradeOpen.
        async fn channel_upgrade_open(
            &self,
            request: tonic::Request<super::MsgChannelUpgradeOpen>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelUpgradeOpenResponse>,
            tonic::Status,
        >;
        /// ChannelUpgradeTimeout defines a rpc handler method for MsgChannelUpgradeTimeout.
        async fn channel_upgrade_timeout(
            &self,
            request: tonic::Request<super::MsgChannelUpgradeTimeout>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelUpgradeTimeoutResponse>,
            tonic::Status,
        >;
        /// ChannelUpgradeCancel defines a rpc handler method for MsgChannelUpgradeCancel.
        async fn channel_upgrade_cancel(
            &self,
            request: tonic::Request<super::MsgChannelUpgradeCancel>,
        ) -> std::result::Result<
            tonic::Response<super::MsgChannelUpgradeCancelResponse>,
            tonic::Status,
        >;
        /// UpdateChannelParams defines a rpc handler method for MsgUpdateParams.
        async fn update_channel_params(
            &self,
            request: tonic::Request<super::MsgUpdateParams>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateParamsResponse>,
            tonic::Status,
        >;
        /// PruneAcknowledgements defines a rpc handler method for MsgPruneAcknowledgements.
        async fn prune_acknowledgements(
            &self,
            request: tonic::Request<super::MsgPruneAcknowledgements>,
        ) -> std::result::Result<
            tonic::Response<super::MsgPruneAcknowledgementsResponse>,
            tonic::Status,
        >;
    }
    /// Msg defines the ibc/channel Msg service.
    #[derive(Debug)]
    pub struct MsgServer<T: Msg> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Msg> MsgServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MsgServer<T>
    where
        T: Msg,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/ibc.core.channel.v1.Msg/ChannelOpenInit" => {
                    #[allow(non_camel_case_types)]
                    struct ChannelOpenInitSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgChannelOpenInit>
                    for ChannelOpenInitSvc<T> {
                        type Response = super::MsgChannelOpenInitResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgChannelOpenInit>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::channel_open_init(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChannelOpenInitSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Msg/ChannelOpenTry" => {
                    #[allow(non_camel_case_types)]
                    struct ChannelOpenTrySvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgChannelOpenTry>
                    for ChannelOpenTrySvc<T> {
                        type Response = super::MsgChannelOpenTryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgChannelOpenTry>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::channel_open_try(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChannelOpenTrySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Msg/ChannelOpenAck" => {
                    #[allow(non_camel_case_types)]
                    struct ChannelOpenAckSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgChannelOpenAck>
                    for ChannelOpenAckSvc<T> {
                        type Response = super::MsgChannelOpenAckResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgChannelOpenAck>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::channel_open_ack(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChannelOpenAckSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Msg/ChannelOpenConfirm" => {
                    #[allow(non_camel_case_types)]
                    struct ChannelOpenConfirmSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgChannelOpenConfirm>
                    for ChannelOpenConfirmSvc<T> {
                        type Response = super::MsgChannelOpenConfirmResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgChannelOpenConfirm>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::channel_open_confirm(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChannelOpenConfirmSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Msg/ChannelCloseInit" => {
                    #[allow(non_camel_case_types)]
                    struct ChannelCloseInitSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgChannelCloseInit>
                    for ChannelCloseInitSvc<T> {
                        type Response = super::MsgChannelCloseInitResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgChannelCloseInit>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::channel_close_init(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChannelCloseInitSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Msg/ChannelCloseConfirm" => {
                    #[allow(non_camel_case_types)]
                    struct ChannelCloseConfirmSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgChannelCloseConfirm>
                    for ChannelCloseConfirmSvc<T> {
                        type Response = super::MsgChannelCloseConfirmResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgChannelCloseConfirm>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::channel_close_confirm(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChannelCloseConfirmSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Msg/ChannelCloseFrozen" => {
                    #[allow(non_camel_case_types)]
                    struct ChannelCloseFrozenSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgChannelCloseFrozen>
                    for ChannelCloseFrozenSvc<T> {
                        type Response = super::MsgChannelCloseFrozenResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgChannelCloseFrozen>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::channel_close_frozen(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChannelCloseFrozenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Msg/RecvPacket" => {
                    #[allow(non_camel_case_types)]
                    struct RecvPacketSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgRecvPacket>
                    for RecvPacketSvc<T> {
                        type Response = super::MsgRecvPacketResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgRecvPacket>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::recv_packet(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RecvPacketSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Msg/Timeout" => {
                    #[allow(non_camel_case_types)]
                    struct TimeoutSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgTimeout>
                    for TimeoutSvc<T> {
                        type Response = super::MsgTimeoutResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgTimeout>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::timeout(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TimeoutSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Msg/TimeoutOnClose" => {
                    #[allow(non_camel_case_types)]
                    struct TimeoutOnCloseSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgTimeoutOnClose>
                    for TimeoutOnCloseSvc<T> {
                        type Response = super::MsgTimeoutOnCloseResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgTimeoutOnClose>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::timeout_on_close(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TimeoutOnCloseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Msg/Acknowledgement" => {
                    #[allow(non_camel_case_types)]
                    struct AcknowledgementSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgAcknowledgement>
                    for AcknowledgementSvc<T> {
                        type Response = super::MsgAcknowledgementResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgAcknowledgement>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::acknowledgement(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AcknowledgementSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Msg/ChannelUpgradeInit" => {
                    #[allow(non_camel_case_types)]
                    struct ChannelUpgradeInitSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgChannelUpgradeInit>
                    for ChannelUpgradeInitSvc<T> {
                        type Response = super::MsgChannelUpgradeInitResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgChannelUpgradeInit>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::channel_upgrade_init(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChannelUpgradeInitSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Msg/ChannelUpgradeTry" => {
                    #[allow(non_camel_case_types)]
                    struct ChannelUpgradeTrySvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgChannelUpgradeTry>
                    for ChannelUpgradeTrySvc<T> {
                        type Response = super::MsgChannelUpgradeTryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgChannelUpgradeTry>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::channel_upgrade_try(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChannelUpgradeTrySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Msg/ChannelUpgradeAck" => {
                    #[allow(non_camel_case_types)]
                    struct ChannelUpgradeAckSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgChannelUpgradeAck>
                    for ChannelUpgradeAckSvc<T> {
                        type Response = super::MsgChannelUpgradeAckResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgChannelUpgradeAck>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::channel_upgrade_ack(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChannelUpgradeAckSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Msg/ChannelUpgradeConfirm" => {
                    #[allow(non_camel_case_types)]
                    struct ChannelUpgradeConfirmSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgChannelUpgradeConfirm>
                    for ChannelUpgradeConfirmSvc<T> {
                        type Response = super::MsgChannelUpgradeConfirmResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgChannelUpgradeConfirm>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::channel_upgrade_confirm(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChannelUpgradeConfirmSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Msg/ChannelUpgradeOpen" => {
                    #[allow(non_camel_case_types)]
                    struct ChannelUpgradeOpenSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgChannelUpgradeOpen>
                    for ChannelUpgradeOpenSvc<T> {
                        type Response = super::MsgChannelUpgradeOpenResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgChannelUpgradeOpen>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::channel_upgrade_open(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChannelUpgradeOpenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Msg/ChannelUpgradeTimeout" => {
                    #[allow(non_camel_case_types)]
                    struct ChannelUpgradeTimeoutSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgChannelUpgradeTimeout>
                    for ChannelUpgradeTimeoutSvc<T> {
                        type Response = super::MsgChannelUpgradeTimeoutResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgChannelUpgradeTimeout>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::channel_upgrade_timeout(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChannelUpgradeTimeoutSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Msg/ChannelUpgradeCancel" => {
                    #[allow(non_camel_case_types)]
                    struct ChannelUpgradeCancelSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgChannelUpgradeCancel>
                    for ChannelUpgradeCancelSvc<T> {
                        type Response = super::MsgChannelUpgradeCancelResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgChannelUpgradeCancel>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::channel_upgrade_cancel(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChannelUpgradeCancelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Msg/UpdateChannelParams" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateChannelParamsSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUpdateParams>
                    for UpdateChannelParamsSvc<T> {
                        type Response = super::MsgUpdateParamsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpdateParams>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::update_channel_params(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateChannelParamsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Msg/PruneAcknowledgements" => {
                    #[allow(non_camel_case_types)]
                    struct PruneAcknowledgementsSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgPruneAcknowledgements>
                    for PruneAcknowledgementsSvc<T> {
                        type Response = super::MsgPruneAcknowledgementsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgPruneAcknowledgements>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::prune_acknowledgements(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PruneAcknowledgementsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Msg> Clone for MsgServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Msg> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Msg> tonic::server::NamedService for MsgServer<T> {
        const NAME: &'static str = "ibc.core.channel.v1.Msg";
    }
}
/// QueryChannelRequest is the request type for the Query/Channel RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChannelRequest {
    /// port unique identifier
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryChannelRequest {
    const NAME: &'static str = "QueryChannelRequest";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryChannelResponse is the response type for the Query/Channel RPC method.
/// Besides the Channel end, it includes a proof and the height from which the
/// proof was retrieved.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChannelResponse {
    /// channel associated with the request identifiers
    #[prost(message, optional, tag = "1")]
    pub channel: ::core::option::Option<Channel>,
    /// merkle proof of existence
    #[prost(bytes = "vec", tag = "2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
impl ::prost::Name for QueryChannelResponse {
    const NAME: &'static str = "QueryChannelResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryChannelsRequest is the request type for the Query/Channels RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChannelsRequest {
    /// pagination request
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
impl ::prost::Name for QueryChannelsRequest {
    const NAME: &'static str = "QueryChannelsRequest";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryChannelsResponse is the response type for the Query/Channels RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChannelsResponse {
    /// list of stored channels of the chain.
    #[prost(message, repeated, tag = "1")]
    pub channels: ::prost::alloc::vec::Vec<IdentifiedChannel>,
    /// pagination response
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
    /// query block height
    #[prost(message, optional, tag = "3")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
}
impl ::prost::Name for QueryChannelsResponse {
    const NAME: &'static str = "QueryChannelsResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryConnectionChannelsRequest is the request type for the
/// Query/QueryConnectionChannels RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConnectionChannelsRequest {
    /// connection unique identifier
    #[prost(string, tag = "1")]
    pub connection: ::prost::alloc::string::String,
    /// pagination request
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
impl ::prost::Name for QueryConnectionChannelsRequest {
    const NAME: &'static str = "QueryConnectionChannelsRequest";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryConnectionChannelsResponse is the Response type for the
/// Query/QueryConnectionChannels RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConnectionChannelsResponse {
    /// list of channels associated with a connection.
    #[prost(message, repeated, tag = "1")]
    pub channels: ::prost::alloc::vec::Vec<IdentifiedChannel>,
    /// pagination response
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
    /// query block height
    #[prost(message, optional, tag = "3")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
}
impl ::prost::Name for QueryConnectionChannelsResponse {
    const NAME: &'static str = "QueryConnectionChannelsResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryChannelClientStateRequest is the request type for the Query/ClientState
/// RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChannelClientStateRequest {
    /// port unique identifier
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryChannelClientStateRequest {
    const NAME: &'static str = "QueryChannelClientStateRequest";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryChannelClientStateResponse is the Response type for the
/// Query/QueryChannelClientState RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChannelClientStateResponse {
    /// client state associated with the channel
    #[prost(message, optional, tag = "1")]
    pub identified_client_state: ::core::option::Option<
        super::super::client::v1::IdentifiedClientState,
    >,
    /// merkle proof of existence
    #[prost(bytes = "vec", tag = "2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
impl ::prost::Name for QueryChannelClientStateResponse {
    const NAME: &'static str = "QueryChannelClientStateResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryChannelConsensusStateRequest is the request type for the
/// Query/ConsensusState RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChannelConsensusStateRequest {
    /// port unique identifier
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    /// revision number of the consensus state
    #[prost(uint64, tag = "3")]
    pub revision_number: u64,
    /// revision height of the consensus state
    #[prost(uint64, tag = "4")]
    pub revision_height: u64,
}
impl ::prost::Name for QueryChannelConsensusStateRequest {
    const NAME: &'static str = "QueryChannelConsensusStateRequest";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryChannelClientStateResponse is the Response type for the
/// Query/QueryChannelClientState RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChannelConsensusStateResponse {
    /// consensus state associated with the channel
    #[prost(message, optional, tag = "1")]
    pub consensus_state: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
    /// client ID associated with the consensus state
    #[prost(string, tag = "2")]
    pub client_id: ::prost::alloc::string::String,
    /// merkle proof of existence
    #[prost(bytes = "vec", tag = "3")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag = "4")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
impl ::prost::Name for QueryChannelConsensusStateResponse {
    const NAME: &'static str = "QueryChannelConsensusStateResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryPacketCommitmentRequest is the request type for the
/// Query/PacketCommitment RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPacketCommitmentRequest {
    /// port unique identifier
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    /// packet sequence
    #[prost(uint64, tag = "3")]
    pub sequence: u64,
}
impl ::prost::Name for QueryPacketCommitmentRequest {
    const NAME: &'static str = "QueryPacketCommitmentRequest";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryPacketCommitmentResponse defines the client query response for a packet
/// which also includes a proof and the height from which the proof was
/// retrieved
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPacketCommitmentResponse {
    /// packet associated with the request fields
    #[prost(bytes = "vec", tag = "1")]
    pub commitment: ::prost::alloc::vec::Vec<u8>,
    /// merkle proof of existence
    #[prost(bytes = "vec", tag = "2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
impl ::prost::Name for QueryPacketCommitmentResponse {
    const NAME: &'static str = "QueryPacketCommitmentResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryPacketCommitmentsRequest is the request type for the
/// Query/QueryPacketCommitments RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPacketCommitmentsRequest {
    /// port unique identifier
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    /// pagination request
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
impl ::prost::Name for QueryPacketCommitmentsRequest {
    const NAME: &'static str = "QueryPacketCommitmentsRequest";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryPacketCommitmentsResponse is the request type for the
/// Query/QueryPacketCommitments RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPacketCommitmentsResponse {
    #[prost(message, repeated, tag = "1")]
    pub commitments: ::prost::alloc::vec::Vec<PacketState>,
    /// pagination response
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
    /// query block height
    #[prost(message, optional, tag = "3")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
}
impl ::prost::Name for QueryPacketCommitmentsResponse {
    const NAME: &'static str = "QueryPacketCommitmentsResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryPacketReceiptRequest is the request type for the
/// Query/PacketReceipt RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPacketReceiptRequest {
    /// port unique identifier
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    /// packet sequence
    #[prost(uint64, tag = "3")]
    pub sequence: u64,
}
impl ::prost::Name for QueryPacketReceiptRequest {
    const NAME: &'static str = "QueryPacketReceiptRequest";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryPacketReceiptResponse defines the client query response for a packet
/// receipt which also includes a proof, and the height from which the proof was
/// retrieved
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPacketReceiptResponse {
    /// success flag for if receipt exists
    #[prost(bool, tag = "2")]
    pub received: bool,
    /// merkle proof of existence
    #[prost(bytes = "vec", tag = "3")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag = "4")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
impl ::prost::Name for QueryPacketReceiptResponse {
    const NAME: &'static str = "QueryPacketReceiptResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryPacketAcknowledgementRequest is the request type for the
/// Query/PacketAcknowledgement RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPacketAcknowledgementRequest {
    /// port unique identifier
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    /// packet sequence
    #[prost(uint64, tag = "3")]
    pub sequence: u64,
}
impl ::prost::Name for QueryPacketAcknowledgementRequest {
    const NAME: &'static str = "QueryPacketAcknowledgementRequest";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryPacketAcknowledgementResponse defines the client query response for a
/// packet which also includes a proof and the height from which the
/// proof was retrieved
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPacketAcknowledgementResponse {
    /// packet associated with the request fields
    #[prost(bytes = "vec", tag = "1")]
    pub acknowledgement: ::prost::alloc::vec::Vec<u8>,
    /// merkle proof of existence
    #[prost(bytes = "vec", tag = "2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
impl ::prost::Name for QueryPacketAcknowledgementResponse {
    const NAME: &'static str = "QueryPacketAcknowledgementResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryPacketAcknowledgementsRequest is the request type for the
/// Query/QueryPacketCommitments RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPacketAcknowledgementsRequest {
    /// port unique identifier
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    /// pagination request
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
    /// list of packet sequences
    #[prost(uint64, repeated, tag = "4")]
    pub packet_commitment_sequences: ::prost::alloc::vec::Vec<u64>,
}
impl ::prost::Name for QueryPacketAcknowledgementsRequest {
    const NAME: &'static str = "QueryPacketAcknowledgementsRequest";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryPacketAcknowledgemetsResponse is the request type for the
/// Query/QueryPacketAcknowledgements RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPacketAcknowledgementsResponse {
    #[prost(message, repeated, tag = "1")]
    pub acknowledgements: ::prost::alloc::vec::Vec<PacketState>,
    /// pagination response
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
    /// query block height
    #[prost(message, optional, tag = "3")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
}
impl ::prost::Name for QueryPacketAcknowledgementsResponse {
    const NAME: &'static str = "QueryPacketAcknowledgementsResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryUnreceivedPacketsRequest is the request type for the
/// Query/UnreceivedPackets RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUnreceivedPacketsRequest {
    /// port unique identifier
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    /// list of packet sequences
    #[prost(uint64, repeated, tag = "3")]
    pub packet_commitment_sequences: ::prost::alloc::vec::Vec<u64>,
}
impl ::prost::Name for QueryUnreceivedPacketsRequest {
    const NAME: &'static str = "QueryUnreceivedPacketsRequest";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryUnreceivedPacketsResponse is the response type for the
/// Query/UnreceivedPacketCommitments RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUnreceivedPacketsResponse {
    /// list of unreceived packet sequences
    #[prost(uint64, repeated, tag = "1")]
    pub sequences: ::prost::alloc::vec::Vec<u64>,
    /// query block height
    #[prost(message, optional, tag = "2")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
}
impl ::prost::Name for QueryUnreceivedPacketsResponse {
    const NAME: &'static str = "QueryUnreceivedPacketsResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryUnreceivedAcks is the request type for the
/// Query/UnreceivedAcks RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUnreceivedAcksRequest {
    /// port unique identifier
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    /// list of acknowledgement sequences
    #[prost(uint64, repeated, tag = "3")]
    pub packet_ack_sequences: ::prost::alloc::vec::Vec<u64>,
}
impl ::prost::Name for QueryUnreceivedAcksRequest {
    const NAME: &'static str = "QueryUnreceivedAcksRequest";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryUnreceivedAcksResponse is the response type for the
/// Query/UnreceivedAcks RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUnreceivedAcksResponse {
    /// list of unreceived acknowledgement sequences
    #[prost(uint64, repeated, tag = "1")]
    pub sequences: ::prost::alloc::vec::Vec<u64>,
    /// query block height
    #[prost(message, optional, tag = "2")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
}
impl ::prost::Name for QueryUnreceivedAcksResponse {
    const NAME: &'static str = "QueryUnreceivedAcksResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryNextSequenceReceiveRequest is the request type for the
/// Query/QueryNextSequenceReceiveRequest RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNextSequenceReceiveRequest {
    /// port unique identifier
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryNextSequenceReceiveRequest {
    const NAME: &'static str = "QueryNextSequenceReceiveRequest";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QuerySequenceResponse is the response type for the
/// Query/QueryNextSequenceReceiveResponse RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNextSequenceReceiveResponse {
    /// next sequence receive number
    #[prost(uint64, tag = "1")]
    pub next_sequence_receive: u64,
    /// merkle proof of existence
    #[prost(bytes = "vec", tag = "2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
impl ::prost::Name for QueryNextSequenceReceiveResponse {
    const NAME: &'static str = "QueryNextSequenceReceiveResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryNextSequenceSendRequest is the request type for the
/// Query/QueryNextSequenceSend RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNextSequenceSendRequest {
    /// port unique identifier
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryNextSequenceSendRequest {
    const NAME: &'static str = "QueryNextSequenceSendRequest";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryNextSequenceSendResponse is the request type for the
/// Query/QueryNextSequenceSend RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNextSequenceSendResponse {
    /// next sequence send number
    #[prost(uint64, tag = "1")]
    pub next_sequence_send: u64,
    /// merkle proof of existence
    #[prost(bytes = "vec", tag = "2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
impl ::prost::Name for QueryNextSequenceSendResponse {
    const NAME: &'static str = "QueryNextSequenceSendResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryUpgradeErrorRequest is the request type for the Query/QueryUpgradeError RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUpgradeErrorRequest {
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryUpgradeErrorRequest {
    const NAME: &'static str = "QueryUpgradeErrorRequest";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryUpgradeErrorResponse is the response type for the Query/QueryUpgradeError RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUpgradeErrorResponse {
    #[prost(message, optional, tag = "1")]
    pub error_receipt: ::core::option::Option<ErrorReceipt>,
    /// merkle proof of existence
    #[prost(bytes = "vec", tag = "2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
impl ::prost::Name for QueryUpgradeErrorResponse {
    const NAME: &'static str = "QueryUpgradeErrorResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryUpgradeRequest is the request type for the QueryUpgradeRequest RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUpgradeRequest {
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryUpgradeRequest {
    const NAME: &'static str = "QueryUpgradeRequest";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryUpgradeResponse is the response type for the QueryUpgradeResponse RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUpgradeResponse {
    #[prost(message, optional, tag = "1")]
    pub upgrade: ::core::option::Option<Upgrade>,
    /// merkle proof of existence
    #[prost(bytes = "vec", tag = "2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
impl ::prost::Name for QueryUpgradeResponse {
    const NAME: &'static str = "QueryUpgradeResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryChannelParamsRequest is the request type for the Query/ChannelParams RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChannelParamsRequest {}
impl ::prost::Name for QueryChannelParamsRequest {
    const NAME: &'static str = "QueryChannelParamsRequest";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// QueryChannelParamsResponse is the response type for the Query/ChannelParams RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChannelParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for QueryChannelParamsResponse {
    const NAME: &'static str = "QueryChannelParamsResponse";
    const PACKAGE: &'static str = "ibc.core.channel.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.channel.v1.{}", Self::NAME)
    }
}
/// Generated client implementations.
#[cfg(feature = "client")]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Query provides defines the gRPC querier service
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Channel queries an IBC Channel.
        pub async fn channel(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryChannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryChannelResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/Channel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.channel.v1.Query", "Channel"));
            self.inner.unary(req, path, codec).await
        }
        /// Channels queries all the IBC channels of a chain.
        pub async fn channels(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryChannelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryChannelsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/Channels",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.channel.v1.Query", "Channels"));
            self.inner.unary(req, path, codec).await
        }
        /// ConnectionChannels queries all the channels associated with a connection
        /// end.
        pub async fn connection_channels(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryConnectionChannelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryConnectionChannelsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/ConnectionChannels",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ibc.core.channel.v1.Query", "ConnectionChannels"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ChannelClientState queries for the client state for the channel associated
        /// with the provided channel identifiers.
        pub async fn channel_client_state(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryChannelClientStateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryChannelClientStateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/ChannelClientState",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ibc.core.channel.v1.Query", "ChannelClientState"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ChannelConsensusState queries for the consensus state for the channel
        /// associated with the provided channel identifiers.
        pub async fn channel_consensus_state(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryChannelConsensusStateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryChannelConsensusStateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/ChannelConsensusState",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ibc.core.channel.v1.Query", "ChannelConsensusState"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// PacketCommitment queries a stored packet commitment hash.
        pub async fn packet_commitment(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPacketCommitmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPacketCommitmentResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/PacketCommitment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ibc.core.channel.v1.Query", "PacketCommitment"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// PacketCommitments returns all the packet commitments hashes associated
        /// with a channel.
        pub async fn packet_commitments(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPacketCommitmentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPacketCommitmentsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/PacketCommitments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ibc.core.channel.v1.Query", "PacketCommitments"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// PacketReceipt queries if a given packet sequence has been received on the
        /// queried chain
        pub async fn packet_receipt(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPacketReceiptRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPacketReceiptResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/PacketReceipt",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.channel.v1.Query", "PacketReceipt"));
            self.inner.unary(req, path, codec).await
        }
        /// PacketAcknowledgement queries a stored packet acknowledgement hash.
        pub async fn packet_acknowledgement(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPacketAcknowledgementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPacketAcknowledgementResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/PacketAcknowledgement",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ibc.core.channel.v1.Query", "PacketAcknowledgement"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// PacketAcknowledgements returns all the packet acknowledgements associated
        /// with a channel.
        pub async fn packet_acknowledgements(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPacketAcknowledgementsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPacketAcknowledgementsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/PacketAcknowledgements",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "ibc.core.channel.v1.Query",
                        "PacketAcknowledgements",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UnreceivedPackets returns all the unreceived IBC packets associated with a
        /// channel and sequences.
        pub async fn unreceived_packets(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryUnreceivedPacketsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryUnreceivedPacketsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/UnreceivedPackets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ibc.core.channel.v1.Query", "UnreceivedPackets"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UnreceivedAcks returns all the unreceived IBC acknowledgements associated
        /// with a channel and sequences.
        pub async fn unreceived_acks(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryUnreceivedAcksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryUnreceivedAcksResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/UnreceivedAcks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.channel.v1.Query", "UnreceivedAcks"));
            self.inner.unary(req, path, codec).await
        }
        /// NextSequenceReceive returns the next receive sequence for a given channel.
        pub async fn next_sequence_receive(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryNextSequenceReceiveRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryNextSequenceReceiveResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/NextSequenceReceive",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ibc.core.channel.v1.Query", "NextSequenceReceive"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// NextSequenceSend returns the next send sequence for a given channel.
        pub async fn next_sequence_send(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryNextSequenceSendRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryNextSequenceSendResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/NextSequenceSend",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ibc.core.channel.v1.Query", "NextSequenceSend"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpgradeError returns the error receipt if the upgrade handshake failed.
        pub async fn upgrade_error(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryUpgradeErrorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryUpgradeErrorResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/UpgradeError",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.channel.v1.Query", "UpgradeError"));
            self.inner.unary(req, path, codec).await
        }
        /// Upgrade returns the upgrade for a given port and channel id.
        pub async fn upgrade(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryUpgradeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryUpgradeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/Upgrade",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.channel.v1.Query", "Upgrade"));
            self.inner.unary(req, path, codec).await
        }
        /// ChannelParams queries all parameters of the ibc channel submodule.
        pub async fn channel_params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryChannelParamsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryChannelParamsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/ChannelParams",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.channel.v1.Query", "ChannelParams"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "server")]
pub mod query_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QueryServer.
    #[async_trait]
    pub trait Query: Send + Sync + 'static {
        /// Channel queries an IBC Channel.
        async fn channel(
            &self,
            request: tonic::Request<super::QueryChannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryChannelResponse>,
            tonic::Status,
        >;
        /// Channels queries all the IBC channels of a chain.
        async fn channels(
            &self,
            request: tonic::Request<super::QueryChannelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryChannelsResponse>,
            tonic::Status,
        >;
        /// ConnectionChannels queries all the channels associated with a connection
        /// end.
        async fn connection_channels(
            &self,
            request: tonic::Request<super::QueryConnectionChannelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryConnectionChannelsResponse>,
            tonic::Status,
        >;
        /// ChannelClientState queries for the client state for the channel associated
        /// with the provided channel identifiers.
        async fn channel_client_state(
            &self,
            request: tonic::Request<super::QueryChannelClientStateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryChannelClientStateResponse>,
            tonic::Status,
        >;
        /// ChannelConsensusState queries for the consensus state for the channel
        /// associated with the provided channel identifiers.
        async fn channel_consensus_state(
            &self,
            request: tonic::Request<super::QueryChannelConsensusStateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryChannelConsensusStateResponse>,
            tonic::Status,
        >;
        /// PacketCommitment queries a stored packet commitment hash.
        async fn packet_commitment(
            &self,
            request: tonic::Request<super::QueryPacketCommitmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPacketCommitmentResponse>,
            tonic::Status,
        >;
        /// PacketCommitments returns all the packet commitments hashes associated
        /// with a channel.
        async fn packet_commitments(
            &self,
            request: tonic::Request<super::QueryPacketCommitmentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPacketCommitmentsResponse>,
            tonic::Status,
        >;
        /// PacketReceipt queries if a given packet sequence has been received on the
        /// queried chain
        async fn packet_receipt(
            &self,
            request: tonic::Request<super::QueryPacketReceiptRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPacketReceiptResponse>,
            tonic::Status,
        >;
        /// PacketAcknowledgement queries a stored packet acknowledgement hash.
        async fn packet_acknowledgement(
            &self,
            request: tonic::Request<super::QueryPacketAcknowledgementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPacketAcknowledgementResponse>,
            tonic::Status,
        >;
        /// PacketAcknowledgements returns all the packet acknowledgements associated
        /// with a channel.
        async fn packet_acknowledgements(
            &self,
            request: tonic::Request<super::QueryPacketAcknowledgementsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPacketAcknowledgementsResponse>,
            tonic::Status,
        >;
        /// UnreceivedPackets returns all the unreceived IBC packets associated with a
        /// channel and sequences.
        async fn unreceived_packets(
            &self,
            request: tonic::Request<super::QueryUnreceivedPacketsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryUnreceivedPacketsResponse>,
            tonic::Status,
        >;
        /// UnreceivedAcks returns all the unreceived IBC acknowledgements associated
        /// with a channel and sequences.
        async fn unreceived_acks(
            &self,
            request: tonic::Request<super::QueryUnreceivedAcksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryUnreceivedAcksResponse>,
            tonic::Status,
        >;
        /// NextSequenceReceive returns the next receive sequence for a given channel.
        async fn next_sequence_receive(
            &self,
            request: tonic::Request<super::QueryNextSequenceReceiveRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryNextSequenceReceiveResponse>,
            tonic::Status,
        >;
        /// NextSequenceSend returns the next send sequence for a given channel.
        async fn next_sequence_send(
            &self,
            request: tonic::Request<super::QueryNextSequenceSendRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryNextSequenceSendResponse>,
            tonic::Status,
        >;
        /// UpgradeError returns the error receipt if the upgrade handshake failed.
        async fn upgrade_error(
            &self,
            request: tonic::Request<super::QueryUpgradeErrorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryUpgradeErrorResponse>,
            tonic::Status,
        >;
        /// Upgrade returns the upgrade for a given port and channel id.
        async fn upgrade(
            &self,
            request: tonic::Request<super::QueryUpgradeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryUpgradeResponse>,
            tonic::Status,
        >;
        /// ChannelParams queries all parameters of the ibc channel submodule.
        async fn channel_params(
            &self,
            request: tonic::Request<super::QueryChannelParamsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryChannelParamsResponse>,
            tonic::Status,
        >;
    }
    /// Query provides defines the gRPC querier service
    #[derive(Debug)]
    pub struct QueryServer<T: Query> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Query> QueryServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for QueryServer<T>
    where
        T: Query,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/ibc.core.channel.v1.Query/Channel" => {
                    #[allow(non_camel_case_types)]
                    struct ChannelSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryChannelRequest>
                    for ChannelSvc<T> {
                        type Response = super::QueryChannelResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryChannelRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::channel(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChannelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Query/Channels" => {
                    #[allow(non_camel_case_types)]
                    struct ChannelsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryChannelsRequest>
                    for ChannelsSvc<T> {
                        type Response = super::QueryChannelsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryChannelsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::channels(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChannelsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Query/ConnectionChannels" => {
                    #[allow(non_camel_case_types)]
                    struct ConnectionChannelsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryConnectionChannelsRequest>
                    for ConnectionChannelsSvc<T> {
                        type Response = super::QueryConnectionChannelsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryConnectionChannelsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::connection_channels(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ConnectionChannelsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Query/ChannelClientState" => {
                    #[allow(non_camel_case_types)]
                    struct ChannelClientStateSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryChannelClientStateRequest>
                    for ChannelClientStateSvc<T> {
                        type Response = super::QueryChannelClientStateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryChannelClientStateRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::channel_client_state(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChannelClientStateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Query/ChannelConsensusState" => {
                    #[allow(non_camel_case_types)]
                    struct ChannelConsensusStateSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryChannelConsensusStateRequest,
                    > for ChannelConsensusStateSvc<T> {
                        type Response = super::QueryChannelConsensusStateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryChannelConsensusStateRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::channel_consensus_state(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChannelConsensusStateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Query/PacketCommitment" => {
                    #[allow(non_camel_case_types)]
                    struct PacketCommitmentSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryPacketCommitmentRequest>
                    for PacketCommitmentSvc<T> {
                        type Response = super::QueryPacketCommitmentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryPacketCommitmentRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::packet_commitment(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PacketCommitmentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Query/PacketCommitments" => {
                    #[allow(non_camel_case_types)]
                    struct PacketCommitmentsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryPacketCommitmentsRequest>
                    for PacketCommitmentsSvc<T> {
                        type Response = super::QueryPacketCommitmentsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryPacketCommitmentsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::packet_commitments(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PacketCommitmentsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Query/PacketReceipt" => {
                    #[allow(non_camel_case_types)]
                    struct PacketReceiptSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryPacketReceiptRequest>
                    for PacketReceiptSvc<T> {
                        type Response = super::QueryPacketReceiptResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryPacketReceiptRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::packet_receipt(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PacketReceiptSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Query/PacketAcknowledgement" => {
                    #[allow(non_camel_case_types)]
                    struct PacketAcknowledgementSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryPacketAcknowledgementRequest,
                    > for PacketAcknowledgementSvc<T> {
                        type Response = super::QueryPacketAcknowledgementResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryPacketAcknowledgementRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::packet_acknowledgement(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PacketAcknowledgementSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Query/PacketAcknowledgements" => {
                    #[allow(non_camel_case_types)]
                    struct PacketAcknowledgementsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryPacketAcknowledgementsRequest,
                    > for PacketAcknowledgementsSvc<T> {
                        type Response = super::QueryPacketAcknowledgementsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryPacketAcknowledgementsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::packet_acknowledgements(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PacketAcknowledgementsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Query/UnreceivedPackets" => {
                    #[allow(non_camel_case_types)]
                    struct UnreceivedPacketsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryUnreceivedPacketsRequest>
                    for UnreceivedPacketsSvc<T> {
                        type Response = super::QueryUnreceivedPacketsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryUnreceivedPacketsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::unreceived_packets(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnreceivedPacketsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Query/UnreceivedAcks" => {
                    #[allow(non_camel_case_types)]
                    struct UnreceivedAcksSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryUnreceivedAcksRequest>
                    for UnreceivedAcksSvc<T> {
                        type Response = super::QueryUnreceivedAcksResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryUnreceivedAcksRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::unreceived_acks(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnreceivedAcksSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Query/NextSequenceReceive" => {
                    #[allow(non_camel_case_types)]
                    struct NextSequenceReceiveSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryNextSequenceReceiveRequest>
                    for NextSequenceReceiveSvc<T> {
                        type Response = super::QueryNextSequenceReceiveResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryNextSequenceReceiveRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::next_sequence_receive(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NextSequenceReceiveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Query/NextSequenceSend" => {
                    #[allow(non_camel_case_types)]
                    struct NextSequenceSendSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryNextSequenceSendRequest>
                    for NextSequenceSendSvc<T> {
                        type Response = super::QueryNextSequenceSendResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryNextSequenceSendRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::next_sequence_send(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NextSequenceSendSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Query/UpgradeError" => {
                    #[allow(non_camel_case_types)]
                    struct UpgradeErrorSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryUpgradeErrorRequest>
                    for UpgradeErrorSvc<T> {
                        type Response = super::QueryUpgradeErrorResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryUpgradeErrorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::upgrade_error(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpgradeErrorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Query/Upgrade" => {
                    #[allow(non_camel_case_types)]
                    struct UpgradeSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryUpgradeRequest>
                    for UpgradeSvc<T> {
                        type Response = super::QueryUpgradeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryUpgradeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::upgrade(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpgradeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.channel.v1.Query/ChannelParams" => {
                    #[allow(non_camel_case_types)]
                    struct ChannelParamsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryChannelParamsRequest>
                    for ChannelParamsSvc<T> {
                        type Response = super::QueryChannelParamsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryChannelParamsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::channel_params(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChannelParamsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Query> Clone for QueryServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Query> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Query> tonic::server::NamedService for QueryServer<T> {
        const NAME: &'static str = "ibc.core.channel.v1.Query";
    }
}

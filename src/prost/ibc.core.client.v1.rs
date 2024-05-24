/// IdentifiedClientState defines a client state with an additional client
/// identifier field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentifiedClientState {
    /// client identifier
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// client state
    #[prost(message, optional, tag = "2")]
    pub client_state: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
}
impl ::prost::Name for IdentifiedClientState {
    const NAME: &'static str = "IdentifiedClientState";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// ConsensusStateWithHeight defines a consensus state with an additional height
/// field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusStateWithHeight {
    /// consensus state height
    #[prost(message, optional, tag = "1")]
    pub height: ::core::option::Option<Height>,
    /// consensus state
    #[prost(message, optional, tag = "2")]
    pub consensus_state: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
}
impl ::prost::Name for ConsensusStateWithHeight {
    const NAME: &'static str = "ConsensusStateWithHeight";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// ClientConsensusStates defines all the stored consensus states for a given
/// client.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientConsensusStates {
    /// client identifier
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// consensus states and their heights associated with the client
    #[prost(message, repeated, tag = "2")]
    pub consensus_states: ::prost::alloc::vec::Vec<ConsensusStateWithHeight>,
}
impl ::prost::Name for ClientConsensusStates {
    const NAME: &'static str = "ClientConsensusStates";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// Height is a monotonically increasing data type
/// that can be compared against another Height for the purposes of updating and
/// freezing clients
///
/// Normally the RevisionHeight is incremented at each height while keeping
/// RevisionNumber the same. However some consensus algorithms may choose to
/// reset the height in certain conditions e.g. hard forks, state-machine
/// breaking changes In these cases, the RevisionNumber is incremented so that
/// height continues to be monitonically increasing even as the RevisionHeight
/// gets reset
#[derive(Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "json-schema", feature = "serde"),
    derive(::schemars::JsonSchema)
)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Height {
    /// the revision that the client is currently on
    #[prost(uint64, tag = "1")]
    pub revision_number: u64,
    /// the height within the given revision
    #[prost(uint64, tag = "2")]
    pub revision_height: u64,
}
impl ::prost::Name for Height {
    const NAME: &'static str = "Height";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// Params defines the set of IBC light client parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// allowed_clients defines the list of allowed client state types which can be created
    /// and interacted with. If a client type is removed from the allowed clients list, usage
    /// of this client will be disabled until it is added again to the list.
    #[prost(string, repeated, tag = "1")]
    pub allowed_clients: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for Params {
    const NAME: &'static str = "Params";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// ClientUpdateProposal is a legacy governance proposal. If it passes, the substitute
/// client's latest consensus state is copied over to the subject client. The proposal
/// handler may fail if the subject and the substitute do not match in client and
/// chain parameters (with exception to latest height, frozen height, and chain-id).
///
/// Deprecated: Please use MsgRecoverClient in favour of this message type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientUpdateProposal {
    /// the title of the update proposal
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// the description of the proposal
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// the client identifier for the client to be updated if the proposal passes
    #[prost(string, tag = "3")]
    pub subject_client_id: ::prost::alloc::string::String,
    /// the substitute client identifier for the client standing in for the subject
    /// client
    #[prost(string, tag = "4")]
    pub substitute_client_id: ::prost::alloc::string::String,
}
impl ::prost::Name for ClientUpdateProposal {
    const NAME: &'static str = "ClientUpdateProposal";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// UpgradeProposal is a gov Content type for initiating an IBC breaking
/// upgrade.
///
/// Deprecated: Please use MsgIBCSoftwareUpgrade in favour of this message type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub plan: ::core::option::Option<
        super::super::super::super::cosmos::upgrade::v1beta1::Plan,
    >,
    /// An UpgradedClientState must be provided to perform an IBC breaking upgrade.
    /// This will make the chain commit to the correct upgraded (self) client state
    /// before the upgrade occurs, so that connecting chains can verify that the
    /// new upgraded client is valid by verifying a proof on the previous version
    /// of the chain. This will allow IBC connections to persist smoothly across
    /// planned chain upgrades
    #[prost(message, optional, tag = "4")]
    pub upgraded_client_state: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
}
impl ::prost::Name for UpgradeProposal {
    const NAME: &'static str = "UpgradeProposal";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// GenesisState defines the ibc client submodule's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// client states with their corresponding identifiers
    #[prost(message, repeated, tag = "1")]
    pub clients: ::prost::alloc::vec::Vec<IdentifiedClientState>,
    /// consensus states from each client
    #[prost(message, repeated, tag = "2")]
    pub clients_consensus: ::prost::alloc::vec::Vec<ClientConsensusStates>,
    /// metadata from each client
    #[prost(message, repeated, tag = "3")]
    pub clients_metadata: ::prost::alloc::vec::Vec<IdentifiedGenesisMetadata>,
    #[prost(message, optional, tag = "4")]
    pub params: ::core::option::Option<Params>,
    /// Deprecated: create_localhost has been deprecated.
    /// The localhost client is automatically created at genesis.
    #[deprecated]
    #[prost(bool, tag = "5")]
    pub create_localhost: bool,
    /// the sequence for the next generated client identifier
    #[prost(uint64, tag = "6")]
    pub next_client_sequence: u64,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// GenesisMetadata defines the genesis type for metadata that clients may return
/// with ExportMetadata
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisMetadata {
    /// store key of metadata without clientID-prefix
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// metadata value
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for GenesisMetadata {
    const NAME: &'static str = "GenesisMetadata";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// IdentifiedGenesisMetadata has the client metadata with the corresponding
/// client id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentifiedGenesisMetadata {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub client_metadata: ::prost::alloc::vec::Vec<GenesisMetadata>,
}
impl ::prost::Name for IdentifiedGenesisMetadata {
    const NAME: &'static str = "IdentifiedGenesisMetadata";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// MsgCreateClient defines a message to create an IBC client
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateClient {
    /// light client state
    #[prost(message, optional, tag = "1")]
    pub client_state: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
    /// consensus state associated with the client that corresponds to a given
    /// height.
    #[prost(message, optional, tag = "2")]
    pub consensus_state: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
    /// signer address
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateClient {
    const NAME: &'static str = "MsgCreateClient";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// MsgCreateClientResponse defines the Msg/CreateClient response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateClientResponse {}
impl ::prost::Name for MsgCreateClientResponse {
    const NAME: &'static str = "MsgCreateClientResponse";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// MsgUpdateClient defines an sdk.Msg to update a IBC client state using
/// the given client message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateClient {
    /// client unique identifier
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// client message to update the light client
    #[prost(message, optional, tag = "2")]
    pub client_message: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
    /// signer address
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgUpdateClient {
    const NAME: &'static str = "MsgUpdateClient";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// MsgUpdateClientResponse defines the Msg/UpdateClient response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateClientResponse {}
impl ::prost::Name for MsgUpdateClientResponse {
    const NAME: &'static str = "MsgUpdateClientResponse";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// MsgUpgradeClient defines an sdk.Msg to upgrade an IBC client to a new client
/// state
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpgradeClient {
    /// client unique identifier
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// upgraded client state
    #[prost(message, optional, tag = "2")]
    pub client_state: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
    /// upgraded consensus state, only contains enough information to serve as a
    /// basis of trust in update logic
    #[prost(message, optional, tag = "3")]
    pub consensus_state: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
    /// proof that old chain committed to new client
    #[prost(bytes = "vec", tag = "4")]
    pub proof_upgrade_client: ::prost::alloc::vec::Vec<u8>,
    /// proof that old chain committed to new consensus state
    #[prost(bytes = "vec", tag = "5")]
    pub proof_upgrade_consensus_state: ::prost::alloc::vec::Vec<u8>,
    /// signer address
    #[prost(string, tag = "6")]
    pub signer: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgUpgradeClient {
    const NAME: &'static str = "MsgUpgradeClient";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// MsgUpgradeClientResponse defines the Msg/UpgradeClient response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpgradeClientResponse {}
impl ::prost::Name for MsgUpgradeClientResponse {
    const NAME: &'static str = "MsgUpgradeClientResponse";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// MsgSubmitMisbehaviour defines an sdk.Msg type that submits Evidence for
/// light client misbehaviour.
/// This message has been deprecated. Use MsgUpdateClient instead.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitMisbehaviour {
    /// client unique identifier
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// misbehaviour used for freezing the light client
    #[prost(message, optional, tag = "2")]
    pub misbehaviour: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
    /// signer address
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgSubmitMisbehaviour {
    const NAME: &'static str = "MsgSubmitMisbehaviour";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// MsgSubmitMisbehaviourResponse defines the Msg/SubmitMisbehaviour response
/// type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitMisbehaviourResponse {}
impl ::prost::Name for MsgSubmitMisbehaviourResponse {
    const NAME: &'static str = "MsgSubmitMisbehaviourResponse";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// MsgRecoverClient defines the message used to recover a frozen or expired client.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRecoverClient {
    /// the client identifier for the client to be updated if the proposal passes
    #[prost(string, tag = "1")]
    pub subject_client_id: ::prost::alloc::string::String,
    /// the substitute client identifier for the client which will replace the subject
    /// client
    #[prost(string, tag = "2")]
    pub substitute_client_id: ::prost::alloc::string::String,
    /// signer address
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgRecoverClient {
    const NAME: &'static str = "MsgRecoverClient";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// MsgRecoverClientResponse defines the Msg/RecoverClient response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRecoverClientResponse {}
impl ::prost::Name for MsgRecoverClientResponse {
    const NAME: &'static str = "MsgRecoverClientResponse";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// MsgIBCSoftwareUpgrade defines the message used to schedule an upgrade of an IBC client using a v1 governance proposal
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIbcSoftwareUpgrade {
    #[prost(message, optional, tag = "1")]
    pub plan: ::core::option::Option<
        super::super::super::super::cosmos::upgrade::v1beta1::Plan,
    >,
    /// An UpgradedClientState must be provided to perform an IBC breaking upgrade.
    /// This will make the chain commit to the correct upgraded (self) client state
    /// before the upgrade occurs, so that connecting chains can verify that the
    /// new upgraded client is valid by verifying a proof on the previous version
    /// of the chain. This will allow IBC connections to persist smoothly across
    /// planned chain upgrades. Correspondingly, the UpgradedClientState field has been
    /// deprecated in the Cosmos SDK to allow for this logic to exist solely in
    /// the 02-client module.
    #[prost(message, optional, tag = "2")]
    pub upgraded_client_state: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
    /// signer address
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgIbcSoftwareUpgrade {
    const NAME: &'static str = "MsgIBCSoftwareUpgrade";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// MsgIBCSoftwareUpgradeResponse defines the Msg/IBCSoftwareUpgrade response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIbcSoftwareUpgradeResponse {}
impl ::prost::Name for MsgIbcSoftwareUpgradeResponse {
    const NAME: &'static str = "MsgIBCSoftwareUpgradeResponse";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// MsgUpdateParams defines the sdk.Msg type to update the client parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// signer address
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    /// params defines the client parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for MsgUpdateParams {
    const NAME: &'static str = "MsgUpdateParams";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// MsgUpdateParamsResponse defines the MsgUpdateParams response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
impl ::prost::Name for MsgUpdateParamsResponse {
    const NAME: &'static str = "MsgUpdateParamsResponse";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// Generated client implementations.
#[cfg(feature = "client")]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Msg defines the ibc/client Msg service.
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
        /// CreateClient defines a rpc handler method for MsgCreateClient.
        pub async fn create_client(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateClient>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreateClientResponse>,
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
                "/ibc.core.client.v1.Msg/CreateClient",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.client.v1.Msg", "CreateClient"));
            self.inner.unary(req, path, codec).await
        }
        /// UpdateClient defines a rpc handler method for MsgUpdateClient.
        pub async fn update_client(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateClient>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateClientResponse>,
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
                "/ibc.core.client.v1.Msg/UpdateClient",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.client.v1.Msg", "UpdateClient"));
            self.inner.unary(req, path, codec).await
        }
        /// UpgradeClient defines a rpc handler method for MsgUpgradeClient.
        pub async fn upgrade_client(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpgradeClient>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpgradeClientResponse>,
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
                "/ibc.core.client.v1.Msg/UpgradeClient",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.client.v1.Msg", "UpgradeClient"));
            self.inner.unary(req, path, codec).await
        }
        /// SubmitMisbehaviour defines a rpc handler method for MsgSubmitMisbehaviour.
        pub async fn submit_misbehaviour(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSubmitMisbehaviour>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSubmitMisbehaviourResponse>,
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
                "/ibc.core.client.v1.Msg/SubmitMisbehaviour",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.client.v1.Msg", "SubmitMisbehaviour"));
            self.inner.unary(req, path, codec).await
        }
        /// RecoverClient defines a rpc handler method for MsgRecoverClient.
        pub async fn recover_client(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgRecoverClient>,
        ) -> std::result::Result<
            tonic::Response<super::MsgRecoverClientResponse>,
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
                "/ibc.core.client.v1.Msg/RecoverClient",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.client.v1.Msg", "RecoverClient"));
            self.inner.unary(req, path, codec).await
        }
        /// IBCSoftwareUpgrade defines a rpc handler method for MsgIBCSoftwareUpgrade.
        pub async fn ibc_software_upgrade(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgIbcSoftwareUpgrade>,
        ) -> std::result::Result<
            tonic::Response<super::MsgIbcSoftwareUpgradeResponse>,
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
                "/ibc.core.client.v1.Msg/IBCSoftwareUpgrade",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.client.v1.Msg", "IBCSoftwareUpgrade"));
            self.inner.unary(req, path, codec).await
        }
        /// UpdateClientParams defines a rpc handler method for MsgUpdateParams.
        pub async fn update_client_params(
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
                "/ibc.core.client.v1.Msg/UpdateClientParams",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.client.v1.Msg", "UpdateClientParams"));
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
        /// CreateClient defines a rpc handler method for MsgCreateClient.
        async fn create_client(
            &self,
            request: tonic::Request<super::MsgCreateClient>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreateClientResponse>,
            tonic::Status,
        >;
        /// UpdateClient defines a rpc handler method for MsgUpdateClient.
        async fn update_client(
            &self,
            request: tonic::Request<super::MsgUpdateClient>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateClientResponse>,
            tonic::Status,
        >;
        /// UpgradeClient defines a rpc handler method for MsgUpgradeClient.
        async fn upgrade_client(
            &self,
            request: tonic::Request<super::MsgUpgradeClient>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpgradeClientResponse>,
            tonic::Status,
        >;
        /// SubmitMisbehaviour defines a rpc handler method for MsgSubmitMisbehaviour.
        async fn submit_misbehaviour(
            &self,
            request: tonic::Request<super::MsgSubmitMisbehaviour>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSubmitMisbehaviourResponse>,
            tonic::Status,
        >;
        /// RecoverClient defines a rpc handler method for MsgRecoverClient.
        async fn recover_client(
            &self,
            request: tonic::Request<super::MsgRecoverClient>,
        ) -> std::result::Result<
            tonic::Response<super::MsgRecoverClientResponse>,
            tonic::Status,
        >;
        /// IBCSoftwareUpgrade defines a rpc handler method for MsgIBCSoftwareUpgrade.
        async fn ibc_software_upgrade(
            &self,
            request: tonic::Request<super::MsgIbcSoftwareUpgrade>,
        ) -> std::result::Result<
            tonic::Response<super::MsgIbcSoftwareUpgradeResponse>,
            tonic::Status,
        >;
        /// UpdateClientParams defines a rpc handler method for MsgUpdateParams.
        async fn update_client_params(
            &self,
            request: tonic::Request<super::MsgUpdateParams>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateParamsResponse>,
            tonic::Status,
        >;
    }
    /// Msg defines the ibc/client Msg service.
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
                "/ibc.core.client.v1.Msg/CreateClient" => {
                    #[allow(non_camel_case_types)]
                    struct CreateClientSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCreateClient>
                    for CreateClientSvc<T> {
                        type Response = super::MsgCreateClientResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCreateClient>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::create_client(&inner, request).await
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
                        let method = CreateClientSvc(inner);
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
                "/ibc.core.client.v1.Msg/UpdateClient" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateClientSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUpdateClient>
                    for UpdateClientSvc<T> {
                        type Response = super::MsgUpdateClientResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpdateClient>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::update_client(&inner, request).await
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
                        let method = UpdateClientSvc(inner);
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
                "/ibc.core.client.v1.Msg/UpgradeClient" => {
                    #[allow(non_camel_case_types)]
                    struct UpgradeClientSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUpgradeClient>
                    for UpgradeClientSvc<T> {
                        type Response = super::MsgUpgradeClientResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpgradeClient>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::upgrade_client(&inner, request).await
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
                        let method = UpgradeClientSvc(inner);
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
                "/ibc.core.client.v1.Msg/SubmitMisbehaviour" => {
                    #[allow(non_camel_case_types)]
                    struct SubmitMisbehaviourSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgSubmitMisbehaviour>
                    for SubmitMisbehaviourSvc<T> {
                        type Response = super::MsgSubmitMisbehaviourResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSubmitMisbehaviour>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::submit_misbehaviour(&inner, request).await
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
                        let method = SubmitMisbehaviourSvc(inner);
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
                "/ibc.core.client.v1.Msg/RecoverClient" => {
                    #[allow(non_camel_case_types)]
                    struct RecoverClientSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgRecoverClient>
                    for RecoverClientSvc<T> {
                        type Response = super::MsgRecoverClientResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgRecoverClient>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::recover_client(&inner, request).await
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
                        let method = RecoverClientSvc(inner);
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
                "/ibc.core.client.v1.Msg/IBCSoftwareUpgrade" => {
                    #[allow(non_camel_case_types)]
                    struct IBCSoftwareUpgradeSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgIbcSoftwareUpgrade>
                    for IBCSoftwareUpgradeSvc<T> {
                        type Response = super::MsgIbcSoftwareUpgradeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgIbcSoftwareUpgrade>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::ibc_software_upgrade(&inner, request).await
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
                        let method = IBCSoftwareUpgradeSvc(inner);
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
                "/ibc.core.client.v1.Msg/UpdateClientParams" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateClientParamsSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUpdateParams>
                    for UpdateClientParamsSvc<T> {
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
                                <T as Msg>::update_client_params(&inner, request).await
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
                        let method = UpdateClientParamsSvc(inner);
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
        const NAME: &'static str = "ibc.core.client.v1.Msg";
    }
}
/// QueryClientStateRequest is the request type for the Query/ClientState RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientStateRequest {
    /// client state unique identifier
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryClientStateRequest {
    const NAME: &'static str = "QueryClientStateRequest";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// QueryClientStateResponse is the response type for the Query/ClientState RPC
/// method. Besides the client state, it includes a proof and the height from
/// which the proof was retrieved.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientStateResponse {
    /// client state associated with the request identifier
    #[prost(message, optional, tag = "1")]
    pub client_state: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
    /// merkle proof of existence
    #[prost(bytes = "vec", tag = "2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<Height>,
}
impl ::prost::Name for QueryClientStateResponse {
    const NAME: &'static str = "QueryClientStateResponse";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// QueryClientStatesRequest is the request type for the Query/ClientStates RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientStatesRequest {
    /// pagination request
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
impl ::prost::Name for QueryClientStatesRequest {
    const NAME: &'static str = "QueryClientStatesRequest";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// QueryClientStatesResponse is the response type for the Query/ClientStates RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientStatesResponse {
    /// list of stored ClientStates of the chain.
    #[prost(message, repeated, tag = "1")]
    pub client_states: ::prost::alloc::vec::Vec<IdentifiedClientState>,
    /// pagination response
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
}
impl ::prost::Name for QueryClientStatesResponse {
    const NAME: &'static str = "QueryClientStatesResponse";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// QueryConsensusStateRequest is the request type for the Query/ConsensusState
/// RPC method. Besides the consensus state, it includes a proof and the height
/// from which the proof was retrieved.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConsensusStateRequest {
    /// client identifier
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// consensus state revision number
    #[prost(uint64, tag = "2")]
    pub revision_number: u64,
    /// consensus state revision height
    #[prost(uint64, tag = "3")]
    pub revision_height: u64,
    /// latest_height overrrides the height field and queries the latest stored
    /// ConsensusState
    #[prost(bool, tag = "4")]
    pub latest_height: bool,
}
impl ::prost::Name for QueryConsensusStateRequest {
    const NAME: &'static str = "QueryConsensusStateRequest";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// QueryConsensusStateResponse is the response type for the Query/ConsensusState
/// RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConsensusStateResponse {
    /// consensus state associated with the client identifier at the given height
    #[prost(message, optional, tag = "1")]
    pub consensus_state: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
    /// merkle proof of existence
    #[prost(bytes = "vec", tag = "2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<Height>,
}
impl ::prost::Name for QueryConsensusStateResponse {
    const NAME: &'static str = "QueryConsensusStateResponse";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// QueryConsensusStatesRequest is the request type for the Query/ConsensusStates
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConsensusStatesRequest {
    /// client identifier
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// pagination request
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
impl ::prost::Name for QueryConsensusStatesRequest {
    const NAME: &'static str = "QueryConsensusStatesRequest";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// QueryConsensusStatesResponse is the response type for the
/// Query/ConsensusStates RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConsensusStatesResponse {
    /// consensus states associated with the identifier
    #[prost(message, repeated, tag = "1")]
    pub consensus_states: ::prost::alloc::vec::Vec<ConsensusStateWithHeight>,
    /// pagination response
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
}
impl ::prost::Name for QueryConsensusStatesResponse {
    const NAME: &'static str = "QueryConsensusStatesResponse";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// QueryConsensusStateHeightsRequest is the request type for Query/ConsensusStateHeights
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConsensusStateHeightsRequest {
    /// client identifier
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// pagination request
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
impl ::prost::Name for QueryConsensusStateHeightsRequest {
    const NAME: &'static str = "QueryConsensusStateHeightsRequest";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// QueryConsensusStateHeightsResponse is the response type for the
/// Query/ConsensusStateHeights RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConsensusStateHeightsResponse {
    /// consensus state heights
    #[prost(message, repeated, tag = "1")]
    pub consensus_state_heights: ::prost::alloc::vec::Vec<Height>,
    /// pagination response
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
}
impl ::prost::Name for QueryConsensusStateHeightsResponse {
    const NAME: &'static str = "QueryConsensusStateHeightsResponse";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// QueryClientStatusRequest is the request type for the Query/ClientStatus RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientStatusRequest {
    /// client unique identifier
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryClientStatusRequest {
    const NAME: &'static str = "QueryClientStatusRequest";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// QueryClientStatusResponse is the response type for the Query/ClientStatus RPC
/// method. It returns the current status of the IBC client.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientStatusResponse {
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryClientStatusResponse {
    const NAME: &'static str = "QueryClientStatusResponse";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// QueryClientParamsRequest is the request type for the Query/ClientParams RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientParamsRequest {}
impl ::prost::Name for QueryClientParamsRequest {
    const NAME: &'static str = "QueryClientParamsRequest";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// QueryClientParamsResponse is the response type for the Query/ClientParams RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for QueryClientParamsResponse {
    const NAME: &'static str = "QueryClientParamsResponse";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// QueryUpgradedClientStateRequest is the request type for the
/// Query/UpgradedClientState RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUpgradedClientStateRequest {}
impl ::prost::Name for QueryUpgradedClientStateRequest {
    const NAME: &'static str = "QueryUpgradedClientStateRequest";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// QueryUpgradedClientStateResponse is the response type for the
/// Query/UpgradedClientState RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUpgradedClientStateResponse {
    /// client state associated with the request identifier
    #[prost(message, optional, tag = "1")]
    pub upgraded_client_state: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
}
impl ::prost::Name for QueryUpgradedClientStateResponse {
    const NAME: &'static str = "QueryUpgradedClientStateResponse";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// QueryUpgradedConsensusStateRequest is the request type for the
/// Query/UpgradedConsensusState RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUpgradedConsensusStateRequest {}
impl ::prost::Name for QueryUpgradedConsensusStateRequest {
    const NAME: &'static str = "QueryUpgradedConsensusStateRequest";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// QueryUpgradedConsensusStateResponse is the response type for the
/// Query/UpgradedConsensusState RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUpgradedConsensusStateResponse {
    /// Consensus state associated with the request identifier
    #[prost(message, optional, tag = "1")]
    pub upgraded_consensus_state: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
}
impl ::prost::Name for QueryUpgradedConsensusStateResponse {
    const NAME: &'static str = "QueryUpgradedConsensusStateResponse";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// QueryVerifyMembershipRequest is the request type for the Query/VerifyMembership RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVerifyMembershipRequest {
    /// client unique identifier.
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// the proof to be verified by the client.
    #[prost(bytes = "vec", tag = "2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// the height of the commitment root at which the proof is verified.
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<Height>,
    /// the commitment key path.
    #[prost(message, optional, tag = "4")]
    pub merkle_path: ::core::option::Option<super::super::commitment::v1::MerklePath>,
    /// the value which is proven.
    #[prost(bytes = "vec", tag = "5")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// optional time delay
    #[prost(uint64, tag = "6")]
    pub time_delay: u64,
    /// optional block delay
    #[prost(uint64, tag = "7")]
    pub block_delay: u64,
}
impl ::prost::Name for QueryVerifyMembershipRequest {
    const NAME: &'static str = "QueryVerifyMembershipRequest";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
    }
}
/// QueryVerifyMembershipResponse is the response type for the Query/VerifyMembership RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVerifyMembershipResponse {
    /// boolean indicating success or failure of proof verification.
    #[prost(bool, tag = "1")]
    pub success: bool,
}
impl ::prost::Name for QueryVerifyMembershipResponse {
    const NAME: &'static str = "QueryVerifyMembershipResponse";
    const PACKAGE: &'static str = "ibc.core.client.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
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
        /// ClientState queries an IBC light client.
        pub async fn client_state(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryClientStateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryClientStateResponse>,
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
                "/ibc.core.client.v1.Query/ClientState",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.client.v1.Query", "ClientState"));
            self.inner.unary(req, path, codec).await
        }
        /// ClientStates queries all the IBC light clients of a chain.
        pub async fn client_states(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryClientStatesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryClientStatesResponse>,
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
                "/ibc.core.client.v1.Query/ClientStates",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.client.v1.Query", "ClientStates"));
            self.inner.unary(req, path, codec).await
        }
        /// ConsensusState queries a consensus state associated with a client state at
        /// a given height.
        pub async fn consensus_state(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryConsensusStateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryConsensusStateResponse>,
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
                "/ibc.core.client.v1.Query/ConsensusState",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.client.v1.Query", "ConsensusState"));
            self.inner.unary(req, path, codec).await
        }
        /// ConsensusStates queries all the consensus state associated with a given
        /// client.
        pub async fn consensus_states(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryConsensusStatesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryConsensusStatesResponse>,
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
                "/ibc.core.client.v1.Query/ConsensusStates",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.client.v1.Query", "ConsensusStates"));
            self.inner.unary(req, path, codec).await
        }
        /// ConsensusStateHeights queries the height of every consensus states associated with a given client.
        pub async fn consensus_state_heights(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryConsensusStateHeightsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryConsensusStateHeightsResponse>,
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
                "/ibc.core.client.v1.Query/ConsensusStateHeights",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ibc.core.client.v1.Query", "ConsensusStateHeights"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Status queries the status of an IBC client.
        pub async fn client_status(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryClientStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryClientStatusResponse>,
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
                "/ibc.core.client.v1.Query/ClientStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.client.v1.Query", "ClientStatus"));
            self.inner.unary(req, path, codec).await
        }
        /// ClientParams queries all parameters of the ibc client submodule.
        pub async fn client_params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryClientParamsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryClientParamsResponse>,
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
                "/ibc.core.client.v1.Query/ClientParams",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.client.v1.Query", "ClientParams"));
            self.inner.unary(req, path, codec).await
        }
        /// UpgradedClientState queries an Upgraded IBC light client.
        pub async fn upgraded_client_state(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryUpgradedClientStateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryUpgradedClientStateResponse>,
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
                "/ibc.core.client.v1.Query/UpgradedClientState",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ibc.core.client.v1.Query", "UpgradedClientState"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpgradedConsensusState queries an Upgraded IBC consensus state.
        pub async fn upgraded_consensus_state(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryUpgradedConsensusStateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryUpgradedConsensusStateResponse>,
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
                "/ibc.core.client.v1.Query/UpgradedConsensusState",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ibc.core.client.v1.Query", "UpgradedConsensusState"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// VerifyMembership queries an IBC light client for proof verification of a value at a given key path.
        pub async fn verify_membership(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryVerifyMembershipRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryVerifyMembershipResponse>,
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
                "/ibc.core.client.v1.Query/VerifyMembership",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.client.v1.Query", "VerifyMembership"));
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
        /// ClientState queries an IBC light client.
        async fn client_state(
            &self,
            request: tonic::Request<super::QueryClientStateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryClientStateResponse>,
            tonic::Status,
        >;
        /// ClientStates queries all the IBC light clients of a chain.
        async fn client_states(
            &self,
            request: tonic::Request<super::QueryClientStatesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryClientStatesResponse>,
            tonic::Status,
        >;
        /// ConsensusState queries a consensus state associated with a client state at
        /// a given height.
        async fn consensus_state(
            &self,
            request: tonic::Request<super::QueryConsensusStateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryConsensusStateResponse>,
            tonic::Status,
        >;
        /// ConsensusStates queries all the consensus state associated with a given
        /// client.
        async fn consensus_states(
            &self,
            request: tonic::Request<super::QueryConsensusStatesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryConsensusStatesResponse>,
            tonic::Status,
        >;
        /// ConsensusStateHeights queries the height of every consensus states associated with a given client.
        async fn consensus_state_heights(
            &self,
            request: tonic::Request<super::QueryConsensusStateHeightsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryConsensusStateHeightsResponse>,
            tonic::Status,
        >;
        /// Status queries the status of an IBC client.
        async fn client_status(
            &self,
            request: tonic::Request<super::QueryClientStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryClientStatusResponse>,
            tonic::Status,
        >;
        /// ClientParams queries all parameters of the ibc client submodule.
        async fn client_params(
            &self,
            request: tonic::Request<super::QueryClientParamsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryClientParamsResponse>,
            tonic::Status,
        >;
        /// UpgradedClientState queries an Upgraded IBC light client.
        async fn upgraded_client_state(
            &self,
            request: tonic::Request<super::QueryUpgradedClientStateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryUpgradedClientStateResponse>,
            tonic::Status,
        >;
        /// UpgradedConsensusState queries an Upgraded IBC consensus state.
        async fn upgraded_consensus_state(
            &self,
            request: tonic::Request<super::QueryUpgradedConsensusStateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryUpgradedConsensusStateResponse>,
            tonic::Status,
        >;
        /// VerifyMembership queries an IBC light client for proof verification of a value at a given key path.
        async fn verify_membership(
            &self,
            request: tonic::Request<super::QueryVerifyMembershipRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryVerifyMembershipResponse>,
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
                "/ibc.core.client.v1.Query/ClientState" => {
                    #[allow(non_camel_case_types)]
                    struct ClientStateSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryClientStateRequest>
                    for ClientStateSvc<T> {
                        type Response = super::QueryClientStateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryClientStateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::client_state(&inner, request).await
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
                        let method = ClientStateSvc(inner);
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
                "/ibc.core.client.v1.Query/ClientStates" => {
                    #[allow(non_camel_case_types)]
                    struct ClientStatesSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryClientStatesRequest>
                    for ClientStatesSvc<T> {
                        type Response = super::QueryClientStatesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryClientStatesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::client_states(&inner, request).await
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
                        let method = ClientStatesSvc(inner);
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
                "/ibc.core.client.v1.Query/ConsensusState" => {
                    #[allow(non_camel_case_types)]
                    struct ConsensusStateSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryConsensusStateRequest>
                    for ConsensusStateSvc<T> {
                        type Response = super::QueryConsensusStateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryConsensusStateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::consensus_state(&inner, request).await
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
                        let method = ConsensusStateSvc(inner);
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
                "/ibc.core.client.v1.Query/ConsensusStates" => {
                    #[allow(non_camel_case_types)]
                    struct ConsensusStatesSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryConsensusStatesRequest>
                    for ConsensusStatesSvc<T> {
                        type Response = super::QueryConsensusStatesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryConsensusStatesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::consensus_states(&inner, request).await
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
                        let method = ConsensusStatesSvc(inner);
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
                "/ibc.core.client.v1.Query/ConsensusStateHeights" => {
                    #[allow(non_camel_case_types)]
                    struct ConsensusStateHeightsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryConsensusStateHeightsRequest,
                    > for ConsensusStateHeightsSvc<T> {
                        type Response = super::QueryConsensusStateHeightsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryConsensusStateHeightsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::consensus_state_heights(&inner, request).await
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
                        let method = ConsensusStateHeightsSvc(inner);
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
                "/ibc.core.client.v1.Query/ClientStatus" => {
                    #[allow(non_camel_case_types)]
                    struct ClientStatusSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryClientStatusRequest>
                    for ClientStatusSvc<T> {
                        type Response = super::QueryClientStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryClientStatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::client_status(&inner, request).await
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
                        let method = ClientStatusSvc(inner);
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
                "/ibc.core.client.v1.Query/ClientParams" => {
                    #[allow(non_camel_case_types)]
                    struct ClientParamsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryClientParamsRequest>
                    for ClientParamsSvc<T> {
                        type Response = super::QueryClientParamsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryClientParamsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::client_params(&inner, request).await
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
                        let method = ClientParamsSvc(inner);
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
                "/ibc.core.client.v1.Query/UpgradedClientState" => {
                    #[allow(non_camel_case_types)]
                    struct UpgradedClientStateSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryUpgradedClientStateRequest>
                    for UpgradedClientStateSvc<T> {
                        type Response = super::QueryUpgradedClientStateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryUpgradedClientStateRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::upgraded_client_state(&inner, request).await
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
                        let method = UpgradedClientStateSvc(inner);
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
                "/ibc.core.client.v1.Query/UpgradedConsensusState" => {
                    #[allow(non_camel_case_types)]
                    struct UpgradedConsensusStateSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryUpgradedConsensusStateRequest,
                    > for UpgradedConsensusStateSvc<T> {
                        type Response = super::QueryUpgradedConsensusStateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryUpgradedConsensusStateRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::upgraded_consensus_state(&inner, request)
                                    .await
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
                        let method = UpgradedConsensusStateSvc(inner);
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
                "/ibc.core.client.v1.Query/VerifyMembership" => {
                    #[allow(non_camel_case_types)]
                    struct VerifyMembershipSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryVerifyMembershipRequest>
                    for VerifyMembershipSvc<T> {
                        type Response = super::QueryVerifyMembershipResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryVerifyMembershipRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::verify_membership(&inner, request).await
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
                        let method = VerifyMembershipSvc(inner);
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
        const NAME: &'static str = "ibc.core.client.v1.Query";
    }
}

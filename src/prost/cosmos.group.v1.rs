/// Member represents a group member with an account address,
/// non-zero weight, metadata and added_at timestamp.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Member {
    /// address is the member's account address.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// weight is the member's voting weight that should be greater than 0.
    #[prost(string, tag = "2")]
    pub weight: ::prost::alloc::string::String,
    /// metadata is any arbitrary metadata attached to the member.
    #[prost(string, tag = "3")]
    pub metadata: ::prost::alloc::string::String,
    /// added_at is a timestamp specifying when a member was added.
    #[prost(message, optional, tag = "4")]
    pub added_at: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
}
/// MemberRequest represents a group member to be used in Msg server requests.
/// Contrary to `Member`, it doesn't have any `added_at` field
/// since this field cannot be set as part of requests.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemberRequest {
    /// address is the member's account address.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// weight is the member's voting weight that should be greater than 0.
    #[prost(string, tag = "2")]
    pub weight: ::prost::alloc::string::String,
    /// metadata is any arbitrary metadata attached to the member.
    #[prost(string, tag = "3")]
    pub metadata: ::prost::alloc::string::String,
}
/// ThresholdDecisionPolicy is a decision policy where a proposal passes when it
/// satisfies the two following conditions:
/// 1. The sum of all `YES` voters' weights is greater or equal than the defined
///     `threshold`.
/// 2. The voting and execution periods of the proposal respect the parameters
///     given by `windows`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThresholdDecisionPolicy {
    /// threshold is the minimum weighted sum of `YES` votes that must be met or
    /// exceeded for a proposal to succeed.
    #[prost(string, tag = "1")]
    pub threshold: ::prost::alloc::string::String,
    /// windows defines the different windows for voting and execution.
    #[prost(message, optional, tag = "2")]
    pub windows: ::core::option::Option<DecisionPolicyWindows>,
}
/// PercentageDecisionPolicy is a decision policy where a proposal passes when
/// it satisfies the two following conditions:
/// 1. The percentage of all `YES` voters' weights out of the total group weight
///     is greater or equal than the given `percentage`.
/// 2. The voting and execution periods of the proposal respect the parameters
///     given by `windows`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PercentageDecisionPolicy {
    /// percentage is the minimum percentage the weighted sum of `YES` votes must
    /// meet for a proposal to succeed.
    #[prost(string, tag = "1")]
    pub percentage: ::prost::alloc::string::String,
    /// windows defines the different windows for voting and execution.
    #[prost(message, optional, tag = "2")]
    pub windows: ::core::option::Option<DecisionPolicyWindows>,
}
/// DecisionPolicyWindows defines the different windows for voting and execution.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecisionPolicyWindows {
    /// voting_period is the duration from submission of a proposal to the end of voting period
    /// Within this times votes can be submitted with MsgVote.
    #[prost(message, optional, tag = "1")]
    pub voting_period: ::core::option::Option<
        super::super::super::google::protobuf::Duration,
    >,
    /// min_execution_period is the minimum duration after the proposal submission
    /// where members can start sending MsgExec. This means that the window for
    /// sending a MsgExec transaction is:
    /// `[ submission + min_execution_period ; submission + voting_period + max_execution_period]`
    /// where max_execution_period is a app-specific config, defined in the keeper.
    /// If not set, min_execution_period will default to 0.
    ///
    /// Please make sure to set a `min_execution_period` that is smaller than
    /// `voting_period + max_execution_period`, or else the above execution window
    /// is empty, meaning that all proposals created with this decision policy
    /// won't be able to be executed.
    #[prost(message, optional, tag = "2")]
    pub min_execution_period: ::core::option::Option<
        super::super::super::google::protobuf::Duration,
    >,
}
/// GroupInfo represents the high-level on-chain information for a group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInfo {
    /// id is the unique ID of the group.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// admin is the account address of the group's admin.
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
    /// metadata is any arbitrary metadata to attached to the group.
    #[prost(string, tag = "3")]
    pub metadata: ::prost::alloc::string::String,
    /// version is used to track changes to a group's membership structure that
    /// would break existing proposals. Whenever any members weight is changed,
    /// or any member is added or removed this version is incremented and will
    /// cause proposals based on older versions of this group to fail
    #[prost(uint64, tag = "4")]
    pub version: u64,
    /// total_weight is the sum of the group members' weights.
    #[prost(string, tag = "5")]
    pub total_weight: ::prost::alloc::string::String,
    /// created_at is a timestamp specifying when a group was created.
    #[prost(message, optional, tag = "6")]
    pub created_at: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
}
/// GroupMember represents the relationship between a group and a member.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMember {
    /// group_id is the unique ID of the group.
    #[prost(uint64, tag = "1")]
    pub group_id: u64,
    /// member is the member data.
    #[prost(message, optional, tag = "2")]
    pub member: ::core::option::Option<Member>,
}
/// GroupPolicyInfo represents the high-level on-chain information for a group policy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupPolicyInfo {
    /// address is the account address of group policy.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// group_id is the unique ID of the group.
    #[prost(uint64, tag = "2")]
    pub group_id: u64,
    /// admin is the account address of the group admin.
    #[prost(string, tag = "3")]
    pub admin: ::prost::alloc::string::String,
    /// metadata is any arbitrary metadata to attached to the group policy.
    #[prost(string, tag = "4")]
    pub metadata: ::prost::alloc::string::String,
    /// version is used to track changes to a group's GroupPolicyInfo structure that
    /// would create a different result on a running proposal.
    #[prost(uint64, tag = "5")]
    pub version: u64,
    /// decision_policy specifies the group policy's decision policy.
    #[prost(message, optional, tag = "6")]
    pub decision_policy: ::core::option::Option<
        super::super::super::google::protobuf::Any,
    >,
    /// created_at is a timestamp specifying when a group policy was created.
    #[prost(message, optional, tag = "7")]
    pub created_at: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
}
/// Proposal defines a group proposal. Any member of a group can submit a proposal
/// for a group policy to decide upon.
/// A proposal consists of a set of `sdk.Msg`s that will be executed if the proposal
/// passes as well as some optional metadata associated with the proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Proposal {
    /// id is the unique id of the proposal.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// group_policy_address is the account address of group policy.
    #[prost(string, tag = "2")]
    pub group_policy_address: ::prost::alloc::string::String,
    /// metadata is any arbitrary metadata to attached to the proposal.
    #[prost(string, tag = "3")]
    pub metadata: ::prost::alloc::string::String,
    /// proposers are the account addresses of the proposers.
    #[prost(string, repeated, tag = "4")]
    pub proposers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// submit_time is a timestamp specifying when a proposal was submitted.
    #[prost(message, optional, tag = "5")]
    pub submit_time: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
    /// group_version tracks the version of the group at proposal submission.
    /// This field is here for informational purposes only.
    #[prost(uint64, tag = "6")]
    pub group_version: u64,
    /// group_policy_version tracks the version of the group policy at proposal submission.
    /// When a decision policy is changed, existing proposals from previous policy
    /// versions will become invalid with the `ABORTED` status.
    /// This field is here for informational purposes only.
    #[prost(uint64, tag = "7")]
    pub group_policy_version: u64,
    /// status represents the high level position in the life cycle of the proposal. Initial value is Submitted.
    #[prost(enumeration = "ProposalStatus", tag = "8")]
    pub status: i32,
    /// final_tally_result contains the sums of all weighted votes for this
    /// proposal for each vote option. It is empty at submission, and only
    /// populated after tallying, at voting period end or at proposal execution,
    /// whichever happens first.
    #[prost(message, optional, tag = "9")]
    pub final_tally_result: ::core::option::Option<TallyResult>,
    /// voting_period_end is the timestamp before which voting must be done.
    /// Unless a successfull MsgExec is called before (to execute a proposal whose
    /// tally is successful before the voting period ends), tallying will be done
    /// at this point, and the `final_tally_result`and `status` fields will be
    /// accordingly updated.
    #[prost(message, optional, tag = "10")]
    pub voting_period_end: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
    /// executor_result is the final result of the proposal execution. Initial value is NotRun.
    #[prost(enumeration = "ProposalExecutorResult", tag = "11")]
    pub executor_result: i32,
    /// messages is a list of `sdk.Msg`s that will be executed if the proposal passes.
    #[prost(message, repeated, tag = "12")]
    pub messages: ::prost::alloc::vec::Vec<super::super::super::google::protobuf::Any>,
}
/// TallyResult represents the sum of weighted votes for each vote option.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TallyResult {
    /// yes_count is the weighted sum of yes votes.
    #[prost(string, tag = "1")]
    pub yes_count: ::prost::alloc::string::String,
    /// abstain_count is the weighted sum of abstainers.
    #[prost(string, tag = "2")]
    pub abstain_count: ::prost::alloc::string::String,
    /// no_count is the weighted sum of no votes.
    #[prost(string, tag = "3")]
    pub no_count: ::prost::alloc::string::String,
    /// no_with_veto_count is the weighted sum of veto.
    #[prost(string, tag = "4")]
    pub no_with_veto_count: ::prost::alloc::string::String,
}
/// Vote represents a vote for a proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vote {
    /// proposal is the unique ID of the proposal.
    #[prost(uint64, tag = "1")]
    pub proposal_id: u64,
    /// voter is the account address of the voter.
    #[prost(string, tag = "2")]
    pub voter: ::prost::alloc::string::String,
    /// option is the voter's choice on the proposal.
    #[prost(enumeration = "VoteOption", tag = "3")]
    pub option: i32,
    /// metadata is any arbitrary metadata to attached to the vote.
    #[prost(string, tag = "4")]
    pub metadata: ::prost::alloc::string::String,
    /// submit_time is the timestamp when the vote was submitted.
    #[prost(message, optional, tag = "5")]
    pub submit_time: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
}
/// VoteOption enumerates the valid vote options for a given proposal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VoteOption {
    /// VOTE_OPTION_UNSPECIFIED defines an unspecified vote option which will
    /// return an error.
    Unspecified = 0,
    /// VOTE_OPTION_YES defines a yes vote option.
    Yes = 1,
    /// VOTE_OPTION_ABSTAIN defines an abstain vote option.
    Abstain = 2,
    /// VOTE_OPTION_NO defines a no vote option.
    No = 3,
    /// VOTE_OPTION_NO_WITH_VETO defines a no with veto vote option.
    NoWithVeto = 4,
}
impl VoteOption {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VoteOption::Unspecified => "VOTE_OPTION_UNSPECIFIED",
            VoteOption::Yes => "VOTE_OPTION_YES",
            VoteOption::Abstain => "VOTE_OPTION_ABSTAIN",
            VoteOption::No => "VOTE_OPTION_NO",
            VoteOption::NoWithVeto => "VOTE_OPTION_NO_WITH_VETO",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VOTE_OPTION_UNSPECIFIED" => Some(Self::Unspecified),
            "VOTE_OPTION_YES" => Some(Self::Yes),
            "VOTE_OPTION_ABSTAIN" => Some(Self::Abstain),
            "VOTE_OPTION_NO" => Some(Self::No),
            "VOTE_OPTION_NO_WITH_VETO" => Some(Self::NoWithVeto),
            _ => None,
        }
    }
}
/// ProposalStatus defines proposal statuses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProposalStatus {
    /// An empty value is invalid and not allowed.
    Unspecified = 0,
    /// Initial status of a proposal when submitted.
    Submitted = 1,
    /// Final status of a proposal when the final tally is done and the outcome
    /// passes the group policy's decision policy.
    Accepted = 2,
    /// Final status of a proposal when the final tally is done and the outcome
    /// is rejected by the group policy's decision policy.
    Rejected = 3,
    /// Final status of a proposal when the group policy is modified before the
    /// final tally.
    Aborted = 4,
    /// A proposal can be withdrawn before the voting start time by the owner.
    /// When this happens the final status is Withdrawn.
    Withdrawn = 5,
}
impl ProposalStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProposalStatus::Unspecified => "PROPOSAL_STATUS_UNSPECIFIED",
            ProposalStatus::Submitted => "PROPOSAL_STATUS_SUBMITTED",
            ProposalStatus::Accepted => "PROPOSAL_STATUS_ACCEPTED",
            ProposalStatus::Rejected => "PROPOSAL_STATUS_REJECTED",
            ProposalStatus::Aborted => "PROPOSAL_STATUS_ABORTED",
            ProposalStatus::Withdrawn => "PROPOSAL_STATUS_WITHDRAWN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROPOSAL_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "PROPOSAL_STATUS_SUBMITTED" => Some(Self::Submitted),
            "PROPOSAL_STATUS_ACCEPTED" => Some(Self::Accepted),
            "PROPOSAL_STATUS_REJECTED" => Some(Self::Rejected),
            "PROPOSAL_STATUS_ABORTED" => Some(Self::Aborted),
            "PROPOSAL_STATUS_WITHDRAWN" => Some(Self::Withdrawn),
            _ => None,
        }
    }
}
/// ProposalExecutorResult defines types of proposal executor results.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProposalExecutorResult {
    /// An empty value is not allowed.
    Unspecified = 0,
    /// We have not yet run the executor.
    NotRun = 1,
    /// The executor was successful and proposed action updated state.
    Success = 2,
    /// The executor returned an error and proposed action didn't update state.
    Failure = 3,
}
impl ProposalExecutorResult {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProposalExecutorResult::Unspecified => "PROPOSAL_EXECUTOR_RESULT_UNSPECIFIED",
            ProposalExecutorResult::NotRun => "PROPOSAL_EXECUTOR_RESULT_NOT_RUN",
            ProposalExecutorResult::Success => "PROPOSAL_EXECUTOR_RESULT_SUCCESS",
            ProposalExecutorResult::Failure => "PROPOSAL_EXECUTOR_RESULT_FAILURE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROPOSAL_EXECUTOR_RESULT_UNSPECIFIED" => Some(Self::Unspecified),
            "PROPOSAL_EXECUTOR_RESULT_NOT_RUN" => Some(Self::NotRun),
            "PROPOSAL_EXECUTOR_RESULT_SUCCESS" => Some(Self::Success),
            "PROPOSAL_EXECUTOR_RESULT_FAILURE" => Some(Self::Failure),
            _ => None,
        }
    }
}
/// MsgCreateGroup is the Msg/CreateGroup request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateGroup {
    /// admin is the account address of the group admin.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// members defines the group members.
    #[prost(message, repeated, tag = "2")]
    pub members: ::prost::alloc::vec::Vec<MemberRequest>,
    /// metadata is any arbitrary metadata to attached to the group.
    #[prost(string, tag = "3")]
    pub metadata: ::prost::alloc::string::String,
}
/// MsgCreateGroupResponse is the Msg/CreateGroup response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateGroupResponse {
    /// group_id is the unique ID of the newly created group.
    #[prost(uint64, tag = "1")]
    pub group_id: u64,
}
/// MsgUpdateGroupMembers is the Msg/UpdateGroupMembers request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateGroupMembers {
    /// admin is the account address of the group admin.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// group_id is the unique ID of the group.
    #[prost(uint64, tag = "2")]
    pub group_id: u64,
    /// member_updates is the list of members to update,
    /// set weight to 0 to remove a member.
    #[prost(message, repeated, tag = "3")]
    pub member_updates: ::prost::alloc::vec::Vec<MemberRequest>,
}
/// MsgUpdateGroupMembersResponse is the Msg/UpdateGroupMembers response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateGroupMembersResponse {}
/// MsgUpdateGroupAdmin is the Msg/UpdateGroupAdmin request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateGroupAdmin {
    /// admin is the current account address of the group admin.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// group_id is the unique ID of the group.
    #[prost(uint64, tag = "2")]
    pub group_id: u64,
    /// new_admin is the group new admin account address.
    #[prost(string, tag = "3")]
    pub new_admin: ::prost::alloc::string::String,
}
/// MsgUpdateGroupAdminResponse is the Msg/UpdateGroupAdmin response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateGroupAdminResponse {}
/// MsgUpdateGroupMetadata is the Msg/UpdateGroupMetadata request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateGroupMetadata {
    /// admin is the account address of the group admin.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// group_id is the unique ID of the group.
    #[prost(uint64, tag = "2")]
    pub group_id: u64,
    /// metadata is the updated group's metadata.
    #[prost(string, tag = "3")]
    pub metadata: ::prost::alloc::string::String,
}
/// MsgUpdateGroupMetadataResponse is the Msg/UpdateGroupMetadata response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateGroupMetadataResponse {}
/// MsgCreateGroupPolicy is the Msg/CreateGroupPolicy request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateGroupPolicy {
    /// admin is the account address of the group admin.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// group_id is the unique ID of the group.
    #[prost(uint64, tag = "2")]
    pub group_id: u64,
    /// metadata is any arbitrary metadata attached to the group policy.
    #[prost(string, tag = "3")]
    pub metadata: ::prost::alloc::string::String,
    /// decision_policy specifies the group policy's decision policy.
    #[prost(message, optional, tag = "4")]
    pub decision_policy: ::core::option::Option<
        super::super::super::google::protobuf::Any,
    >,
}
/// MsgCreateGroupPolicyResponse is the Msg/CreateGroupPolicy response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateGroupPolicyResponse {
    /// address is the account address of the newly created group policy.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// MsgUpdateGroupPolicyAdmin is the Msg/UpdateGroupPolicyAdmin request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateGroupPolicyAdmin {
    /// admin is the account address of the group admin.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// group_policy_address is the account address of the group policy.
    #[prost(string, tag = "2")]
    pub group_policy_address: ::prost::alloc::string::String,
    /// new_admin is the new group policy admin.
    #[prost(string, tag = "3")]
    pub new_admin: ::prost::alloc::string::String,
}
/// MsgCreateGroupWithPolicy is the Msg/CreateGroupWithPolicy request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateGroupWithPolicy {
    /// admin is the account address of the group and group policy admin.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// members defines the group members.
    #[prost(message, repeated, tag = "2")]
    pub members: ::prost::alloc::vec::Vec<MemberRequest>,
    /// group_metadata is any arbitrary metadata attached to the group.
    #[prost(string, tag = "3")]
    pub group_metadata: ::prost::alloc::string::String,
    /// group_policy_metadata is any arbitrary metadata attached to the group policy.
    #[prost(string, tag = "4")]
    pub group_policy_metadata: ::prost::alloc::string::String,
    /// group_policy_as_admin is a boolean field, if set to true, the group policy account address will be used as group
    /// and group policy admin.
    #[prost(bool, tag = "5")]
    pub group_policy_as_admin: bool,
    /// decision_policy specifies the group policy's decision policy.
    #[prost(message, optional, tag = "6")]
    pub decision_policy: ::core::option::Option<
        super::super::super::google::protobuf::Any,
    >,
}
/// MsgCreateGroupWithPolicyResponse is the Msg/CreateGroupWithPolicy response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateGroupWithPolicyResponse {
    /// group_id is the unique ID of the newly created group with policy.
    #[prost(uint64, tag = "1")]
    pub group_id: u64,
    /// group_policy_address is the account address of the newly created group policy.
    #[prost(string, tag = "2")]
    pub group_policy_address: ::prost::alloc::string::String,
}
/// MsgUpdateGroupPolicyAdminResponse is the Msg/UpdateGroupPolicyAdmin response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateGroupPolicyAdminResponse {}
/// MsgUpdateGroupPolicyDecisionPolicy is the Msg/UpdateGroupPolicyDecisionPolicy request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateGroupPolicyDecisionPolicy {
    /// admin is the account address of the group admin.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// group_policy_address is the account address of group policy.
    #[prost(string, tag = "2")]
    pub group_policy_address: ::prost::alloc::string::String,
    /// decision_policy is the updated group policy's decision policy.
    #[prost(message, optional, tag = "3")]
    pub decision_policy: ::core::option::Option<
        super::super::super::google::protobuf::Any,
    >,
}
/// MsgUpdateGroupPolicyDecisionPolicyResponse is the Msg/UpdateGroupPolicyDecisionPolicy response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateGroupPolicyDecisionPolicyResponse {}
/// MsgUpdateGroupPolicyMetadata is the Msg/UpdateGroupPolicyMetadata request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateGroupPolicyMetadata {
    /// admin is the account address of the group admin.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// group_policy_address is the account address of group policy.
    #[prost(string, tag = "2")]
    pub group_policy_address: ::prost::alloc::string::String,
    /// metadata is the updated group policy metadata.
    #[prost(string, tag = "3")]
    pub metadata: ::prost::alloc::string::String,
}
/// MsgUpdateGroupPolicyMetadataResponse is the Msg/UpdateGroupPolicyMetadata response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateGroupPolicyMetadataResponse {}
/// MsgSubmitProposal is the Msg/SubmitProposal request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitProposal {
    /// group_policy_address is the account address of group policy.
    #[prost(string, tag = "1")]
    pub group_policy_address: ::prost::alloc::string::String,
    /// proposers are the account addresses of the proposers.
    /// Proposers signatures will be counted as yes votes.
    #[prost(string, repeated, tag = "2")]
    pub proposers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// metadata is any arbitrary metadata to attached to the proposal.
    #[prost(string, tag = "3")]
    pub metadata: ::prost::alloc::string::String,
    /// messages is a list of `sdk.Msg`s that will be executed if the proposal passes.
    #[prost(message, repeated, tag = "4")]
    pub messages: ::prost::alloc::vec::Vec<super::super::super::google::protobuf::Any>,
    /// exec defines the mode of execution of the proposal,
    /// whether it should be executed immediately on creation or not.
    /// If so, proposers signatures are considered as Yes votes.
    #[prost(enumeration = "Exec", tag = "5")]
    pub exec: i32,
}
/// MsgSubmitProposalResponse is the Msg/SubmitProposal response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitProposalResponse {
    /// proposal is the unique ID of the proposal.
    #[prost(uint64, tag = "1")]
    pub proposal_id: u64,
}
/// MsgWithdrawProposal is the Msg/WithdrawProposal request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawProposal {
    /// proposal is the unique ID of the proposal.
    #[prost(uint64, tag = "1")]
    pub proposal_id: u64,
    /// address is the admin of the group policy or one of the proposer of the proposal.
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
/// MsgWithdrawProposalResponse is the Msg/WithdrawProposal response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawProposalResponse {}
/// MsgVote is the Msg/Vote request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgVote {
    /// proposal is the unique ID of the proposal.
    #[prost(uint64, tag = "1")]
    pub proposal_id: u64,
    /// voter is the voter account address.
    #[prost(string, tag = "2")]
    pub voter: ::prost::alloc::string::String,
    /// option is the voter's choice on the proposal.
    #[prost(enumeration = "VoteOption", tag = "3")]
    pub option: i32,
    /// metadata is any arbitrary metadata to attached to the vote.
    #[prost(string, tag = "4")]
    pub metadata: ::prost::alloc::string::String,
    /// exec defines whether the proposal should be executed
    /// immediately after voting or not.
    #[prost(enumeration = "Exec", tag = "5")]
    pub exec: i32,
}
/// MsgVoteResponse is the Msg/Vote response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgVoteResponse {}
/// MsgExec is the Msg/Exec request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExec {
    /// proposal is the unique ID of the proposal.
    #[prost(uint64, tag = "1")]
    pub proposal_id: u64,
    /// executor is the account address used to execute the proposal.
    #[prost(string, tag = "2")]
    pub executor: ::prost::alloc::string::String,
}
/// MsgExecResponse is the Msg/Exec request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecResponse {
    /// result is the final result of the proposal execution.
    #[prost(enumeration = "ProposalExecutorResult", tag = "2")]
    pub result: i32,
}
/// MsgLeaveGroup is the Msg/LeaveGroup request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLeaveGroup {
    /// address is the account address of the group member.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// group_id is the unique ID of the group.
    #[prost(uint64, tag = "2")]
    pub group_id: u64,
}
/// MsgLeaveGroupResponse is the Msg/LeaveGroup response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLeaveGroupResponse {}
/// Exec defines modes of execution of a proposal on creation or on new vote.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Exec {
    /// An empty value means that there should be a separate
    /// MsgExec request for the proposal to execute.
    Unspecified = 0,
    /// Try to execute the proposal immediately.
    /// If the proposal is not allowed per the DecisionPolicy,
    /// the proposal will still be open and could
    /// be executed at a later point.
    Try = 1,
}
impl Exec {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Exec::Unspecified => "EXEC_UNSPECIFIED",
            Exec::Try => "EXEC_TRY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXEC_UNSPECIFIED" => Some(Self::Unspecified),
            "EXEC_TRY" => Some(Self::Try),
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
    /// Msg is the cosmos.group.v1 Msg service.
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
        /// CreateGroup creates a new group with an admin account address, a list of members and some optional metadata.
        pub async fn create_group(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateGroup>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreateGroupResponse>,
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
                "/cosmos.group.v1.Msg/CreateGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.group.v1.Msg", "CreateGroup"));
            self.inner.unary(req, path, codec).await
        }
        /// UpdateGroupMembers updates the group members with given group id and admin address.
        pub async fn update_group_members(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateGroupMembers>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateGroupMembersResponse>,
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
                "/cosmos.group.v1.Msg/UpdateGroupMembers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.group.v1.Msg", "UpdateGroupMembers"));
            self.inner.unary(req, path, codec).await
        }
        /// UpdateGroupAdmin updates the group admin with given group id and previous admin address.
        pub async fn update_group_admin(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateGroupAdmin>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateGroupAdminResponse>,
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
                "/cosmos.group.v1.Msg/UpdateGroupAdmin",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.group.v1.Msg", "UpdateGroupAdmin"));
            self.inner.unary(req, path, codec).await
        }
        /// UpdateGroupMetadata updates the group metadata with given group id and admin address.
        pub async fn update_group_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateGroupMetadata>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateGroupMetadataResponse>,
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
                "/cosmos.group.v1.Msg/UpdateGroupMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.group.v1.Msg", "UpdateGroupMetadata"));
            self.inner.unary(req, path, codec).await
        }
        /// CreateGroupPolicy creates a new group policy using given DecisionPolicy.
        pub async fn create_group_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateGroupPolicy>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreateGroupPolicyResponse>,
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
                "/cosmos.group.v1.Msg/CreateGroupPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.group.v1.Msg", "CreateGroupPolicy"));
            self.inner.unary(req, path, codec).await
        }
        /// CreateGroupWithPolicy creates a new group with policy.
        pub async fn create_group_with_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateGroupWithPolicy>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreateGroupWithPolicyResponse>,
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
                "/cosmos.group.v1.Msg/CreateGroupWithPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.group.v1.Msg", "CreateGroupWithPolicy"));
            self.inner.unary(req, path, codec).await
        }
        /// UpdateGroupPolicyAdmin updates a group policy admin.
        pub async fn update_group_policy_admin(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateGroupPolicyAdmin>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateGroupPolicyAdminResponse>,
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
                "/cosmos.group.v1.Msg/UpdateGroupPolicyAdmin",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cosmos.group.v1.Msg", "UpdateGroupPolicyAdmin"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpdateGroupPolicyDecisionPolicy allows a group policy's decision policy to be updated.
        pub async fn update_group_policy_decision_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateGroupPolicyDecisionPolicy>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateGroupPolicyDecisionPolicyResponse>,
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
                "/cosmos.group.v1.Msg/UpdateGroupPolicyDecisionPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cosmos.group.v1.Msg",
                        "UpdateGroupPolicyDecisionPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpdateGroupPolicyMetadata updates a group policy metadata.
        pub async fn update_group_policy_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateGroupPolicyMetadata>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateGroupPolicyMetadataResponse>,
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
                "/cosmos.group.v1.Msg/UpdateGroupPolicyMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cosmos.group.v1.Msg", "UpdateGroupPolicyMetadata"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// SubmitProposal submits a new proposal.
        pub async fn submit_proposal(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSubmitProposal>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSubmitProposalResponse>,
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
                "/cosmos.group.v1.Msg/SubmitProposal",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.group.v1.Msg", "SubmitProposal"));
            self.inner.unary(req, path, codec).await
        }
        /// WithdrawProposal withdraws a proposal.
        pub async fn withdraw_proposal(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWithdrawProposal>,
        ) -> std::result::Result<
            tonic::Response<super::MsgWithdrawProposalResponse>,
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
                "/cosmos.group.v1.Msg/WithdrawProposal",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.group.v1.Msg", "WithdrawProposal"));
            self.inner.unary(req, path, codec).await
        }
        /// Vote allows a voter to vote on a proposal.
        pub async fn vote(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgVote>,
        ) -> std::result::Result<
            tonic::Response<super::MsgVoteResponse>,
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
            let path = http::uri::PathAndQuery::from_static("/cosmos.group.v1.Msg/Vote");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cosmos.group.v1.Msg", "Vote"));
            self.inner.unary(req, path, codec).await
        }
        /// Exec executes a proposal.
        pub async fn exec(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgExec>,
        ) -> std::result::Result<
            tonic::Response<super::MsgExecResponse>,
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
            let path = http::uri::PathAndQuery::from_static("/cosmos.group.v1.Msg/Exec");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cosmos.group.v1.Msg", "Exec"));
            self.inner.unary(req, path, codec).await
        }
        /// LeaveGroup allows a group member to leave the group.
        pub async fn leave_group(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgLeaveGroup>,
        ) -> std::result::Result<
            tonic::Response<super::MsgLeaveGroupResponse>,
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
                "/cosmos.group.v1.Msg/LeaveGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.group.v1.Msg", "LeaveGroup"));
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
        /// CreateGroup creates a new group with an admin account address, a list of members and some optional metadata.
        async fn create_group(
            &self,
            request: tonic::Request<super::MsgCreateGroup>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreateGroupResponse>,
            tonic::Status,
        >;
        /// UpdateGroupMembers updates the group members with given group id and admin address.
        async fn update_group_members(
            &self,
            request: tonic::Request<super::MsgUpdateGroupMembers>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateGroupMembersResponse>,
            tonic::Status,
        >;
        /// UpdateGroupAdmin updates the group admin with given group id and previous admin address.
        async fn update_group_admin(
            &self,
            request: tonic::Request<super::MsgUpdateGroupAdmin>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateGroupAdminResponse>,
            tonic::Status,
        >;
        /// UpdateGroupMetadata updates the group metadata with given group id and admin address.
        async fn update_group_metadata(
            &self,
            request: tonic::Request<super::MsgUpdateGroupMetadata>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateGroupMetadataResponse>,
            tonic::Status,
        >;
        /// CreateGroupPolicy creates a new group policy using given DecisionPolicy.
        async fn create_group_policy(
            &self,
            request: tonic::Request<super::MsgCreateGroupPolicy>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreateGroupPolicyResponse>,
            tonic::Status,
        >;
        /// CreateGroupWithPolicy creates a new group with policy.
        async fn create_group_with_policy(
            &self,
            request: tonic::Request<super::MsgCreateGroupWithPolicy>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreateGroupWithPolicyResponse>,
            tonic::Status,
        >;
        /// UpdateGroupPolicyAdmin updates a group policy admin.
        async fn update_group_policy_admin(
            &self,
            request: tonic::Request<super::MsgUpdateGroupPolicyAdmin>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateGroupPolicyAdminResponse>,
            tonic::Status,
        >;
        /// UpdateGroupPolicyDecisionPolicy allows a group policy's decision policy to be updated.
        async fn update_group_policy_decision_policy(
            &self,
            request: tonic::Request<super::MsgUpdateGroupPolicyDecisionPolicy>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateGroupPolicyDecisionPolicyResponse>,
            tonic::Status,
        >;
        /// UpdateGroupPolicyMetadata updates a group policy metadata.
        async fn update_group_policy_metadata(
            &self,
            request: tonic::Request<super::MsgUpdateGroupPolicyMetadata>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateGroupPolicyMetadataResponse>,
            tonic::Status,
        >;
        /// SubmitProposal submits a new proposal.
        async fn submit_proposal(
            &self,
            request: tonic::Request<super::MsgSubmitProposal>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSubmitProposalResponse>,
            tonic::Status,
        >;
        /// WithdrawProposal withdraws a proposal.
        async fn withdraw_proposal(
            &self,
            request: tonic::Request<super::MsgWithdrawProposal>,
        ) -> std::result::Result<
            tonic::Response<super::MsgWithdrawProposalResponse>,
            tonic::Status,
        >;
        /// Vote allows a voter to vote on a proposal.
        async fn vote(
            &self,
            request: tonic::Request<super::MsgVote>,
        ) -> std::result::Result<tonic::Response<super::MsgVoteResponse>, tonic::Status>;
        /// Exec executes a proposal.
        async fn exec(
            &self,
            request: tonic::Request<super::MsgExec>,
        ) -> std::result::Result<tonic::Response<super::MsgExecResponse>, tonic::Status>;
        /// LeaveGroup allows a group member to leave the group.
        async fn leave_group(
            &self,
            request: tonic::Request<super::MsgLeaveGroup>,
        ) -> std::result::Result<
            tonic::Response<super::MsgLeaveGroupResponse>,
            tonic::Status,
        >;
    }
    /// Msg is the cosmos.group.v1 Msg service.
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
                "/cosmos.group.v1.Msg/CreateGroup" => {
                    #[allow(non_camel_case_types)]
                    struct CreateGroupSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCreateGroup>
                    for CreateGroupSvc<T> {
                        type Response = super::MsgCreateGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCreateGroup>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_group(request).await
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
                        let method = CreateGroupSvc(inner);
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
                "/cosmos.group.v1.Msg/UpdateGroupMembers" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateGroupMembersSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgUpdateGroupMembers>
                    for UpdateGroupMembersSvc<T> {
                        type Response = super::MsgUpdateGroupMembersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpdateGroupMembers>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_group_members(request).await
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
                        let method = UpdateGroupMembersSvc(inner);
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
                "/cosmos.group.v1.Msg/UpdateGroupAdmin" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateGroupAdminSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUpdateGroupAdmin>
                    for UpdateGroupAdminSvc<T> {
                        type Response = super::MsgUpdateGroupAdminResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpdateGroupAdmin>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_group_admin(request).await
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
                        let method = UpdateGroupAdminSvc(inner);
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
                "/cosmos.group.v1.Msg/UpdateGroupMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateGroupMetadataSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgUpdateGroupMetadata>
                    for UpdateGroupMetadataSvc<T> {
                        type Response = super::MsgUpdateGroupMetadataResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpdateGroupMetadata>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_group_metadata(request).await
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
                        let method = UpdateGroupMetadataSvc(inner);
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
                "/cosmos.group.v1.Msg/CreateGroupPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct CreateGroupPolicySvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCreateGroupPolicy>
                    for CreateGroupPolicySvc<T> {
                        type Response = super::MsgCreateGroupPolicyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCreateGroupPolicy>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_group_policy(request).await
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
                        let method = CreateGroupPolicySvc(inner);
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
                "/cosmos.group.v1.Msg/CreateGroupWithPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct CreateGroupWithPolicySvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgCreateGroupWithPolicy>
                    for CreateGroupWithPolicySvc<T> {
                        type Response = super::MsgCreateGroupWithPolicyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCreateGroupWithPolicy>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_group_with_policy(request).await
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
                        let method = CreateGroupWithPolicySvc(inner);
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
                "/cosmos.group.v1.Msg/UpdateGroupPolicyAdmin" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateGroupPolicyAdminSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgUpdateGroupPolicyAdmin>
                    for UpdateGroupPolicyAdminSvc<T> {
                        type Response = super::MsgUpdateGroupPolicyAdminResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpdateGroupPolicyAdmin>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_group_policy_admin(request).await
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
                        let method = UpdateGroupPolicyAdminSvc(inner);
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
                "/cosmos.group.v1.Msg/UpdateGroupPolicyDecisionPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateGroupPolicyDecisionPolicySvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<
                        super::MsgUpdateGroupPolicyDecisionPolicy,
                    > for UpdateGroupPolicyDecisionPolicySvc<T> {
                        type Response = super::MsgUpdateGroupPolicyDecisionPolicyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::MsgUpdateGroupPolicyDecisionPolicy,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_group_policy_decision_policy(request).await
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
                        let method = UpdateGroupPolicyDecisionPolicySvc(inner);
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
                "/cosmos.group.v1.Msg/UpdateGroupPolicyMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateGroupPolicyMetadataSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgUpdateGroupPolicyMetadata>
                    for UpdateGroupPolicyMetadataSvc<T> {
                        type Response = super::MsgUpdateGroupPolicyMetadataResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpdateGroupPolicyMetadata>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_group_policy_metadata(request).await
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
                        let method = UpdateGroupPolicyMetadataSvc(inner);
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
                "/cosmos.group.v1.Msg/SubmitProposal" => {
                    #[allow(non_camel_case_types)]
                    struct SubmitProposalSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgSubmitProposal>
                    for SubmitProposalSvc<T> {
                        type Response = super::MsgSubmitProposalResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSubmitProposal>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).submit_proposal(request).await
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
                        let method = SubmitProposalSvc(inner);
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
                "/cosmos.group.v1.Msg/WithdrawProposal" => {
                    #[allow(non_camel_case_types)]
                    struct WithdrawProposalSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgWithdrawProposal>
                    for WithdrawProposalSvc<T> {
                        type Response = super::MsgWithdrawProposalResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWithdrawProposal>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).withdraw_proposal(request).await
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
                        let method = WithdrawProposalSvc(inner);
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
                "/cosmos.group.v1.Msg/Vote" => {
                    #[allow(non_camel_case_types)]
                    struct VoteSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgVote>
                    for VoteSvc<T> {
                        type Response = super::MsgVoteResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgVote>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).vote(request).await };
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
                        let method = VoteSvc(inner);
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
                "/cosmos.group.v1.Msg/Exec" => {
                    #[allow(non_camel_case_types)]
                    struct ExecSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgExec>
                    for ExecSvc<T> {
                        type Response = super::MsgExecResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgExec>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).exec(request).await };
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
                        let method = ExecSvc(inner);
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
                "/cosmos.group.v1.Msg/LeaveGroup" => {
                    #[allow(non_camel_case_types)]
                    struct LeaveGroupSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgLeaveGroup>
                    for LeaveGroupSvc<T> {
                        type Response = super::MsgLeaveGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgLeaveGroup>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).leave_group(request).await };
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
                        let method = LeaveGroupSvc(inner);
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
        const NAME: &'static str = "cosmos.group.v1.Msg";
    }
}
/// EventCreateGroup is an event emitted when a group is created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateGroup {
    /// group_id is the unique ID of the group.
    #[prost(uint64, tag = "1")]
    pub group_id: u64,
}
/// EventUpdateGroup is an event emitted when a group is updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventUpdateGroup {
    /// group_id is the unique ID of the group.
    #[prost(uint64, tag = "1")]
    pub group_id: u64,
}
/// EventCreateGroupPolicy is an event emitted when a group policy is created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateGroupPolicy {
    /// address is the account address of the group policy.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// EventUpdateGroupPolicy is an event emitted when a group policy is updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventUpdateGroupPolicy {
    /// address is the account address of the group policy.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// EventSubmitProposal is an event emitted when a proposal is created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSubmitProposal {
    /// proposal_id is the unique ID of the proposal.
    #[prost(uint64, tag = "1")]
    pub proposal_id: u64,
}
/// EventWithdrawProposal is an event emitted when a proposal is withdrawn.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventWithdrawProposal {
    /// proposal_id is the unique ID of the proposal.
    #[prost(uint64, tag = "1")]
    pub proposal_id: u64,
}
/// EventVote is an event emitted when a voter votes on a proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventVote {
    /// proposal_id is the unique ID of the proposal.
    #[prost(uint64, tag = "1")]
    pub proposal_id: u64,
}
/// EventExec is an event emitted when a proposal is executed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventExec {
    /// proposal_id is the unique ID of the proposal.
    #[prost(uint64, tag = "1")]
    pub proposal_id: u64,
    /// result is the proposal execution result.
    #[prost(enumeration = "ProposalExecutorResult", tag = "2")]
    pub result: i32,
    /// logs contains error logs in case the execution result is FAILURE.
    #[prost(string, tag = "3")]
    pub logs: ::prost::alloc::string::String,
}
/// EventLeaveGroup is an event emitted when group member leaves the group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventLeaveGroup {
    /// group_id is the unique ID of the group.
    #[prost(uint64, tag = "1")]
    pub group_id: u64,
    /// address is the account address of the group member.
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
/// QueryGroupInfoRequest is the Query/GroupInfo request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGroupInfoRequest {
    /// group_id is the unique ID of the group.
    #[prost(uint64, tag = "1")]
    pub group_id: u64,
}
/// QueryGroupInfoResponse is the Query/GroupInfo response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGroupInfoResponse {
    /// info is the GroupInfo for the group.
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<GroupInfo>,
}
/// QueryGroupPolicyInfoRequest is the Query/GroupPolicyInfo request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGroupPolicyInfoRequest {
    /// address is the account address of the group policy.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryGroupPolicyInfoResponse is the Query/GroupPolicyInfo response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGroupPolicyInfoResponse {
    /// info is the GroupPolicyInfo for the group policy.
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<GroupPolicyInfo>,
}
/// QueryGroupMembersRequest is the Query/GroupMembers request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGroupMembersRequest {
    /// group_id is the unique ID of the group.
    #[prost(uint64, tag = "1")]
    pub group_id: u64,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::base::query::v1beta1::PageRequest,
    >,
}
/// QueryGroupMembersResponse is the Query/GroupMembersResponse response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGroupMembersResponse {
    /// members are the members of the group with given group_id.
    #[prost(message, repeated, tag = "1")]
    pub members: ::prost::alloc::vec::Vec<GroupMember>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::base::query::v1beta1::PageResponse,
    >,
}
/// QueryGroupsByAdminRequest is the Query/GroupsByAdmin request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGroupsByAdminRequest {
    /// admin is the account address of a group's admin.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::base::query::v1beta1::PageRequest,
    >,
}
/// QueryGroupsByAdminResponse is the Query/GroupsByAdminResponse response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGroupsByAdminResponse {
    /// groups are the groups info with the provided admin.
    #[prost(message, repeated, tag = "1")]
    pub groups: ::prost::alloc::vec::Vec<GroupInfo>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::base::query::v1beta1::PageResponse,
    >,
}
/// QueryGroupPoliciesByGroupRequest is the Query/GroupPoliciesByGroup request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGroupPoliciesByGroupRequest {
    /// group_id is the unique ID of the group policy's group.
    #[prost(uint64, tag = "1")]
    pub group_id: u64,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::base::query::v1beta1::PageRequest,
    >,
}
/// QueryGroupPoliciesByGroupResponse is the Query/GroupPoliciesByGroup response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGroupPoliciesByGroupResponse {
    /// group_policies are the group policies info associated with the provided group.
    #[prost(message, repeated, tag = "1")]
    pub group_policies: ::prost::alloc::vec::Vec<GroupPolicyInfo>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::base::query::v1beta1::PageResponse,
    >,
}
/// QueryGroupPoliciesByAdminRequest is the Query/GroupPoliciesByAdmin request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGroupPoliciesByAdminRequest {
    /// admin is the admin address of the group policy.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::base::query::v1beta1::PageRequest,
    >,
}
/// QueryGroupPoliciesByAdminResponse is the Query/GroupPoliciesByAdmin response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGroupPoliciesByAdminResponse {
    /// group_policies are the group policies info with provided admin.
    #[prost(message, repeated, tag = "1")]
    pub group_policies: ::prost::alloc::vec::Vec<GroupPolicyInfo>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::base::query::v1beta1::PageResponse,
    >,
}
/// QueryProposalRequest is the Query/Proposal request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProposalRequest {
    /// proposal_id is the unique ID of a proposal.
    #[prost(uint64, tag = "1")]
    pub proposal_id: u64,
}
/// QueryProposalResponse is the Query/Proposal response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProposalResponse {
    /// proposal is the proposal info.
    #[prost(message, optional, tag = "1")]
    pub proposal: ::core::option::Option<Proposal>,
}
/// QueryProposalsByGroupPolicyRequest is the Query/ProposalByGroupPolicy request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProposalsByGroupPolicyRequest {
    /// address is the account address of the group policy related to proposals.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::base::query::v1beta1::PageRequest,
    >,
}
/// QueryProposalsByGroupPolicyResponse is the Query/ProposalByGroupPolicy response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProposalsByGroupPolicyResponse {
    /// proposals are the proposals with given group policy.
    #[prost(message, repeated, tag = "1")]
    pub proposals: ::prost::alloc::vec::Vec<Proposal>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::base::query::v1beta1::PageResponse,
    >,
}
/// QueryVoteByProposalVoterRequest is the Query/VoteByProposalVoter request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVoteByProposalVoterRequest {
    /// proposal_id is the unique ID of a proposal.
    #[prost(uint64, tag = "1")]
    pub proposal_id: u64,
    /// voter is a proposal voter account address.
    #[prost(string, tag = "2")]
    pub voter: ::prost::alloc::string::String,
}
/// QueryVoteByProposalVoterResponse is the Query/VoteByProposalVoter response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVoteByProposalVoterResponse {
    /// vote is the vote with given proposal_id and voter.
    #[prost(message, optional, tag = "1")]
    pub vote: ::core::option::Option<Vote>,
}
/// QueryVotesByProposalRequest is the Query/VotesByProposal request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVotesByProposalRequest {
    /// proposal_id is the unique ID of a proposal.
    #[prost(uint64, tag = "1")]
    pub proposal_id: u64,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::base::query::v1beta1::PageRequest,
    >,
}
/// QueryVotesByProposalResponse is the Query/VotesByProposal response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVotesByProposalResponse {
    /// votes are the list of votes for given proposal_id.
    #[prost(message, repeated, tag = "1")]
    pub votes: ::prost::alloc::vec::Vec<Vote>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::base::query::v1beta1::PageResponse,
    >,
}
/// QueryVotesByVoterRequest is the Query/VotesByVoter request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVotesByVoterRequest {
    /// voter is a proposal voter account address.
    #[prost(string, tag = "1")]
    pub voter: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::base::query::v1beta1::PageRequest,
    >,
}
/// QueryVotesByVoterResponse is the Query/VotesByVoter response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVotesByVoterResponse {
    /// votes are the list of votes by given voter.
    #[prost(message, repeated, tag = "1")]
    pub votes: ::prost::alloc::vec::Vec<Vote>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::base::query::v1beta1::PageResponse,
    >,
}
/// QueryGroupsByMemberRequest is the Query/GroupsByMember request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGroupsByMemberRequest {
    /// address is the group member address.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::base::query::v1beta1::PageRequest,
    >,
}
/// QueryGroupsByMemberResponse is the Query/GroupsByMember response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGroupsByMemberResponse {
    /// groups are the groups info with the provided group member.
    #[prost(message, repeated, tag = "1")]
    pub groups: ::prost::alloc::vec::Vec<GroupInfo>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::base::query::v1beta1::PageResponse,
    >,
}
/// QueryTallyResultRequest is the Query/TallyResult request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTallyResultRequest {
    /// proposal_id is the unique id of a proposal.
    #[prost(uint64, tag = "1")]
    pub proposal_id: u64,
}
/// QueryTallyResultResponse is the Query/TallyResult response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTallyResultResponse {
    /// tally defines the requested tally.
    #[prost(message, optional, tag = "1")]
    pub tally: ::core::option::Option<TallyResult>,
}
/// Generated client implementations.
#[cfg(feature = "client")]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Query is the cosmos.group.v1 Query service.
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
        /// GroupInfo queries group info based on group id.
        pub async fn group_info(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGroupInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGroupInfoResponse>,
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
                "/cosmos.group.v1.Query/GroupInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.group.v1.Query", "GroupInfo"));
            self.inner.unary(req, path, codec).await
        }
        /// GroupPolicyInfo queries group policy info based on account address of group policy.
        pub async fn group_policy_info(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGroupPolicyInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGroupPolicyInfoResponse>,
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
                "/cosmos.group.v1.Query/GroupPolicyInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.group.v1.Query", "GroupPolicyInfo"));
            self.inner.unary(req, path, codec).await
        }
        /// GroupMembers queries members of a group
        pub async fn group_members(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGroupMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGroupMembersResponse>,
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
                "/cosmos.group.v1.Query/GroupMembers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.group.v1.Query", "GroupMembers"));
            self.inner.unary(req, path, codec).await
        }
        /// GroupsByAdmin queries groups by admin address.
        pub async fn groups_by_admin(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGroupsByAdminRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGroupsByAdminResponse>,
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
                "/cosmos.group.v1.Query/GroupsByAdmin",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.group.v1.Query", "GroupsByAdmin"));
            self.inner.unary(req, path, codec).await
        }
        /// GroupPoliciesByGroup queries group policies by group id.
        pub async fn group_policies_by_group(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGroupPoliciesByGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGroupPoliciesByGroupResponse>,
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
                "/cosmos.group.v1.Query/GroupPoliciesByGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cosmos.group.v1.Query", "GroupPoliciesByGroup"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GroupsByAdmin queries group policies by admin address.
        pub async fn group_policies_by_admin(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGroupPoliciesByAdminRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGroupPoliciesByAdminResponse>,
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
                "/cosmos.group.v1.Query/GroupPoliciesByAdmin",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cosmos.group.v1.Query", "GroupPoliciesByAdmin"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Proposal queries a proposal based on proposal id.
        pub async fn proposal(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryProposalRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryProposalResponse>,
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
                "/cosmos.group.v1.Query/Proposal",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.group.v1.Query", "Proposal"));
            self.inner.unary(req, path, codec).await
        }
        /// ProposalsByGroupPolicy queries proposals based on account address of group policy.
        pub async fn proposals_by_group_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryProposalsByGroupPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryProposalsByGroupPolicyResponse>,
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
                "/cosmos.group.v1.Query/ProposalsByGroupPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cosmos.group.v1.Query", "ProposalsByGroupPolicy"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// VoteByProposalVoter queries a vote by proposal id and voter.
        pub async fn vote_by_proposal_voter(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryVoteByProposalVoterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryVoteByProposalVoterResponse>,
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
                "/cosmos.group.v1.Query/VoteByProposalVoter",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.group.v1.Query", "VoteByProposalVoter"));
            self.inner.unary(req, path, codec).await
        }
        /// VotesByProposal queries a vote by proposal.
        pub async fn votes_by_proposal(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryVotesByProposalRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryVotesByProposalResponse>,
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
                "/cosmos.group.v1.Query/VotesByProposal",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.group.v1.Query", "VotesByProposal"));
            self.inner.unary(req, path, codec).await
        }
        /// VotesByVoter queries a vote by voter.
        pub async fn votes_by_voter(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryVotesByVoterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryVotesByVoterResponse>,
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
                "/cosmos.group.v1.Query/VotesByVoter",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.group.v1.Query", "VotesByVoter"));
            self.inner.unary(req, path, codec).await
        }
        /// GroupsByMember queries groups by member address.
        pub async fn groups_by_member(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGroupsByMemberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGroupsByMemberResponse>,
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
                "/cosmos.group.v1.Query/GroupsByMember",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.group.v1.Query", "GroupsByMember"));
            self.inner.unary(req, path, codec).await
        }
        /// TallyResult returns the tally result of a proposal. If the proposal is
        /// still in voting period, then this query computes the current tally state,
        /// which might not be final. On the other hand, if the proposal is final,
        /// then it simply returns the `final_tally_result` state stored in the
        /// proposal itself.
        pub async fn tally_result(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTallyResultRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTallyResultResponse>,
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
                "/cosmos.group.v1.Query/TallyResult",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.group.v1.Query", "TallyResult"));
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
        /// GroupInfo queries group info based on group id.
        async fn group_info(
            &self,
            request: tonic::Request<super::QueryGroupInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGroupInfoResponse>,
            tonic::Status,
        >;
        /// GroupPolicyInfo queries group policy info based on account address of group policy.
        async fn group_policy_info(
            &self,
            request: tonic::Request<super::QueryGroupPolicyInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGroupPolicyInfoResponse>,
            tonic::Status,
        >;
        /// GroupMembers queries members of a group
        async fn group_members(
            &self,
            request: tonic::Request<super::QueryGroupMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGroupMembersResponse>,
            tonic::Status,
        >;
        /// GroupsByAdmin queries groups by admin address.
        async fn groups_by_admin(
            &self,
            request: tonic::Request<super::QueryGroupsByAdminRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGroupsByAdminResponse>,
            tonic::Status,
        >;
        /// GroupPoliciesByGroup queries group policies by group id.
        async fn group_policies_by_group(
            &self,
            request: tonic::Request<super::QueryGroupPoliciesByGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGroupPoliciesByGroupResponse>,
            tonic::Status,
        >;
        /// GroupsByAdmin queries group policies by admin address.
        async fn group_policies_by_admin(
            &self,
            request: tonic::Request<super::QueryGroupPoliciesByAdminRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGroupPoliciesByAdminResponse>,
            tonic::Status,
        >;
        /// Proposal queries a proposal based on proposal id.
        async fn proposal(
            &self,
            request: tonic::Request<super::QueryProposalRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryProposalResponse>,
            tonic::Status,
        >;
        /// ProposalsByGroupPolicy queries proposals based on account address of group policy.
        async fn proposals_by_group_policy(
            &self,
            request: tonic::Request<super::QueryProposalsByGroupPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryProposalsByGroupPolicyResponse>,
            tonic::Status,
        >;
        /// VoteByProposalVoter queries a vote by proposal id and voter.
        async fn vote_by_proposal_voter(
            &self,
            request: tonic::Request<super::QueryVoteByProposalVoterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryVoteByProposalVoterResponse>,
            tonic::Status,
        >;
        /// VotesByProposal queries a vote by proposal.
        async fn votes_by_proposal(
            &self,
            request: tonic::Request<super::QueryVotesByProposalRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryVotesByProposalResponse>,
            tonic::Status,
        >;
        /// VotesByVoter queries a vote by voter.
        async fn votes_by_voter(
            &self,
            request: tonic::Request<super::QueryVotesByVoterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryVotesByVoterResponse>,
            tonic::Status,
        >;
        /// GroupsByMember queries groups by member address.
        async fn groups_by_member(
            &self,
            request: tonic::Request<super::QueryGroupsByMemberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGroupsByMemberResponse>,
            tonic::Status,
        >;
        /// TallyResult returns the tally result of a proposal. If the proposal is
        /// still in voting period, then this query computes the current tally state,
        /// which might not be final. On the other hand, if the proposal is final,
        /// then it simply returns the `final_tally_result` state stored in the
        /// proposal itself.
        async fn tally_result(
            &self,
            request: tonic::Request<super::QueryTallyResultRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTallyResultResponse>,
            tonic::Status,
        >;
    }
    /// Query is the cosmos.group.v1 Query service.
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
                "/cosmos.group.v1.Query/GroupInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GroupInfoSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryGroupInfoRequest>
                    for GroupInfoSvc<T> {
                        type Response = super::QueryGroupInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGroupInfoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).group_info(request).await };
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
                        let method = GroupInfoSvc(inner);
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
                "/cosmos.group.v1.Query/GroupPolicyInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GroupPolicyInfoSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryGroupPolicyInfoRequest>
                    for GroupPolicyInfoSvc<T> {
                        type Response = super::QueryGroupPolicyInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGroupPolicyInfoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).group_policy_info(request).await
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
                        let method = GroupPolicyInfoSvc(inner);
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
                "/cosmos.group.v1.Query/GroupMembers" => {
                    #[allow(non_camel_case_types)]
                    struct GroupMembersSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryGroupMembersRequest>
                    for GroupMembersSvc<T> {
                        type Response = super::QueryGroupMembersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGroupMembersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).group_members(request).await
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
                        let method = GroupMembersSvc(inner);
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
                "/cosmos.group.v1.Query/GroupsByAdmin" => {
                    #[allow(non_camel_case_types)]
                    struct GroupsByAdminSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryGroupsByAdminRequest>
                    for GroupsByAdminSvc<T> {
                        type Response = super::QueryGroupsByAdminResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGroupsByAdminRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).groups_by_admin(request).await
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
                        let method = GroupsByAdminSvc(inner);
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
                "/cosmos.group.v1.Query/GroupPoliciesByGroup" => {
                    #[allow(non_camel_case_types)]
                    struct GroupPoliciesByGroupSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryGroupPoliciesByGroupRequest,
                    > for GroupPoliciesByGroupSvc<T> {
                        type Response = super::QueryGroupPoliciesByGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryGroupPoliciesByGroupRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).group_policies_by_group(request).await
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
                        let method = GroupPoliciesByGroupSvc(inner);
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
                "/cosmos.group.v1.Query/GroupPoliciesByAdmin" => {
                    #[allow(non_camel_case_types)]
                    struct GroupPoliciesByAdminSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryGroupPoliciesByAdminRequest,
                    > for GroupPoliciesByAdminSvc<T> {
                        type Response = super::QueryGroupPoliciesByAdminResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryGroupPoliciesByAdminRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).group_policies_by_admin(request).await
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
                        let method = GroupPoliciesByAdminSvc(inner);
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
                "/cosmos.group.v1.Query/Proposal" => {
                    #[allow(non_camel_case_types)]
                    struct ProposalSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryProposalRequest>
                    for ProposalSvc<T> {
                        type Response = super::QueryProposalResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryProposalRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).proposal(request).await };
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
                        let method = ProposalSvc(inner);
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
                "/cosmos.group.v1.Query/ProposalsByGroupPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct ProposalsByGroupPolicySvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryProposalsByGroupPolicyRequest,
                    > for ProposalsByGroupPolicySvc<T> {
                        type Response = super::QueryProposalsByGroupPolicyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryProposalsByGroupPolicyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).proposals_by_group_policy(request).await
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
                        let method = ProposalsByGroupPolicySvc(inner);
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
                "/cosmos.group.v1.Query/VoteByProposalVoter" => {
                    #[allow(non_camel_case_types)]
                    struct VoteByProposalVoterSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryVoteByProposalVoterRequest>
                    for VoteByProposalVoterSvc<T> {
                        type Response = super::QueryVoteByProposalVoterResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryVoteByProposalVoterRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).vote_by_proposal_voter(request).await
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
                        let method = VoteByProposalVoterSvc(inner);
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
                "/cosmos.group.v1.Query/VotesByProposal" => {
                    #[allow(non_camel_case_types)]
                    struct VotesByProposalSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryVotesByProposalRequest>
                    for VotesByProposalSvc<T> {
                        type Response = super::QueryVotesByProposalResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryVotesByProposalRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).votes_by_proposal(request).await
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
                        let method = VotesByProposalSvc(inner);
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
                "/cosmos.group.v1.Query/VotesByVoter" => {
                    #[allow(non_camel_case_types)]
                    struct VotesByVoterSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryVotesByVoterRequest>
                    for VotesByVoterSvc<T> {
                        type Response = super::QueryVotesByVoterResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryVotesByVoterRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).votes_by_voter(request).await
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
                        let method = VotesByVoterSvc(inner);
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
                "/cosmos.group.v1.Query/GroupsByMember" => {
                    #[allow(non_camel_case_types)]
                    struct GroupsByMemberSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryGroupsByMemberRequest>
                    for GroupsByMemberSvc<T> {
                        type Response = super::QueryGroupsByMemberResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGroupsByMemberRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).groups_by_member(request).await
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
                        let method = GroupsByMemberSvc(inner);
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
                "/cosmos.group.v1.Query/TallyResult" => {
                    #[allow(non_camel_case_types)]
                    struct TallyResultSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryTallyResultRequest>
                    for TallyResultSvc<T> {
                        type Response = super::QueryTallyResultResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryTallyResultRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).tally_result(request).await
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
                        let method = TallyResultSvc(inner);
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
        const NAME: &'static str = "cosmos.group.v1.Query";
    }
}
/// GenesisState defines the group module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// group_seq is the group table orm.Sequence,
    /// it is used to get the next group ID.
    #[prost(uint64, tag = "1")]
    pub group_seq: u64,
    /// groups is the list of groups info.
    #[prost(message, repeated, tag = "2")]
    pub groups: ::prost::alloc::vec::Vec<GroupInfo>,
    /// group_members is the list of groups members.
    #[prost(message, repeated, tag = "3")]
    pub group_members: ::prost::alloc::vec::Vec<GroupMember>,
    /// group_policy_seq is the group policy table orm.Sequence,
    /// it is used to generate the next group policy account address.
    #[prost(uint64, tag = "4")]
    pub group_policy_seq: u64,
    /// group_policies is the list of group policies info.
    #[prost(message, repeated, tag = "5")]
    pub group_policies: ::prost::alloc::vec::Vec<GroupPolicyInfo>,
    /// proposal_seq is the proposal table orm.Sequence,
    /// it is used to get the next proposal ID.
    #[prost(uint64, tag = "6")]
    pub proposal_seq: u64,
    /// proposals is the list of proposals.
    #[prost(message, repeated, tag = "7")]
    pub proposals: ::prost::alloc::vec::Vec<Proposal>,
    /// votes is the list of votes.
    #[prost(message, repeated, tag = "8")]
    pub votes: ::prost::alloc::vec::Vec<Vote>,
}

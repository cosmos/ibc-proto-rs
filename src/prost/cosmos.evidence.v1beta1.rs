/// Equivocation implements the Evidence interface and defines evidence of double
/// signing misbehavior.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Equivocation {
    #[prost(int64, tag="1")]
    pub height: i64,
    #[prost(message, optional, tag="2")]
    pub time: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
    #[prost(int64, tag="3")]
    pub power: i64,
    #[prost(string, tag="4")]
    pub consensus_address: ::prost::alloc::string::String,
}

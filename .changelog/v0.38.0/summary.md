*October 19th, 2023*

This release contains a breaking change, where the `Protobuf` trait is not
object-safe any longer, but rather re-exported from the `tendermint-proto` crate.

It also updates the Interchain Security protos to include misbehaviour-related messages.

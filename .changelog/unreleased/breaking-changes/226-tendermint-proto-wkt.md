- Use well-know types from `tendermint-proto` instead of bundling them

  Use the `google.protobuf.{Duration, Timestamp}` Protobuf messages
  exposed by `tendermint-proto` instead of defining and bundling our own.
  ([\#226](https://github.com/cosmos/ibc-proto-rs/pull/226))
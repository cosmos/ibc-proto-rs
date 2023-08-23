- Fix token transfer deserialization for cases with no memo by deriving
  `Default` serde for the memo field in `MsgTransfer` and
  `FungibleTokenPacketData` types.
  ([#119](https://github.com/cosmos/ibc-proto-rs/pull/119))

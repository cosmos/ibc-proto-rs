- Use the v0.34 definition of `abci.Event` which does not enforce 
  valid UTF-8 data for its `key` and `value` attributes, specifying 
  them as `bytes` instead of `string`. ([#180](https://github.com/cosmos/ibc-proto-rs/issues/180))

  This is required, because ibc-go emits event attributes which are not valid UTF-8,
  so we need to use this definition to be able to parse them.

  In Protobuf, `bytes` and `string` are wire-compatible, so doing this strictly increases the amount fo data we can parse.

  See this Hermes PR for background information: https://github.com/informalsystems/hermes/pull/3768

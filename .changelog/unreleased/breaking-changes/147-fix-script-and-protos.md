- Since ibc-proto v0.34.0, the script in charge of generating the Rust proto definitions
  has been mistakenly checking out their latest version instead of the one
  specified in the corresponding `src/*_COMMIT` file. This has now been fixed
  and the protos have therefore been downgraded to their proper versions:
  * IBC-Go: v7.3.0,
  * Cosmos SDK: v0.47.5 
  * Interchain Security: v3.1.0
  ([\#147](https://github.com/cosmos/ibc-proto-rs/pull/147))

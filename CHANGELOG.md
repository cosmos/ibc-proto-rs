# CHANGELOG

## v0.24.0

*December 13th, 2022*

This release updates the Tendermint Protobuf definitons to v0.28.0.

### BREAKING CHANGES

- Update to tendermint-proto 0.28 ([#45](https://github.com/cosmos/ibc-proto-rs/issues/45))

## v0.23.0

*November 29th, 2022*

This release updates the Tendermint Protobuf definitons to v0.27.0.

### BREAKING CHANGES

- Update to tendermint-proto 0.27 ([#40](https://github.com/cosmos/ibc-proto-rs/pull/40))

## v0.22.0

*November 9, 2022*

This release updates the Cosmos SDK protobufs to v0.46.4.

### BREAKING CHANGES

- Update tendermint-rs libraries to v0.26
  ([#33](https://github.com/cosmos/ibc-proto-rs/issues/33))
- Update protobufs for Cosmos SDK to v0.46.4
  - Adds the `module_account_by_name` method to the `Query` trait
  ([#2776](https://github.com/informalsystems/hermes/2776))

## v0.21.0

*October 19, 2022*

This is the first release of ibc-proto with its own changelog. For past releases, please check the [Hermes](https://github.com/informalsystems/hermes/blob/c34b354e310da7f59631ae315ea22c5f2b420d44/CHANGELOG.md) changelog.

### BREAKING CHANGES

- Update protos to IBC-Go v5.0.0 and Cosmos SDK v0.46.1
  ([#24](https://github.com/cosmos/ibc-proto-rs/issues/24))
- Update tendermint-proto requirement from =0.23.9 to =0.25.0
  ([#26](https://github.com/cosmos/ibc-proto-rs/issues/26))

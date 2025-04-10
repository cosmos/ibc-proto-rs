# CHANGELOG

# v0.52.0

*April 4th, 2025*

This release bumps the `tonic` and `cosmos-sdk-proto` dependencies.

### FEATURES

- Bump `tonic` to v0.13 and `cosmos-sdk-proto` to v0.27 ([\#260](https://github.com/cosmos/ibc-proto-rs/pull/260))

## v0.51.1

*November 8th, 2024*

This patch release enables the `json-schema` feature for the `tendermint-proto` dependency.

### BUG FIXES

- Enable `json-schema` for `tendermint-proto` dependency.
  ([\#252](https://github.com/cosmos/ibc-proto-rs/issues/252))

## v0.51.0

*October 24th, 2024*

This patch release updates `cosmos-sdk-proto` to `v0.26.0`.

### BREAKING CHANGES

- Update `cosmos-sdk-proto` to `v0.26.0`
  ([\#249](https://github.com/cosmos/ibc-proto-rs/issues/249))

## v0.50.0

*October 23rd, 2024*

This release updates `interchain-security` to `v6.1.0` and `tendermint-proto` to `v0.40.0`.

### BREAKING CHANGES

- Update `interchain-security` to v6.1.0
  ([\#245](https://github.com/cosmos/ibc-proto-rs/issues/245))
- Update `tendermint-proto` to `v0.40.0`
  ([\#246](https://github.com/cosmos/ibc-proto-rs/issues/246))

## v0.49.1

*October 16th, 2024*

This patch release re-exports Google protobuf types from `tendermint_proto` for
easier use by downstream projects.

### FEATURES

- Re-export Google proto types from the `tendermint_proto` for added convenience
  ([\#242](https://github.com/cosmos/ibc-proto-rs/pull/242))

## v0.49.0

*September 24th, 2024*

Starting from this release, Protobuf messages for the Cosmos SDK are not longer included in this crate,
but rather are now re-exported from the [`cosmos-sdk-proto`](http://crates.io/crates/cosmos-sdk-proto) crate.

Moreover, the generated transport code for `tonic` is now feature-gated under the `transport` feature,
which is enabled by default.

### BREAKING CHANGES

- Cosmos SDK protos are now re-exported from the `cosmos_sdk_proto`
  crate instead of being generated as part of `ibc-proto`
  ([\#187](https://github.com/cosmos/ibc-proto-rs/pull/187))

### FEATURES

- Feature-gate generated `tonic` transport code behind `transport` feature
  ([\#237](https://github.com/cosmos/ibc-proto-rs/issues/237))

## v0.48.0

*September 5th, 2024*

This release updates `tendermint-proto` to v0.39 and now uses
the `google.protobuf.{Duration, Timestamp}` Protobuf messages
exposed by `tendermint-proto` instead of defining and bundling our own.

### BREAKING CHANGES

- Update `tendermint-proto` to v0.39 ([\#227](https://github.com/cosmos/ibc-proto-rs/pull/227))
- Use the `google.protobuf.{Duration, Timestamp}` Protobuf messages
  exposed by `tendermint-proto` instead of defining and bundling our own.
  ([\#226](https://github.com/cosmos/ibc-proto-rs/pull/226))

## v0.47.1

*September 3rd, 2024*

This release exports the `tendermint_proto::Error` type.

### IMPROVEMENTS

- Export `tendermint_proto::Error` ([#229](https://github.com/cosmos/ibc-proto-rs/pull/229))

## v0.47.0

*July 18th, 2024*

This release bumps `tendermint-proto` to v0.38, `prost` to v0.13 and `tonic` to v0.12.

### BREAKING CHANGES

- Update `tendermint-proto` to v0.38 ([#223](https://github.com/cosmos/ibc-proto-rs/pull/223))
- Update `prost` to v0.13 and `tonic` to v0.12 ([#223](https://github.com/cosmos/ibc-proto-rs/pull/223))

## v0.46.0

*June 3rd, 2024*

This release only bumps `tendermint-proto` to v0.37.

### BREAKING CHANGES

- Update `tendermint-proto` to v0.37 ([#215](https://github.com/cosmos/ibc-proto-rs/issues/215))

## v0.45.0

*May 30th, 2024*

This release updates `tonic` to v0.11.0 and `borsh` to v1.

### BREAKING CHANGES

- Update `tonic` to v0.11.0 ([#195](https://github.com/cosmos/ibc-proto-rs/pull/195))
- Update `borsh` to v1 ([\#210](https://github.com/cosmos/ibc-proto-rs/pull/210))

## v0.44.0

*April 25th, 2024*

This release updates `tendermint-proto` to v0.36.0.

### BREAKING CHANGES

- Update `tendermint-proto` to v0.36.0 ([\#208](https://github.com/cosmos/ibc-proto-rs/issues/208))

## v0.43.0

*April 22nd, 2024*

This release updates `tendermint-proto` to v0.35.0 and `tonic` to v0.11.0.

### BREAKING CHANGES

- Update `tendermint-proto` to v0.35.0 ([\#200](https://github.com/cosmos/ibc-proto-rs/pull/200))
- Update `tonic` to v0.11 ([\#207](https://github.com/cosmos/ibc-proto-rs/pull/207))

## v0.42.2

*March 14th, 2024*

This release only updates the Protobuf messages to ibc-go v8.1.1, which brings back the `ibc.lightclients.wasm.v1` protos.

### FEATURES

- Update to ibc-go v8.1.1 which brings back the `ibc.lightclients.wasm.v1`
  protos ([\#201](https://github.com/cosmos/ibc-proto-rs/pull/201))

## v0.42.0

*February 8th, 2024*

This release updates the proto types for IBC-Go from `v8.0.0` to `v8.1.0`.
This includes proto types used for channel upgrade.

Please note that IBC-Go v8.1.0 doesn't provide the Wasm light client proto types anymore (`ibc.lightclients.wasm.v1`). These types will live in
ibc-proto-rs versions `v0.41.x`

### BREAKING CHANGES

- Removed WASM light client proto types
  ([\#192](https://github.com/cosmos/ibc-proto-rs/pull/192))
- Use the v0.34 definition of `abci.Event` which does not enforce
  valid UTF-8 data for its `key` and `value` attributes, specifying
  them as `bytes` instead of `string`. ([#180](https://github.com/cosmos/ibc-proto-rs/issues/180))

  This is required, because ibc-go emits event attributes which are not valid UTF-8,
  so we need to use this definition to be able to parse them.

  In Protobuf, `bytes` and `string` are wire-compatible, so doing this strictly increases the amount fo data we can parse.

  See this Hermes PR for background information: https://github.com/informalsystems/hermes/pull/3768

### FEATURES

- Bump IBC-Go to v8.1.0 ([\#192](https://github.com/cosmos/ibc-proto-rs/pull/192))

### IMPROVEMENTS

- Extend `ibc::mock::ClientState` with `trusting_period` and `frozen` data
  fields to cover a wider range of client state testing scenarios
  ([\#186](https://github.com/cosmos/ibc-proto-rs/issues/186)).

## v0.41.0

*January 9th, 2024*

This release integrates the ICS-08 WASM light client and ICS-721 NFT transfer
application proto types. It additionally resolves the removal of the `#[no_std]`
attribute, introduces the `informalsystems-pbjson/std` dependency for std
feature compatibility.

### BUG FIXES

- Fix `#[no_std]` attribute removal and add `informalsystems-pbjson/std` dependency
  for `std` feature ([\#171](https://github.com/cosmos/ibc-proto-rs/issues/171)).

### FEATURES

- Add ICS-721 NFT transfer application proto types
  ([\#167](https://github.com/cosmos/ibc-proto-rs/issues/167)).
- Integrate WASM light client proto types by updating `IBC_GO_COMMIT` to the
  hash associated with the ibc-go `wasm-v8.0.0` tag
  ([\#168](https://github.com/cosmos/ibc-proto-rs/issues/168)).

## v0.40.0

*December 29th, 2023*

### BREAKING CHANGES

- Added ProtoJSON support.
  The `serde` feature flag now abides by [Protobuf JSON rules](https://protobuf.dev/programming-guides/proto3/#json)
  when it comes to JSON serialization/deserialization.
  ([#166](https://github.com/cosmos/ibc-proto-rs/pull/166))

## v0.39.1

*November 22nd, 2023*

### FEATURES

- Derive the `prost::Name` trait for all Protobuf messages
  ([\#163](https://github.com/cosmos/ibc-proto-rs/pull/163))

## v0.39.0

*November 15th, 2023*

This release updates the ibc-go version used for the ibc protos to version v8.0.0
which contains the new messages `MsgRecoverClient` and `MsgIBCSoftwareUpgrade`.

### FEATURES

- Bump ibc-go to v8.0.0
  ([\#161](https://github.com/cosmos/ibc-proto-rs/pull/161))

## v0.38.0

*October 19th, 2023*

This release contains a breaking change, where the `Protobuf` trait is not
object-safe any longer, but rather re-exported from the `tendermint-proto` crate.

It also updates the Interchain Security protos to include misbehaviour-related messages.

### BREAKING CHANGES

- Switch from using object safe `Protobuf` definitions and re-export
 `Protobuf` from `tendermint-proto` crate` instead.
 ([#116](https://github.com/cosmos/ibc-proto-rs/issues/116))

### FEATURES

- Update CCV provider protos to include misbehaviour-related messages
  ([\#113](https://github.com/cosmos/ibc-proto-rs/issues/113))

## v0.37.1

*October 10th, 2023*

This releases adds `JsonSchema` derivation for the `Any` type.

### FEATURES

- Implement `JsonSchema` for the `Any` type
  ([#156](https://github.com/cosmos/ibc-proto-rs/issues/156))

## v0.37.0

*October 4th, 2023*

This release updates `prost` to v0.12 and `tonic` to v0.10,
and makes `serde` an optional dependency, now only enabled with the `serde` feature.

### FEATURES

- Update `prost` to v0.12 and `tonic` to v0.10
  ([\#145](https://github.com/cosmos/ibc-proto-rs/issues/145))

### IMPROVEMENTS

- Make `serde` an optional dependency, now only enabled with the `serde` feature
  ([\#152](https://github.com/cosmos/ibc-proto-rs/pull/152))

## v0.36.1

*September 28th, 2023*

This release adds Protobuf definitions from the `cosmos.crypto.*` package.

### FEATURES

- Add Protobuf definitions from `cosmos.crypto.*` ([\#149](https://github.com/cosmos/ibc-proto-rs/pull/149))

## v0.36.0

*September 28th, 2023*

Warning: This release downgrades the Protobuf definitions for IBC-Go, Cosmos SDK, and Interchain Security.

### BREAKING CHANGES

- Since ibc-proto v0.34.0, the script in charge of generating the Rust proto definitions
  has been mistakenly checking out their latest version instead of the one
  specified in the corresponding `src/*_COMMIT` file. This has now been fixed
  and the protos have therefore been downgraded to their proper versions:
  * IBC-Go: v7.3.0,
  * Cosmos SDK: v0.47.5
  * Interchain Security: v3.1.0
  ([\#147](https://github.com/cosmos/ibc-proto-rs/pull/147))

## v0.35.0

*September 14th, 2023*

This release updates the IBC-Go protos to v7.3.0, the Cosmos SDK protos to v0.47.5,
and the Interchain Security protos to v3.1.0.

It also adds back the `ibc.lightclients.localhost.v1` proto definition that was
removed in IBC-Go v7.0.0, for users which may need to interact with v1 localhost clients.

### FEATURES

- Bump ibc-go to v7.3.0, Cosmos SDK to 0.47.5 and Interchain Security to v3.1.0
  ([\#140](https://github.com/cosmos/ibc-proto-rs/pull/140))
- Add proto definition for `ibc.lightclients.localhost.v1`
  ([\#143](https://github.com/cosmos/ibc-proto-rs/pull/143))

## v0.34.1

*August 29th, 2023*

This release updates the `borsh` dependency to v0.10.

## v0.34.0

*August 17th, 2023*

This release updates the Cosmos SDK protos to v0.47.3 and IBC-Go protos to v7.2.0.

Additionally, it restore `no_std` support for JSON serialization via `serde`.
Previously, `Serialize` and `Deserialize` instances were only derived when
the `std` feature was enabled, but that is no longer required.

As such, they now require the `serde` feature to be enabled, independently of
whether or not the `std` feature is enabled.

### BUG FIXES

- Restore `no_std` support for JSON serialization
  ([\#98](https://github.com/cosmos/ibc-proto-rs/issues/98))

### FEATURES

- Update Cosmos SDK protos to v0.47.3 and IBC-Go protos to v7.2.0
  ([\#129](https://github.com/cosmos/ibc-proto-rs/issues/129))

## v0.33.0

*Aug 16th, 2023*

This release bumps `tendermint-rs` to `0.33.0`.

## v0.32.1

*July 31st, 2023*

This release includes default `serde` value for token transfer memos and fixes
the issue with deserialization of `MsgTransfer` and `FungibleTokenPacketData`
types when no memo is present.

### BUG FIXES

- Fix token transfer deserialization for cases with no memo by deriving
  `Default` serde for the memo field in `MsgTransfer` and
  `FungibleTokenPacketData` types.
  ([#119](https://github.com/cosmos/ibc-proto-rs/pull/119))

## v0.32.0

*June 14th, 2023*

This release updates the `ibc-go` proto files from version `v5.1.0` to `v6.2.0` which includes the ICA Protobuf definitions.

The version of `borsh` has been downgraded from `v0.10.0` to `v0.9` in order to match `near-sdk-rs` and Solana smart contract frame `anchor`.

### BUG FIXES

- Downgrade `borsh` version from `v0.10.0` to `v0.9`
  ([#106](https://github.com/cosmos/ibc-proto-rs/pull/104))

### IMPROVEMENTS

- Update `ibc-go` commit from `v5.1.0` to `v6.2.0`
  ([#106](https://github.com/cosmos/ibc-proto-rs/issues/106))

## v0.31.0

*May 31st, 2023*

This is the final release of `ibc-proto` v0.31.0.

There are no changes from v0.31.0-alpha.2.

For the differences since v0.30.0, please see the changelog entries for v0.31.0-alpha.1 and v0.31.0-alpha.1.

> **Warning**
> This release removes support for `Serialize` and `Deserailize` trait impls being available in `no_std` context.
> See the release notes below and associated issues for more details.

## v0.31.0-alpha.2

*May 3rd, 2023*

This is the second alpha release of `ibc-proto` v0.31.0.

It only updates the `tendermint-proto` version to v0.32.0.

### BREAKING CHANGES

- Update `tendermint-proto` to v0.32.0
  ([\#99](https://github.com/cosmos/ibc-proto-rs/issues/99))

## v0.31.0-alpha.1

*May 1st, 2023*

This is the first alpha release of `ibc-proto` v0.31.0.

The proto definitions for `cosmos.ics23.v1` messages are now re-exported from the [`ics23`](https://crates.io/crates/ics23) crate under both the `ibc_proto::cosmos::ics23::v1` >
The latter will removed in a subsequent release.

This is nonetheless a breaking change as it may break compilation or trigger warnings in code which relied on these definitions being different than the ones in `ics23`.

Moreover, because the code generated by `pbjson-build` is not `no_std` compatible, the serde annotations on the generated protos are only enabled when the `std` feature of `ibc-proto` is enabled.

### BREAKING CHANGES

- Re-export the `ics23.cosmos.v1` Protobuf definitions from the `ics23` crate instead of including them directly in this crate.
  The proto definitions are exported both under the `ibc_proto::cosmos::ics23::v1` module and under the `ibc_proto::ics23` module
  in an attempt to preserve backward source compatiblity.
  This is a breaking change as it may break compilation or trigger warnings
  in code which relied on these definitions being different than the ones in `ics23`.
  ([\#10](https://github.com/cosmos/ibc-proto-rs/issues/10))

- Derive `Serialize` and `Deserialize` impls compatible with ProtoJSON using `pbjson-build`.
  This is a breaking change because the code generated by `pbjson-build` is not
  `no_std` compatible. Therefore, the `serde` annotations on the generated protos
  are only enabled when the `std` feature of `ibc-proto` is enabled.
  ([\#95](https://github.com/cosmos/ibc-proto-rs/issues/95))

## v0.30.0

*April 20th, 2023*

This release updates `tendermint-proto` to v0.31.

### BREAKING CHANGES

- Update `tendermint-proto` to v0.31.x
  ([\#90](https://github.com/cosmos/ibc-proto-rs/pull/90))

## v0.29.0

*April 12th, 2023*

In this update, Protobuf definitions have been included for Interchain Security v1 CCV within
the `ibc_proto::interchain_security::ccv` module.

It should also be noted that the return type of `Protobuf::encode{,_length_delimited}_vec`
has been modified from `Result<Vec<u8>, Error>` to `Vec<u8>`.

Furthermore, the version of `tonic` has been raised from 0.8 to 0.9.

### BREAKING CHANGES

- Remove errors for `encode_vec` and `encode_length_delimited_vec` in
  `Protobuf` ([#73](https://github.com/cosmos/ibc-proto-rs/issues/73))
- Update `tonic` to 0.9 and re-generate the protos
  ([\#79](https://github.com/cosmos/ibc-proto-rs/issues/79))

### FEATURES

- Add Interchain Security v1 CCV Protobuf definitions
  ([\#76](https://github.com/cosmos/ibc-proto-rs/issues/76))

### BUG FIXES

- Automatically patch the generated Rust code for it to compile
  ([\#2](https://github.com/cosmos/ibc-proto-rs/issues/2))

## v0.28.0

*March 10th, 2023*

This release updates the `ibc-go` proto files from version `v5.0.0` to `v5.1.0`.

This includes the `memo` field in the following struct:

* `ibc.applications.transfer.v1 MsgTransfer`
* `ibc.applications.transfer.v2 FungibleTokenPacketData`

As well as the `sequence` field in:

* `ibc.applications.transfer.v1 MsgTransferResponse`

### IMPROVEMENTS

- Update `ibc-go` commit from `v5.0.0` to `v5.1.0`
([#71](https://github.com/cosmos/ibc-proto-rs/issues/71))

## v0.27.0

*March 7th, 2023*

This release updates the `tendermint-proto` crate to v0.30.0.

At the moment, only the Tendermint Protobuf definitions for CometBFT 0.37 are exported
and supported. In the future, side-by-side support for 0.34 and 0.37 definitions may be provided.

### BREAKING CHANGE

- Update `tendermint-proto` to v0.30.0 ([#64](https://github.com/cosmos/ibc-proto-rs/issues/64))

## v0.26.0

*February 17, 2023*

This release updates tendermint protobuf defintions to `v0.29.0`.

## v0.25.0

*February 9th, 2023*

This release updates borsh to v0.10.0 and fixes a typo in borsh deserialization of `Any`
([#59](https://github.com/cosmos/ibc-proto-rs/pull/59)).

## v0.24.1

*January 10th, 2023*

This release adds `parity-scale-codec` and `borsh` serialize/deserialize for the `Any` type.

### FEATURES

- Add parity-scale-codec and borsh for Any ([#47](https://github.com/cosmos/ibc-
  proto-rs/issues/47))

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

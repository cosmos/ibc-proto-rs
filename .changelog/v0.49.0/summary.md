*September 24th, 2024*

Starting from this release, Protobuf messages for the Cosmos SDK are not longer included in this crate,
but rather are now re-exported from the [`cosmos-sdk-proto`](http://crates.io/crates/cosmos-sdk-proto) crate.

Moreover, the generated transport code for `tonic` is now feature-gated under the `transport` feature,
which is enabled by default.

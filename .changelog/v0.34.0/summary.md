This release updates the Cosmos SDK protos to v0.47.3 and IBC-Go protos to v7.2.0.

Additionally, it restore `no_std` support for JSON serialization via `serde`.
Previously, `Serialize` and `Deserialize` instances were only derived when
the `std` feature was enabled, but that is no longer required.

As such, they now require the `serde` feature to be enabled, independently of
whether or not the `std` feature is enabled.

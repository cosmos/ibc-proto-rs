*January 8th, 2024*

This release integrates the ICS-08 Wasm light client and ICS-721 NFT transfer
application proto types. It additionally resolves the removal of the `#[no_std]`
attribute, introduces the `informalsystems-pbjson/std` dependency for std
feature compatibility, and includes the `counterparty_upgrade_sequence` field in
the `MsgTimeoutOnClose` and `MsgChannelCloseConfirm`, optimizing for the channel
upgradability support.

//! ibc-proto library gives the developer access to the Cosmos SDK IBC proto-defined structs.

// Todo: automate the creation of this module setup based on the dots in the filenames.
// This module setup is necessary because the generated code contains "super::" calls for dependencies.
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(warnings, trivial_casts, trivial_numeric_casts, unused_import_braces)]
#![allow(
    clippy::large_enum_variant,
    clippy::derive_partial_eq_without_eq,
    clippy::needless_borrows_for_generic_args
)]
#![allow(rustdoc::bare_urls)]
#![forbid(unsafe_code)]

pub use tendermint_proto::Error;
pub use tendermint_proto::Protobuf;

extern crate alloc;

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate core as std;

#[macro_export]
macro_rules! include_proto {
    ($path:literal) => {
        include!(concat!("prost/", $path));
    };
}

/// The version (commit hash) of IBC Go used when generating this library.
pub const IBC_GO_COMMIT: &str = include_str!("IBC_GO_COMMIT");

/// The version (commit hash) of Interchain Security used when generating this library.
pub const INTERCHAIN_SECURITY_COMMIT: &str = include_str!("INTERCHAIN_SECURITY_COMMIT");

/// The version (commit hash) of nft-transfer used when generating this library.
pub const NFT_TRANSFER_COMMIT: &str = include_str!("NFT_TRANSFER_COMMIT");

/// File descriptor set of compiled proto.
#[cfg(feature = "proto-descriptor")]
pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("prost/proto_descriptor.bin");

// Re-export Cosmos SDK protos from the `cosmos_sdk_proto` crate
pub use cosmos_sdk_proto::cosmos;

// Re-export the ICS23 proto from the `ics23` crate
pub use ics23;

pub mod ibc {
    #[deprecated(since = "0.15.0", note = "Use `ibc_proto::ibc::applications` instead")]
    pub mod apps {
        pub use super::applications::*;
    }
    pub mod applications {
        pub mod transfer {
            pub mod v1 {
                include_proto!("ibc.applications.transfer.v1.rs");
                #[cfg(feature = "serde")]
                include_proto!("ibc.applications.transfer.v1.serde.rs");
            }
            pub mod v2 {
                include_proto!("ibc.applications.transfer.v2.rs");
                #[cfg(feature = "serde")]
                include_proto!("ibc.applications.transfer.v2.serde.rs");
            }
        }
        pub mod fee {
            pub mod v1 {
                include_proto!("ibc.applications.fee.v1.rs");
                #[cfg(feature = "serde")]
                include_proto!("ibc.applications.fee.v1.serde.rs");
            }
        }
        pub mod interchain_accounts {
            pub mod v1 {
                include_proto!("ibc.applications.interchain_accounts.v1.rs");
                #[cfg(feature = "serde")]
                include_proto!("ibc.applications.interchain_accounts.v1.serde.rs");
            }
            pub mod controller {
                pub mod v1 {
                    include_proto!("ibc.applications.interchain_accounts.controller.v1.rs");
                    #[cfg(feature = "serde")]
                    include_proto!("ibc.applications.interchain_accounts.controller.v1.serde.rs");
                }
            }
            pub mod host {
                pub mod v1 {
                    include_proto!("ibc.applications.interchain_accounts.host.v1.rs");
                    #[cfg(feature = "serde")]
                    include_proto!("ibc.applications.interchain_accounts.host.v1.serde.rs");
                }
            }
        }
        pub mod nft_transfer {
            pub mod v1 {
                include_proto!("ibc.applications.nft_transfer.v1.rs");
                #[cfg(feature = "serde")]
                include_proto!("ibc.applications.nft_transfer.v1.serde.rs");
            }
        }
    }
    pub mod core {
        pub mod channel {
            pub mod v1 {
                include_proto!("ibc.core.channel.v1.rs");
                #[cfg(feature = "serde")]
                include_proto!("ibc.core.channel.v1.serde.rs");
            }
        }
        pub mod client {
            pub mod v1 {
                include_proto!("ibc.core.client.v1.rs");
                #[cfg(feature = "serde")]
                include_proto!("ibc.core.client.v1.serde.rs");
            }
        }
        pub mod commitment {
            pub mod v1 {
                include_proto!("ibc.core.commitment.v1.rs");
                #[cfg(feature = "serde")]
                include_proto!("ibc.core.commitment.v1.serde.rs");
            }
        }
        pub mod connection {
            pub mod v1 {
                include_proto!("ibc.core.connection.v1.rs");
                #[cfg(feature = "serde")]
                include_proto!("ibc.core.connection.v1.serde.rs");
            }
        }
        pub mod types {
            pub mod v1 {
                include_proto!("ibc.core.types.v1.rs");
                #[cfg(feature = "serde")]
                include_proto!("ibc.core.types.v1.serde.rs");
            }
        }
    }
    pub mod lightclients {
        pub mod localhost {
            pub mod v1 {
                include_proto!("ibc.lightclients.localhost.v1.rs");
                #[cfg(feature = "serde")]
                include_proto!("ibc.lightclients.localhost.v1.serde.rs");
            }
            pub mod v2 {
                include_proto!("ibc.lightclients.localhost.v2.rs");
                #[cfg(feature = "serde")]
                include_proto!("ibc.lightclients.localhost.v2.serde.rs");
            }
        }
        pub mod solomachine {
            pub mod v3 {
                include_proto!("ibc.lightclients.solomachine.v3.rs");
                #[cfg(feature = "serde")]
                include_proto!("ibc.lightclients.solomachine.v3.serde.rs");
            }
        }
        pub mod tendermint {
            pub mod v1 {
                include_proto!("ibc.lightclients.tendermint.v1.rs");
                #[cfg(feature = "serde")]
                include_proto!("ibc.lightclients.tendermint.v1.serde.rs");
            }
        }
        pub mod wasm {
            pub mod v1 {
                include_proto!("ibc.lightclients.wasm.v1.rs");
                #[cfg(feature = "serde")]
                include_proto!("ibc.lightclients.wasm.v1.serde.rs");
            }
        }
    }
    pub mod mock {
        include_proto!("ibc.mock.rs");
        #[cfg(feature = "serde")]
        include_proto!("ibc.mock.serde.rs");
    }
}

pub mod interchain_security {
    pub mod ccv {
        #[allow(clippy::match_single_binding)]
        pub mod v1 {
            include_proto!("interchain_security.ccv.v1.rs");
        }
        pub mod provider {
            pub mod v1 {
                include_proto!("interchain_security.ccv.provider.v1.rs");
            }
        }
        pub mod consumer {
            pub mod v1 {
                include_proto!("interchain_security.ccv.consumer.v1.rs");
            }
        }
    }
}

pub mod stride {
    pub mod interchainquery {
        pub mod v1 {
            include_proto!("stride.interchainquery.v1.rs");
        }
    }
}

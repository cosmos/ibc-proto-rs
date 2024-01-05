//! ibc-proto library gives the developer access to the Cosmos SDK IBC proto-defined structs.

// Todo: automate the creation of this module setup based on the dots in the filenames.
// This module setup is necessary because the generated code contains "super::" calls for dependencies.
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(warnings, trivial_casts, trivial_numeric_casts, unused_import_braces)]
#![allow(clippy::large_enum_variant, clippy::derive_partial_eq_without_eq)]
#![allow(rustdoc::bare_urls)]
#![forbid(unsafe_code)]

pub mod google;

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

/// The version (commit hash) of the Cosmos SDK used when generating this library.
pub const COSMOS_SDK_COMMIT: &str = include_str!("COSMOS_SDK_COMMIT");

/// The version (commit hash) of IBC Go used when generating this library.
pub const IBC_GO_COMMIT: &str = include_str!("IBC_GO_COMMIT");

/// The version (commit hash) of Interchain Security used when generating this library.
pub const INTERCHAIN_SECURITY_COMMIT: &str = include_str!("INTERCHAIN_SECURITY_COMMIT");

/// File descriptor set of compiled proto.
#[cfg(feature = "proto-descriptor")]
pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("prost/proto_descriptor.bin");

pub mod cosmos {
    pub mod app {
        pub mod v1alpha1 {
            include_proto!("cosmos.app.v1alpha1.rs");
            #[cfg(feature = "serde")]
            include_proto!("cosmos.app.v1alpha1.serde.rs");
        }
    }
    pub mod auth {
        pub mod v1beta1 {
            include_proto!("cosmos.auth.v1beta1.rs");
            #[cfg(feature = "serde")]
            include_proto!("cosmos.auth.v1beta1.serde.rs");
            /// EthAccount defines an Ethermint account.
            /// TODO: remove when/if a canonical `EthAccount`
            /// lands in the next Cosmos SDK release
            /// (note https://github.com/cosmos/cosmos-sdk/pull/9981
            /// only adds the PubKey type)
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct EthAccount {
                #[prost(message, optional, tag = "1")]
                pub base_account: ::core::option::Option<BaseAccount>,
                #[prost(bytes = "vec", tag = "2")]
                pub code_hash: ::prost::alloc::vec::Vec<u8>,
            }
        }
        pub mod module {
            pub mod v1 {
                include_proto!("cosmos.auth.module.v1.rs");
                #[cfg(feature = "serde")]
                include_proto!("cosmos.auth.module.v1.serde.rs");
            }
        }
    }
    pub mod staking {
        pub mod v1beta1 {
            include_proto!("cosmos.staking.v1beta1.rs");
            #[cfg(feature = "serde")]
            include_proto!("cosmos.staking.v1beta1.serde.rs");
        }
        pub mod module {
            pub mod v1 {
                include_proto!("cosmos.staking.module.v1.rs");
                #[cfg(feature = "serde")]
                include_proto!("cosmos.staking.module.v1.serde.rs");
            }
        }
    }
    pub mod bank {
        pub mod v1beta1 {
            include_proto!("cosmos.bank.v1beta1.rs");
            #[cfg(feature = "serde")]
            include_proto!("cosmos.bank.v1beta1.serde.rs");
        }
        pub mod module {
            pub mod v1 {
                include_proto!("cosmos.bank.module.v1.rs");
                #[cfg(feature = "serde")]
                include_proto!("cosmos.bank.module.v1.serde.rs");
            }
        }
    }
    pub mod base {
        pub mod abci {
            pub mod v1beta1 {
                include_proto!("cosmos.base.abci.v1beta1.rs");
            }
        }
        pub mod node {
            pub mod v1beta1 {
                include_proto!("cosmos.base.node.v1beta1.rs");
                #[cfg(feature = "serde")]
                include_proto!("cosmos.base.node.v1beta1.serde.rs");
            }
        }
        pub mod query {
            pub mod v1beta1 {
                include_proto!("cosmos.base.query.v1beta1.rs");
                #[cfg(feature = "serde")]
                include_proto!("cosmos.base.query.v1beta1.serde.rs");
            }
        }
        pub mod reflection {
            pub mod v1beta1 {
                include_proto!("cosmos.base.reflection.v1beta1.rs");
                #[cfg(feature = "serde")]
                include_proto!("cosmos.base.reflection.v1beta1.serde.rs");
            }
        }
        pub mod v1beta1 {
            include_proto!("cosmos.base.v1beta1.rs");
            #[cfg(feature = "serde")]
            include_proto!("cosmos.base.v1beta1.serde.rs");
        }
        pub mod tendermint {
            pub mod v1beta1 {
                include_proto!("cosmos.base.tendermint.v1beta1.rs");
            }
        }
        pub mod kv {
            pub mod v1beta1 {
                include_proto!("cosmos.base.kv.v1beta1.rs");
                #[cfg(feature = "serde")]
                include_proto!("cosmos.base.kv.v1beta1.serde.rs");
            }
        }
        pub mod snapshots {
            pub mod v1beta1 {
                include_proto!("cosmos.base.snapshots.v1beta1.rs");
                #[cfg(feature = "serde")]
                include_proto!("cosmos.base.snapshots.v1beta1.serde.rs");
            }
        }
    }
    pub mod crypto {
        pub mod multisig {
            pub mod v1beta1 {
                include_proto!("cosmos.crypto.multisig.v1beta1.rs");
                #[cfg(feature = "serde")]
                include_proto!("cosmos.crypto.multisig.v1beta1.serde.rs");
            }
            include_proto!("cosmos.crypto.multisig.rs");
            #[cfg(feature = "serde")]
            include_proto!("cosmos.crypto.multisig.serde.rs");
        }
        pub mod ed25519 {
            include_proto!("cosmos.crypto.ed25519.rs");
            #[cfg(feature = "serde")]
            include_proto!("cosmos.crypto.ed25519.serde.rs");
        }
        pub mod secp256k1 {
            include_proto!("cosmos.crypto.secp256k1.rs");
            #[cfg(feature = "serde")]
            include_proto!("cosmos.crypto.secp256k1.serde.rs");
        }
        pub mod secp256r1 {
            include_proto!("cosmos.crypto.secp256r1.rs");
            #[cfg(feature = "serde")]
            include_proto!("cosmos.crypto.secp256r1.serde.rs");
        }
        pub mod keyring {
            pub mod v1 {
                include_proto!("cosmos.crypto.keyring.v1.rs");
                #[cfg(feature = "serde")]
                include_proto!("cosmos.crypto.keyring.v1.serde.rs");
            }
        }
        pub mod hd {
            pub mod v1 {
                include_proto!("cosmos.crypto.hd.v1.rs");
                #[cfg(feature = "serde")]
                include_proto!("cosmos.crypto.hd.v1.serde.rs");
            }
        }
    }
    pub mod tx {
        pub mod config {
            pub mod v1 {
                include_proto!("cosmos.tx.config.v1.rs");
                #[cfg(feature = "serde")]
                include_proto!("cosmos.tx.config.v1.serde.rs");
            }
        }
        pub mod signing {
            pub mod v1beta1 {
                include_proto!("cosmos.tx.signing.v1beta1.rs");
                #[cfg(feature = "serde")]
                include_proto!("cosmos.tx.signing.v1beta1.serde.rs");
            }
        }
        pub mod v1beta1 {
            include_proto!("cosmos.tx.v1beta1.rs");
        }
    }
    pub mod upgrade {
        pub mod v1beta1 {
            include_proto!("cosmos.upgrade.v1beta1.rs");
            #[cfg(feature = "serde")]
            include_proto!("cosmos.upgrade.v1beta1.serde.rs");
        }
        pub mod module {
            pub mod v1 {
                include_proto!("cosmos.upgrade.module.v1.rs");
                #[cfg(feature = "serde")]
                include_proto!("cosmos.upgrade.module.v1.serde.rs");
            }
        }
    }
    pub mod gov {
        pub mod v1 {
            include_proto!("cosmos.gov.v1.rs");
            #[cfg(feature = "serde")]
            include_proto!("cosmos.gov.v1.serde.rs");
        }
        pub mod v1beta1 {
            include_proto!("cosmos.gov.v1beta1.rs");
            #[cfg(feature = "serde")]
            include_proto!("cosmos.gov.v1beta1.serde.rs");
        }
        pub mod module {
            pub mod v1 {
                include_proto!("cosmos.gov.module.v1.rs");
                #[cfg(feature = "serde")]
                include_proto!("cosmos.gov.module.v1.serde.rs");
            }
        }
    }
    pub mod ics23 {
        pub use ics23 as v1;
    }
}

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
    }
    pub mod mock {
        include_proto!("ibc.mock.rs");
        #[cfg(feature = "serde")]
        include_proto!("ibc.mock.serde.rs");
    }
}

// Re-export `ics23` definitions for backward compatibility
pub use ics23;

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

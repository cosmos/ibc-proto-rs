use std::path::{Path, PathBuf};
use std::process;

use similar::TextDiff;
use walkdir::WalkDir;

use argh::FromArgs;
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "compile")]
/// Compile
pub struct CompileCmd {
    #[argh(option, short = 'i')]
    /// path to the IBC-Go proto files
    ibc: PathBuf,

    #[argh(option, short = 's')]
    /// path to the Cosmos SDK proto files
    sdk: PathBuf,

    #[argh(option, short = 'c')]
    /// path to the Cosmos ICS proto files
    ics: PathBuf,

    #[argh(option, short = 'n')]
    /// path to the nft-transfer proto files
    nft: PathBuf,

    #[argh(option, short = 'o')]
    /// path to output the generated Rust sources into
    out: PathBuf,
}

impl CompileCmd {
    pub fn run(&self) {
        self.compile_ibc_protos().unwrap_or_else(|e| {
            eprintln!("[error] failed to compile protos: {}", e);
            process::exit(1);
        });

        Self::patch_generated_files(self.out.as_ref()).unwrap_or_else(|e| {
            eprintln!("[error] failed to patch generated files: {}", e);
            process::exit(1);
        });

        Self::build_pbjson_impls(self.out.as_ref()).unwrap_or_else(|e| {
            eprintln!("[error] failed to build pbjson impls: {}", e);
            process::exit(1);
        });

        println!("[info ] Done!");
    }

    fn compile_ibc_protos(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "[info ] Compiling IBC .proto files to Rust into '{}'...",
            self.out.display()
        );

        let root = Path::new(env!("CARGO_MANIFEST_DIR"));

        // Paths
        let proto_paths = [
            root.join("../../definitions/mock"),
            root.join("../../definitions/ibc/lightclients/localhost/v1"),
            root.join("../../definitions/stride/interchainquery/v1"),
            self.ibc.join("ibc"),
            self.sdk.join("cosmos/auth"),
            self.sdk.join("cosmos/gov"),
            self.sdk.join("cosmos/tx"),
            self.sdk.join("cosmos/base"),
            self.sdk.join("cosmos/crypto"),
            self.sdk.join("cosmos/bank"),
            self.sdk.join("cosmos/staking"),
            self.sdk.join("cosmos/upgrade"),
            self.ics.join("interchain_security/ccv/v1"),
            self.ics.join("interchain_security/ccv/provider"),
            self.ics.join("interchain_security/ccv/consumer"),
            self.nft.join("ibc"),
        ];

        let proto_includes_paths = [
            self.sdk.to_path_buf(),
            self.ibc.to_path_buf(),
            self.ics.to_path_buf(),
            self.nft.to_path_buf(),
            root.join("../../definitions/mock"),
            root.join("../../definitions/ibc/lightclients/localhost/v1"),
            root.join("../../definitions/stride/interchainquery/v1"),
        ];

        // List available proto files
        let mut protos: Vec<PathBuf> = vec![];
        for proto_path in &proto_paths {
            println!("Looking for proto files in '{}'", proto_path.display());
            protos.append(
                &mut WalkDir::new(proto_path)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| {
                        e.file_type().is_file()
                            && e.path().extension().is_some()
                            && e.path().extension().unwrap() == "proto"
                    })
                    .map(|e| e.into_path())
                    .collect(),
            );
        }

        println!("Found the following protos:");
        // Show which protos will be compiled
        for proto in &protos {
            println!("\t-> {}", proto.display());
        }
        println!("[info ] Compiling..");

        let attrs_jsonschema = r#"#[cfg_attr(all(feature = "json-schema", feature = "serde"), derive(::schemars::JsonSchema))]"#;
        let attrs_jsonschema_str = r#"#[cfg_attr(all(feature = "json-schema", feature = "serde"), schemars(with = "String"))]"#;

        let attrs_ord = "#[derive(Eq, PartialOrd, Ord)]";
        let attrs_eq = "#[derive(Eq)]";

        // Automatically derive a `prost::Name` implementation.
        let mut config = prost_build::Config::new();
        config.enable_type_names();

        tonic_build::configure()
            .build_client(true)
            .compile_well_known_types(true)
            .client_mod_attribute(".", r#"#[cfg(feature = "client")]"#)
            .build_server(true)
            .server_mod_attribute(".", r#"#[cfg(feature = "server")]"#)
            .out_dir(&self.out)
            .file_descriptor_set_path(self.out.join("proto_descriptor.bin"))
            // Use the v0.34 definition of `abci.Event` which does not enforce valid UTF-8 data
            // for its `key` and `value` attributes, specifying them as `bytes` instead of `string`.
            // This is required, because ibc-go emits event attributes which are not valid UTF-8
            // (sounds like a bug if it's up to CometBFT 0.38?),
            // so we need to use this definition to be able to parse them.
            // In Protobuf, `bytes` and `string` are wire-compatible, so doing this strictly
            // increases the amount fo data we can parse.
            .extern_path(
                ".tendermint.abci.Event",
                "::cometbft_proto::abci::v1beta1::Event",
            )
            // All other types should be up to CometBFT 0.38
            .extern_path(".tendermint.abci.Validator", "::cometbft_proto::abci::v1beta1::Validator")
            .extern_path(".tendermint.abci.ValidatorUpdate", "::cometbft_proto::abci::v1beta1::ValidatorUpdate")
            .extern_path(".tendermint.abci", "::cometbft_proto::abci::v1beta3")
            .extern_path(".tendermint.crypto", "::cometbft_proto::crypto::v1")
            .extern_path(".tendermint.p2p", "::cometbft_proto::p2p::v1")
            .extern_path(".tendermint.types", "::cometbft_proto::types::v1")
            .extern_path(".tendermint.version", "::cometbft_proto::version::v1")
            .extern_path(".ics23", "::ics23")
            .type_attribute(".google.protobuf.Any", attrs_eq)
            .type_attribute(".google.protobuf.Any", attrs_jsonschema)
            .type_attribute(".google.protobuf.Duration", attrs_eq)
            .type_attribute(".ibc.core.client.v1.Height", attrs_ord)
            .type_attribute(".ibc.core.client.v1.Height", attrs_jsonschema)
            .type_attribute(".ibc.core.commitment.v1.MerkleRoot", attrs_jsonschema)
            .field_attribute(
                ".ibc.core.commitment.v1.MerkleRoot.hash",
                attrs_jsonschema_str,
            )
            .type_attribute(".ibc.core.commitment.v1.MerklePrefix", attrs_jsonschema)
            .field_attribute(
                ".ibc.core.commitment.v1.MerklePrefix.key_prefix",
                attrs_jsonschema_str,
            )
            .type_attribute(".ibc.core.channel.v1.Channel", attrs_jsonschema)
            .type_attribute(".ibc.core.channel.v1.Counterparty", attrs_jsonschema)
            .type_attribute(".ibc.core.connection.v1.ConnectionEnd", attrs_jsonschema)
            .type_attribute(".ibc.core.connection.v1.Counterparty", attrs_jsonschema)
            .type_attribute(".ibc.core.connection.v1.Version", attrs_jsonschema)
            .type_attribute(".ibc.lightclients.wasm.v1.ClientMessage", attrs_jsonschema)
            .compile_with_config(config, &protos, &proto_includes_paths)?;

        println!("[info ] Protos compiled successfully");

        Ok(())
    }

    fn build_pbjson_impls(out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("[info] Building pbjson Serialize, Deserialize impls...");
        let descriptor_set_path = out_dir.join("proto_descriptor.bin");
        let descriptor_set = std::fs::read(descriptor_set_path)?;

        pbjson_build::Builder::new()
            .register_descriptors(&descriptor_set)?
            .out_dir(&out_dir)
            .exclude([
                // The validator patch is not compatible with protojson builds
                ".cosmos.staking.v1beta1.StakeAuthorization",
                ".cosmos.staking.v1beta1.ValidatorUpdates",
                // TODO: These have dependencies on tendermint-proto, which does not implement protojson.
                //       After it's implemented there, we can delete these exclusions.
                ".cosmos.base.abci.v1beta1",
                ".cosmos.tx.v1beta1",
                ".cosmos.base.tendermint.v1beta1",
                ".interchain_security.ccv.v1",
                ".interchain_security.ccv.provider.v1",
                ".interchain_security.ccv.consumer.v1",
                ".stride.interchainquery.v1",
            ])
            .emit_fields()
            .build(&[
                ".ibc",
                ".cosmos",
                ".interchain_security",
                ".stride",
                ".google",
            ])?;

        Ok(())
    }

    fn patch_generated_files(out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "[info ] Patching generated files in '{}'...",
            out_dir.display()
        );

        const PATCHES: &[(&str, &[(&str, &str)])] = &[
            (
                "cosmos.staking.v1beta1.rs",
                &[
                    ("pub struct Validators", "pub struct ValidatorsVec"),
                    (
                        "impl ::prost::Name for Validators {",
                        "impl ::prost::Name for ValidatorsVec {",
                    ),
                    ("AllowList(Validators)", "AllowList(ValidatorsVec)"),
                    ("DenyList(Validators)", "DenyList(ValidatorsVec)"),
                ],
            ),
            (
                "ibc.applications.transfer.v1.rs",
                &[(
                    "The denomination trace (\\[port_id\\]/[channel_id])+/\\[denom\\]",
                    "The denomination trace `([port_id]/[channel_id])+/[denom]`",
                )],
            ),
            (
                "ibc.applications.nft_transfer.v1.rs",
                &[(
                    "The class trace (\\[port_id\\]/[channel_id])+/\\[denom\\]",
                    "The class trace `([port_id]/[channel_id])+/[class]`",
                )],
            ),
        ];

        for (file, patches) in PATCHES {
            println!("[info ] Patching {file}...");

            let path = out_dir.join(file);
            let original = std::fs::read_to_string(&path)?;
            let mut patched = original.clone();

            for (before, after) in patches.iter() {
                patched = patched.replace(before, after);
            }

            let diff = TextDiff::from_lines(&original, &patched);
            println!("{}", diff.unified_diff().context_radius(3));

            std::fs::write(&path, patched)?;
        }

        Ok(())
    }
}

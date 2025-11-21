use std::path::{Path, PathBuf};
use std::{fs, process};

use regex::Regex;
use similar::TextDiff;
use walkdir::WalkDir;

use argh::FromArgs;
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "compile")]
/// Compile
pub struct CompileCmd {
    #[argh(switch, short = 't')]
    /// generate transport client/server code
    transport: bool,

    #[argh(option, short = 'i')]
    /// path to the IBC-Go proto files
    ibc: PathBuf,

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
        Self::compile_ibc_protos(
            self.transport,
            self.ibc.as_ref(),
            self.ics.as_ref(),
            self.nft.as_ref(),
            self.out.as_ref(),
        )
        .unwrap_or_else(|e| {
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

    fn compile_ibc_protos(
        transport: bool,
        ibc_dir: &Path,
        ics_dir: &Path,
        nft_dir: &Path,
        out_dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "[info ] Compiling IBC .proto files to Rust into '{}'...",
            out_dir.display()
        );

        let root = Path::new(env!("CARGO_MANIFEST_DIR"));

        // Paths
        let proto_paths = [
            root.join("../../definitions/mock"),
            root.join("../../definitions/ibc/lightclients/localhost/v1"),
            root.join("../../definitions/stride/interchainquery/v1"),
            ibc_dir.join("ibc"),
            ics_dir.join("interchain_security/ccv/v1"),
            ics_dir.join("interchain_security/ccv/provider"),
            ics_dir.join("interchain_security/ccv/consumer"),
            nft_dir.join("ibc"),
        ];

        let proto_includes_paths = [
            ibc_dir.to_path_buf(),
            ics_dir.to_path_buf(),
            nft_dir.to_path_buf(),
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

        // Automatically derive a `prost::Name` implementation.
        let mut config = prost_build::Config::new();
        config.enable_type_names();

        tonic_build::configure()
            .build_transport(transport)
            .build_client(true)
            .compile_well_known_types(true)
            .client_mod_attribute(".", r#"#[cfg(feature = "client")]"#)
            .build_server(true)
            .server_mod_attribute(".", r#"#[cfg(feature = "server")]"#)
            .out_dir(out_dir)
            .file_descriptor_set_path(out_dir.join("proto_descriptor.bin"))
            // Use the v0.34 definition of `abci.Event` which does not enforce valid UTF-8 data
            // for its `key` and `value` attributes, specifying them as `bytes` instead of `string`.
            // This is required, because ibc-go emits event attributes which are not valid UTF-8,
            // so we need to use this definition to be able to parse them.
            // In Protobuf, `bytes` and `string` are wire-compatible, so doing this strictly
            // increases the amount of data we can parse.
            .extern_path(
                ".tendermint.abci.Event",
                "::tendermint_proto::v0_34::abci::Event",
            )
            .extern_path(".tendermint", "::tendermint_proto")
            .extern_path(".cometbft.types.v1", "::tendermint_proto::types")
            .extern_path(".cosmos.ics23.v1", "::ics23")
            .extern_path(".google.protobuf", "::tendermint_proto::google::protobuf")
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
            .compile_protos_with_config(config, &protos, &proto_includes_paths)?;

        println!("[info ] Protos compiled successfully");

        Ok(())
    }

    fn build_pbjson_impls(out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("[info ] Building pbjson Serialize, Deserialize impls...");

        let descriptor_set_path = out_dir.join("proto_descriptor.bin");
        let descriptor_set = std::fs::read(descriptor_set_path)?;

        pbjson_build::Builder::new()
            .register_descriptors(&descriptor_set)?
            .out_dir(out_dir)
            .exclude([
                ".interchain_security.ccv.v1",
                ".interchain_security.ccv.provider.v1",
                ".interchain_security.ccv.consumer.v1",
                ".stride.interchainquery.v1",
            ])
            .emit_fields()
            .build(&[".ibc", ".interchain_security", ".stride"])?;

        Ok(())
    }

    fn patch_generated_files(out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "[info ] Patching generated files in '{}'...",
            out_dir.display()
        );

        const PATCHES: &[(&str, &[(&str, &str)])] = &[
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

        // patches applied to all generated files
        println!("applying global patches");
        const GLOBAL_REPLACEMENTS: &[(&str, &str)] = &[
            // Feature-gate gRPC impls which use `tonic::transport`
            (
                "impl(.+)tonic::transport(.+)",
                "#[cfg(feature = \"transport\")]\n    \
                impl${1}tonic::transport${2}",
            ),
        ];

        let files_iter = WalkDir::new(out_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| e.path().extension() == Some("rs".as_ref()));

        for file in files_iter {
            println!("patching file: {:?}", file.path());
            let mut contents = fs::read_to_string(file.path())?;

            for &(regex, replacement) in GLOBAL_REPLACEMENTS {
                contents = Regex::new(regex)
                    .unwrap_or_else(|_| panic!("invalid regex: {}", regex))
                    .replace_all(&contents, replacement)
                    .to_string();
            }

            fs::write(file.path(), contents)?;
        }
        println!("finished global patches");

        Ok(())
    }
}

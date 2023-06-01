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

    #[argh(option, short = 'o')]
    /// path to output the generated Rust sources into
    out: PathBuf,
}

impl CompileCmd {
    pub fn run(&self) {
        Self::compile_ibc_protos(
            self.ibc.as_ref(),
            self.sdk.as_ref(),
            self.ics.as_ref(),
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

        println!("[info ] Done!");
    }

    fn compile_ibc_protos(
        ibc_dir: &Path,
        sdk_dir: &Path,
        ics_dir: &Path,
        out_dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "[info ] Compiling IBC .proto files to Rust into '{}'...",
            out_dir.display()
        );

        let root = env!("CARGO_MANIFEST_DIR");

        // Paths
        let proto_paths = [
            format!("{}/../../definitions/mock", root),
            format!("{}/../../definitions/stride/interchainquery/v1", root),
            format!("{}/ibc", ibc_dir.display()),
            format!("{}/cosmos/auth", sdk_dir.display()),
            format!("{}/cosmos/gov", sdk_dir.display()),
            format!("{}/cosmos/tx", sdk_dir.display()),
            format!("{}/cosmos/base", sdk_dir.display()),
            format!("{}/cosmos/bank", sdk_dir.display()),
            format!("{}/cosmos/staking", sdk_dir.display()),
            format!("{}/cosmos/upgrade", sdk_dir.display()),
            format!("{}/interchain_security/ccv/v1", ics_dir.display()),
            format!("{}/interchain_security/ccv/provider", ics_dir.display()),
            format!("{}/interchain_security/ccv/consumer", ics_dir.display()),
        ];

        let proto_includes_paths = [
            format!("{}", sdk_dir.display()),
            format!("{}", ibc_dir.display()),
            format!("{}", ics_dir.display()),
            format!("{}/../../definitions/mock", root),
            format!("{}/../../definitions/stride/interchainquery/v1", root),
        ];

        // List available proto files
        let mut protos: Vec<PathBuf> = vec![];
        for proto_path in &proto_paths {
            println!("Looking for proto files in {:?}", proto_path);
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
            println!("\t-> {:?}", proto);
        }
        println!("[info ] Compiling..");

        // List available paths for dependencies
        let includes: Vec<PathBuf> = proto_includes_paths.iter().map(PathBuf::from).collect();

        // We can only enable JSON serialization when the `std` feature is enabled,
        // as it is currently required by `ics23` for it to implement JSON serialization
        // let attrs_serde = r#"#[derive(::serde::Serialize, ::serde::Deserialize)]"#;
        // let attrs_serde_default = r#"#[serde(default)]"#;
        // let attrs_jsonschema =
        //     r#"#[cfg_attr(feature = "json-schema", derive(::schemars::JsonSchema))]"#;

        let attrs_jsonschema = r#"#[cfg_attr(all(feature = "json-schema", feature = "std"), derive(::schemars::JsonSchema))]"#;
        let attrs_ord = "#[derive(Eq, PartialOrd, Ord)]";
        let attrs_eq = "#[derive(Eq)]";
        let attrs_serde =
            r#"#[cfg_attr(feature = "std", derive(::serde::Serialize, ::serde::Deserialize))]"#;
        let attrs_serde_default = r#"#[cfg_attr(feature = "std", serde(default))]"#;
        let attrs_serde_base64 = r#"#[cfg_attr(feature = "std", serde(with = "crate::base64"))]"#;
        let attrs_jsonschema_str = r#"#[cfg_attr(all(feature = "json-schema", feature = "std"), schemars(with = "String"))]"#;

        tonic_build::configure()
            .build_client(true)
            .compile_well_known_types(true)
            .client_mod_attribute(".", r#"#[cfg(feature = "client")]"#)
            .build_server(true)
            .server_mod_attribute(".", r#"#[cfg(feature = "server")]"#)
            .out_dir(out_dir)
            .extern_path(".tendermint", "::tendermint_proto")
            .extern_path(".ics23", "::ics23")
            .type_attribute(".google.protobuf.Any", attrs_serde)
            .type_attribute(".google.protobuf.Any", attrs_eq)
            .type_attribute(".google.protobuf.Timestamp", attrs_serde)
            .type_attribute(".google.protobuf.Duration", attrs_serde)
            .type_attribute(".google.protobuf.Duration", attrs_eq)
            .type_attribute(".ibc.core.client.v1", attrs_serde)
            .type_attribute(".ibc.core.client.v1.Height", attrs_ord)
            .type_attribute(".ibc.core.client.v1.Height", attrs_jsonschema)
            .field_attribute(".ibc.core.client.v1.Height", attrs_serde_default)
            .type_attribute(".ibc.core.commitment.v1", attrs_serde)
            .type_attribute(".ibc.core.commitment.v1.MerkleRoot", attrs_jsonschema)
            .field_attribute(
                ".ibc.core.commitment.v1.MerkleRoot.hash",
                attrs_serde_base64,
            )
            .field_attribute(
                ".ibc.core.commitment.v1.MerkleRoot.hash",
                attrs_jsonschema_str,
            )
            .type_attribute(".ibc.core.commitment.v1.MerklePrefix", attrs_jsonschema)
            .field_attribute(
                ".ibc.core.commitment.v1.MerklePrefix.key_prefix",
                attrs_serde_base64,
            )
            .field_attribute(
                ".ibc.core.commitment.v1.MerklePrefix.key_prefix",
                attrs_jsonschema_str,
            )
            .type_attribute(".ibc.core.channel.v1", attrs_serde)
            .type_attribute(".ibc.core.channel.v1.Channel", attrs_jsonschema)
            .type_attribute(".ibc.core.channel.v1.Counterparty", attrs_jsonschema)
            .type_attribute(".ibc.core.connection.v1", attrs_serde)
            .type_attribute(".ibc.core.connection.v1.ConnectionEnd", attrs_jsonschema)
            .type_attribute(".ibc.core.connection.v1.Counterparty", attrs_jsonschema)
            .type_attribute(".ibc.core.connection.v1.Version", attrs_jsonschema)
            .type_attribute(".ibc.core.types.v1", attrs_serde)
            .type_attribute(".ibc.applications.transfer.v1", attrs_serde)
            .type_attribute(".ibc.applications.transfer.v2", attrs_serde)
            .type_attribute(
                ".ibc.applications.interchain_accounts.controller.v1",
                attrs_serde,
            )
            .type_attribute(
                ".ibc.applications.interchain_accounts.genesis.v1",
                attrs_serde,
            )
            .type_attribute(".ibc.applications.interchain_accounts.host.v1", attrs_serde)
            .type_attribute(".ibc.applications.interchain_accounts.v1", attrs_serde)
            .type_attribute(".cosmos.auth.v1beta1", attrs_serde)
            .type_attribute(".cosmos.bank.v1beta1", attrs_serde)
            .type_attribute(".cosmos.base.v1beta1", attrs_serde)
            .type_attribute(".cosmos.base.query.v1beta1", attrs_serde)
            .type_attribute(".cosmos.upgrade.v1beta1", attrs_serde)
            .compile(&protos, &includes)?;

        println!("[info ] Protos compiled successfully");

        Ok(())
    }

    fn patch_generated_files(out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "[info ] Patching generated files in '{}'...",
            out_dir.display()
        );

        {
            println!("[info ] Patching cosmos.staking.v1beta1.rs...");

            let path = out_dir.join("cosmos.staking.v1beta1.rs");
            let contents = std::fs::read_to_string(&path)?;

            let patched_contents = contents
                .replace("pub struct Validators", "pub struct ValidatorsVec")
                .replace("AllowList(Validators)", "AllowList(ValidatorsVec)")
                .replace("DenyList(Validators)", "DenyList(ValidatorsVec)");

            let diff = TextDiff::from_lines(&contents, &patched_contents);
            println!("{}", diff.unified_diff().context_radius(3));

            std::fs::write(&path, patched_contents)?;
        }

        Ok(())
    }
}

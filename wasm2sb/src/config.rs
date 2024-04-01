use std::{
    path::PathBuf,
    process::{Command, Stdio},
    vec,
};

use cargo_metadata::{CargoOpt, Message, MetadataCommand};
use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use eyre::{Context, Result};
use path_slash::PathBufExt as _;

/// Command line arguments
#[derive(Parser, Debug)]
// #[clap(name = "wasm2sb", author, version, about, long_about = None, arg_required_else_help(true))]
#[clap(author, version, about, long_about = None, arg_required_else_help(true))]
pub struct CommandLineArgs {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    /// Convert package to Scratch project
    Package {
        /// Package with the target to run
        #[clap(help_heading = "Package Selection")]
        #[arg(short, long, default_value = "./.")]
        package: String,

        /// debug
        #[arg(short, long, default_value = "false")]
        debug: bool,

        #[command(flatten)]
        common_args: Arg,
    },

    /// Convert wasm to Scratch project
    Wasm {
        /// Wasm file with the target to run
        #[clap(help_heading = "Wasm Selection")]
        wasm: String,

        #[command(flatten)]
        common_args: Arg,
    },
}

#[derive(Args, Debug)]
pub struct Arg {
    /// output file
    #[arg(short, long, default_value = "output.sb3")]
    pub output: PathBuf,

    /// quiet mode
    #[arg(short, long, default_value = "false")]
    pub quiet: bool,

    /// verbose mode
    #[arg(short, long, default_value = "false")]
    pub verbose: bool,

    /// input file
    #[arg(short, long)]
    pub input: Option<PathBuf>,
}

impl CommandLineArgs {
    pub fn parse_and_check() -> Result<(Self, PathBuf)> {
        let opt = CommandLineArgs::parse();

        match &opt.command {
            SubCommands::Package {
                package,
                debug,
                common_args,
            } => {
                // let package = match PathBuf::from(package).canonicalize() {
                //     Ok(path) => path,
                //     Err(e) => panic!("Failed to canonicalize path: {:?}", e),
                // };
                let package = PathBuf::from(package);

                let metadata = MetadataCommand::new()
                    .manifest_path(package.join("Cargo.toml").to_slash().unwrap().to_string())
                    .features(CargoOpt::AllFeatures)
                    .exec()
                    .wrap_err("failed to find Cargo.toml")?;

                let mut options = vec!["build", "--message-format=json-render-diagnostics"];
                if !debug {
                    options.push("--release");
                }
                options.push("--target=wasm32-unknown-unknown");

                if common_args.quiet {
                    let mut command = Command::new("cargo")
                        .args(options)
                        .current_dir(&package)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                        .unwrap();

                    let reader = std::io::BufReader::new(command.stdout.take().unwrap());

                    println!("{}{:?}", "Building package: ".green(), package);

                    for message in cargo_metadata::Message::parse_stream(reader) {
                        match message.unwrap() {
                            Message::CompilerMessage(_) => {}
                            Message::CompilerArtifact(_) => {}
                            Message::BuildScriptExecuted(_) => {}
                            Message::BuildFinished(_) => {}
                            _ => (), // Unknown message
                        }
                    }
                }

                println!("{}", "Build finished\n".green());

                let path = if *debug {
                    metadata
                        .target_directory
                        .join("wasm32-unknown-unknown/debug/wasm_sb_bindgen_testcode.wasm")
                } else {
                    metadata
                        .target_directory
                        .join("wasm32-unknown-unknown/release/wasm_sb_bindgen_testcode.wasm")
                };

                Ok((opt, path.into()))
            }
            SubCommands::Wasm {
                wasm,
                common_args: _common_args,
            } => {
                let wasm_path = PathBuf::from(&wasm);
                if !wasm_path.exists() {
                    return Err(eyre::eyre!("Wasm file not found: {:?}", wasm_path));
                }
                Ok((opt, wasm_path))
            }
        }
    }
}

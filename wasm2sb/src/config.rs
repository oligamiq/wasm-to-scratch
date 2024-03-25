use clap::Parser;

// Command line arguments
#[derive(Parser, Debug)]
#[clap(name = "wasm2sb", version, about, long_about = None)]
pub struct CommandLineArgs {
    /// target wasm file
    #[arg(short, long)]
    pub file: String,

    /// output file
    #[arg(short, long, default_value = "output.sb3")]
    pub output: String
}

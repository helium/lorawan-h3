use anyhow::Result;
use clap::Parser;
mod index;
mod params;

#[derive(Debug, clap::Parser)]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Helium LoRaWAN Region Generator")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, clap::Subcommand)]

enum Cmd {
    Index(index::Cmd),
    Params(params::Cmd),
}

impl Cmd {
    fn run(&self) -> Result<()> {
        match self {
            Self::Index(cmd) => cmd.run(),
            Self::Params(cmd) => cmd.run(),
        }
    }
}

impl Cli {
    pub fn run(self) -> Result<()> {
        self.cmd.run()
    }
}

fn main() -> Result<()> {
    simple_logger::init_with_env()?;
    let cli = Cli::parse();
    cli.run()
}

pub(crate) fn print_json<T: ?Sized + serde::Serialize>(value: &T) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

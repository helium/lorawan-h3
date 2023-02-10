use anyhow::Result;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use helium_proto::{BlockchainRegionParamsV1, Message};
use std::{
    fs,
    io::{Read, Write},
    path,
};

/// Commands on region indexes
#[derive(Debug, clap::Args)]
pub struct Cmd {
    #[command(subcommand)]
    cmd: ParamsCmd,
}

impl Cmd {
    pub fn run(&self) -> Result<()> {
        self.cmd.run()
    }
}

#[derive(Debug, clap::Subcommand)]
pub enum ParamsCmd {
    Generate(Generate),
    Export(Export),
}

impl ParamsCmd {
    pub fn run(&self) -> Result<()> {
        match self {
            Self::Generate(cmd) => cmd.run(),
            Self::Export(cmd) => cmd.run(),
        }
    }
}

/// Generate a binary region parameter file the given json input file
/// files
#[derive(Debug, clap::Args)]
pub struct Generate {
    /// GeoJson file to process
    input: path::PathBuf,

    /// Output file to write to
    output: path::PathBuf,
}

impl Generate {
    pub fn run(&self) -> Result<()> {
        read_params_json(&self.input).and_then(|params| write_params_bin(params, &self.output))
    }
}

/// Export an binary region params file as json output
#[derive(Debug, clap::Args)]
pub struct Export {
    input: path::PathBuf,
}

impl Export {
    fn run(&self) -> Result<()> {
        read_params_bin(&self.input).and_then(|params| write_params_json(std::io::stdout(), params))
    }
}

fn read_params_json<P: AsRef<path::Path>>(file: P) -> Result<BlockchainRegionParamsV1> {
    let reader = fs::File::open(file.as_ref())?;
    let params: BlockchainRegionParamsV1 = serde_json::from_reader(reader)?;
    Ok(params)
}

fn write_params_json<P: Write>(mut output: P, params: BlockchainRegionParamsV1) -> Result<()> {
    output.write_all(serde_json::to_string_pretty(&params)?.as_bytes())?;
    Ok(())
}

fn read_params_bin<P: AsRef<path::Path>>(file: P) -> Result<BlockchainRegionParamsV1> {
    let file = fs::File::open(file.as_ref())?;
    let mut reader = GzDecoder::new(file);
    let mut data = Vec::new();
    reader.read_to_end(&mut data)?;

    Ok(BlockchainRegionParamsV1::decode(data.as_ref())?)
}

fn write_params_bin<P: AsRef<path::Path>>(
    params: BlockchainRegionParamsV1,
    output: P,
) -> Result<()> {
    let file = fs::File::create(output.as_ref())?;
    let mut writer = GzEncoder::new(file, Compression::default());
    writer.write_all(&params.encode_to_vec())?;
    Ok(())
}

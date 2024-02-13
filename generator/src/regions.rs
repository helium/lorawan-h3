use crate::{print_json, utils::*};
use anyhow::Result;
use h3o::{CellIndex, Resolution};
use std::path;

/// Commands on region indexes
#[derive(Debug, clap::Args)]
pub struct Cmd {
    #[command(subcommand)]
    cmd: RegionsCmd,
}

impl Cmd {
    pub fn run(&self) -> Result<()> {
        self.cmd.run()
    }
}

#[derive(Debug, clap::Subcommand)]
pub enum RegionsCmd {
    Generate(Generate),
    Export(Export),
    Find(Find),
}

impl RegionsCmd {
    pub fn run(&self) -> Result<()> {
        match self {
            Self::Generate(cmd) => cmd.run(),
            Self::Export(cmd) => cmd.run(),
            Self::Find(cmd) => cmd.run(),
        }
    }
}

/// Generate a binary region index for a given geojson input file
#[derive(Debug, clap::Args)]
pub struct Generate {
    /// GeoJson file to process
    input: path::PathBuf,

    /// Output file to write to
    output: path::PathBuf,

    /// Resolution to use for h3 cells
    #[arg(default_value_t = Resolution::Seven, short, long)]
    resolution: Resolution,
}

impl Generate {
    pub fn run(&self) -> Result<()> {
        read_geojson(&self.input)
            .and_then(|geojson| to_cells(geojson, self.resolution))
            .and_then(dedup_cells)
            .and_then(compact_cells)
            .and_then(sort_cells)
            .and_then(|cells| write_cells(cells, &self.output))
    }
}

/// Export an h3 index file as a kml file
#[derive(Debug, clap::Args)]
pub struct Export {
    input: path::PathBuf,
    output: path::PathBuf,
    #[arg(default_value_t = Resolution::Seven, short, long)]
    resolution: Resolution,
}

impl Export {
    pub fn run(&self) -> Result<()> {
        read_cells(&self.input)
            .and_then(|cells| to_geojson(cells, self.resolution))
            .and_then(|geojson| write_geojson(geojson, &self.output))?;
        Ok(())
    }
}

/// Check membership of one or moreh3 indexes in all h3idz files in a given
/// folder
#[derive(Debug, clap::Args)]
pub struct Find {
    input: path::PathBuf,
    cells: Vec<CellIndex>,
}

impl Find {
    pub fn run(&self) -> Result<()> {
        use std::collections::HashMap;
        let paths = std::fs::read_dir(&self.input)?;
        let needles: Vec<(String, hextree::Cell)> = self
            .cells
            .iter()
            .map(|entry| {
                hextree::Cell::from_raw(u64::from(*entry)).map(|cell| (entry.to_string(), cell))
            })
            .collect::<hextree::Result<Vec<(String, hextree::Cell)>>>()?;
        let mut matches: HashMap<String, Vec<path::PathBuf>> = HashMap::new();
        for path_result in paths {
            let path = path_result?.path();
            if path.extension().map(|ext| ext == "h3idz").unwrap_or(false) {
                let hex_set = read_hexset(&path)?;
                for (name, needle) in &needles {
                    if hex_set.contains(*needle) {
                        let match_list = matches.entry(name.to_string()).or_insert(vec![]);

                        // Avoid duplicate path entries if the same location is
                        // specified multiple times
                        if !match_list.contains(&path) {
                            match_list.push(path.clone())
                        }
                    }
                }
            }
        }
        print_json(&matches)
    }
}

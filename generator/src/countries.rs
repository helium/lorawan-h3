use crate::{print_json, utils::*};
use anyhow::Result;
use byteorder::{ReadBytesExt, WriteBytesExt};
use h3o::{CellIndex, Resolution};
use hextree::{compaction::EqCompactor, Cell, HexTreeMap};
use isocountry::CountryCode;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    path,
    sync::{Arc, Mutex},
};

/// Commands on region indexes
#[derive(Debug, clap::Args)]
pub struct Cmd {
    #[command(subcommand)]
    cmd: CountriesCmd,
}

impl Cmd {
    pub fn run(&self) -> Result<()> {
        self.cmd.run()
    }
}

#[derive(Debug, clap::Subcommand)]
pub enum CountriesCmd {
    Generate(Generate),
    Find(Find),
}

impl CountriesCmd {
    pub fn run(&self) -> Result<()> {
        match self {
            Self::Generate(cmd) => cmd.run(),
            Self::Find(cmd) => cmd.run(),
        }
    }
}

/// Generate a binary region index for geojson files in a given folder. The
/// files are expected to have the three letter country code with `.geojson` as
/// the extension
#[derive(Debug, clap::Args)]
pub struct Generate {
    /// Path to a folder of GeoJson files to process
    input: path::PathBuf,

    /// Output file to write to
    output: path::PathBuf,

    /// Resolution to use for h3 cells
    #[arg(default_value_t = Resolution::Seven, short, long)]
    resolution: Resolution,
}

type CountryMap = HexTreeMap<CountryCode, EqCompactor>;

impl Generate {
    pub fn run(&self) -> Result<()> {
        fn country_code_from_path(path: &path::Path) -> Result<CountryCode> {
            path.file_stem()
                .ok_or_else(|| anyhow::anyhow!("Missing filename"))
                .and_then(|stem| {
                    CountryCode::for_alpha3_caseless(&stem.to_string_lossy())
                        .map_err(anyhow::Error::from)
                })
        }

        fn insert_country_cells(
            path: &path::Path,
            resolution: Resolution,
            country_code: CountryCode,
            hextree: Arc<Mutex<CountryMap>>,
        ) -> Result<()> {
            let cells = read_geojson(path).and_then(|geojson| to_cells(geojson, resolution))?;

            cells
                .par_chunks(100_000)
                .for_each_with(hextree, |hextree, chunk| {
                    let mut hextree = hextree.lock().unwrap();
                    chunk
                        .iter()
                        .filter_map(to_hextree_cell)
                        .for_each(|hex_cell| hextree.insert(hex_cell, country_code));
                });
            log::info!("Inserted {}", country_code);
            Ok(())
        }

        // Read filenames from folder
        let paths = std::fs::read_dir(&self.input)?
            .map(|entry| entry.map(|e| e.path()))
            .collect::<Result<Vec<path::PathBuf>, std::io::Error>>()?;

        // Filter out non-geojson files
        let paths = paths
            .into_iter()
            .filter(|path| {
                path.extension()
                    .map(|ext| ext == "geojson")
                    .unwrap_or(false)
            })
            .collect::<Vec<path::PathBuf>>();

        let hexmap: Arc<Mutex<HexTreeMap<CountryCode, _>>> =
            Arc::new(Mutex::new(HexTreeMap::with_compactor(EqCompactor)));

        paths.into_par_iter().for_each(|path| {
            let hexmap = hexmap.clone();
            let _ = country_code_from_path(&path).and_then(|country_code| {
                insert_country_cells(&path, self.resolution, country_code, hexmap)
            });
        });

        let result = hexmap.lock().unwrap();
        write_hexmap(&result, &self.output)?;
        println!("size: {}", result.len());
        Ok(())
    }
}

/// Check membership of one or more h3 indexes in a country map file
#[derive(Debug, clap::Args)]
pub struct Find {
    /// Country map file to search
    input: path::PathBuf,
    /// Indexes to look up countries for
    cells: Vec<CellIndex>,
}

impl HexMapValueReader for CountryCode {
    fn read_value(reader: &mut dyn std::io::Read) -> Result<Self> {
        let decoded = reader.read_u32::<byteorder::LittleEndian>()?;
        Ok(CountryCode::for_id(decoded)?)
    }
}

impl HexMapValueWriter for CountryCode {
    fn write_value(&self, writer: &mut dyn std::io::Write) -> Result<()> {
        writer.write_u32::<byteorder::LittleEndian>(self.numeric_id())?;
        Ok(())
    }
}

impl Find {
    pub fn run(&self) -> Result<()> {
        log::info!("Reading hexmap");
        let hex_map = read_hexmap::<_, CountryCode>(&self.input)?;
        log::info!("Read hexmap: {}", hex_map.len());

        let result: HashMap<String, CountryCode> = self
            .cells
            .clone()
            .into_iter()
            .filter_map(|cell| {
                Cell::from_raw(u64::from(cell))
                    .ok()
                    .and_then(|cell| hex_map.get(cell).map(|(_, code)| (cell.to_string(), *code)))
            })
            .collect();
        print_json(&result)
    }
}

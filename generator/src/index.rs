use crate::print_json;
use anyhow::Result;
use byteorder::ReadBytesExt;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use geojson::GeoJson;
use h3o::{
    geom::{Geometry, ToCells, ToGeo},
    CellIndex, Resolution,
};
use std::{fs, io::Write, path};

/// Commands on region indexes
#[derive(Debug, clap::Args)]
pub struct Cmd {
    #[command(subcommand)]
    cmd: IndexCmd,
}

impl Cmd {
    pub fn run(&self) -> Result<()> {
        self.cmd.run()
    }
}

#[derive(Debug, clap::Subcommand)]
pub enum IndexCmd {
    Generate(Generate),
    Export(Export),
    Find(Find),
}

impl IndexCmd {
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

fn read_geojson<P: AsRef<path::Path>>(file: P) -> Result<Geometry<'static>> {
    let json = GeoJson::from_reader(fs::File::open(file.as_ref())?)?;
    Ok(Geometry::try_from(&json)?)
}

fn to_cells(geometry: Geometry, resolution: Resolution) -> Result<Vec<CellIndex>> {
    let collection = geometry.to_cells(resolution).collect();
    Ok(collection)
}

fn to_geojson(cells: Vec<CellIndex>, resolution: Resolution) -> Result<geojson::GeoJson> {
    let cells: Vec<_> = CellIndex::uncompact(cells, resolution).collect();
    let geojson = cells.to_geojson()?;
    Ok(geojson::GeoJson::from(geojson))
}

fn sort_cells(mut cells: Vec<CellIndex>) -> Result<Vec<CellIndex>> {
    cells.as_mut_slice().sort_by(|a, b| {
        let ar = a.resolution();
        let br = b.resolution();
        if ar == br {
            a.cmp(b)
        } else {
            ar.cmp(&br)
        }
    });
    Ok(cells)
}

fn dedup_cells(mut cells: Vec<CellIndex>) -> Result<Vec<CellIndex>> {
    cells.sort_unstable();
    cells.dedup();
    Ok(cells)
}

fn compact_cells(cells: Vec<CellIndex>) -> Result<Vec<CellIndex>> {
    let compacted = CellIndex::compact(cells)?;
    Ok(compacted.collect())
}

fn read_cells<P: AsRef<path::Path>>(file: P) -> Result<Vec<CellIndex>> {
    let file = fs::File::open(file.as_ref())?;
    let mut reader = GzDecoder::new(file);

    let mut vec = Vec::new();
    while let Ok(entry) = reader.read_u64::<byteorder::LittleEndian>() {
        vec.push(CellIndex::try_from(entry)?);
    }
    Ok(vec)
}

fn read_hexset<P: AsRef<path::Path>>(file: P) -> Result<hextree::HexTreeSet> {
    let file = fs::File::open(file.as_ref())?;
    let mut reader = GzDecoder::new(file);
    let mut vec = Vec::new();

    while let Ok(entry) = reader.read_u64::<byteorder::LittleEndian>() {
        vec.push(hextree::Cell::from_raw(entry)?);
    }

    Ok(vec.iter().collect())
}

fn write_cells<P: AsRef<path::Path>>(cells: Vec<CellIndex>, output: P) -> Result<()> {
    let file = fs::File::create(output.as_ref())?;
    let mut writer = GzEncoder::new(file, Compression::default());
    for cell in cells.iter() {
        writer.write_all(&u64::from(*cell).to_le_bytes())?;
    }
    Ok(())
}

fn write_geojson<P: AsRef<path::Path>>(geojson: GeoJson, output: P) -> Result<()> {
    let mut writer = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(output.as_ref())?;
    writer.write_all(geojson.to_string().as_bytes())?;
    Ok(())
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

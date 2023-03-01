use crate::polyfill;
use anyhow::Result;
use byteorder::ReadBytesExt;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use geojson::GeoJson;
use h3ron::{self, to_geo::ToLinkedPolygons, H3Cell, Index};
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
}

impl IndexCmd {
    pub fn run(&self) -> Result<()> {
        match self {
            Self::Generate(cmd) => cmd.run(),
            Self::Export(cmd) => cmd.run(),
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
    #[arg(default_value_t = 7, short, long)]
    resolution: u8,
}

fn read_geojson<P: AsRef<path::Path>>(file: P) -> Result<GeoJson> {
    let json = GeoJson::from_reader(fs::File::open(file.as_ref())?)?;
    Ok(json)
}

fn to_h3_cells(geojson: GeoJson, resolution: u8) -> Result<Vec<H3Cell>> {
    let collection = geojson::quick_collection(&geojson)?;
    let cells = polyfill::to_h3_cells(collection, resolution)?;
    Ok(cells)
}

fn to_multi_polygon(cells: Vec<H3Cell>) -> Result<geo_types::MultiPolygon> {
    let multi_polygon = cells
        .to_linked_polygons(false)
        .map(geo_types::MultiPolygon::from)?;
    Ok(multi_polygon)
}

fn sort_cells(mut cells: Vec<H3Cell>) -> Result<Vec<H3Cell>> {
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

fn dedup_cells(mut cells: Vec<H3Cell>) -> Result<Vec<H3Cell>> {
    cells.sort_unstable();
    cells.dedup();
    Ok(cells)
}

fn compact_cells(cells: Vec<H3Cell>) -> Result<Vec<H3Cell>> {
    let mut compacted = h3ron::compact_cells(&cells)?;
    Ok(compacted.drain().collect())
}

fn read_cells<P: AsRef<path::Path>>(file: P) -> Result<Vec<H3Cell>> {
    let file = fs::File::open(file.as_ref())?;
    let mut reader = GzDecoder::new(file);

    let mut vec = Vec::new();
    while let Ok(entry) = reader.read_u64::<byteorder::LittleEndian>() {
        vec.push(H3Cell::try_from(entry)?);
    }
    Ok(vec)
}

fn write_cells<P: AsRef<path::Path>>(cells: Vec<H3Cell>, output: P) -> Result<()> {
    let file = fs::File::create(output.as_ref())?;
    let mut writer = GzEncoder::new(file, Compression::default());
    for cell in cells.iter() {
        writer.write_all(&cell.to_le_bytes())?;
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
            .and_then(|geojson| to_h3_cells(geojson, self.resolution))
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
}

impl Export {
    pub fn run(&self) -> Result<()> {
        read_cells(&self.input)
            .and_then(to_multi_polygon)
            .and_then(|multi_polygon| {
                let geojson = GeoJson::from(geojson::Geometry::from(&multi_polygon));
                write_geojson(geojson, &self.output)
            })?;
        Ok(())
    }
}

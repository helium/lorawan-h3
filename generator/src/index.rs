use crate::{polyfill, print_json};
use anyhow::Result;
use byteorder::ReadBytesExt;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use geojson::GeoJson;
use h3ron::{self, to_geo::ToLinkedPolygons, H3Cell, Index};
use log::debug;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::{collections::HashMap, fs, io::Write, path, path::PathBuf};

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
    Overlaps(Overlaps),
}

impl IndexCmd {
    pub fn run(&self) -> Result<()> {
        match self {
            Self::Generate(cmd) => cmd.run(),
            Self::Overlaps(cmd) => cmd.run(),
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

fn read_hexset<P: AsRef<path::Path>>(file: P) -> Result<hextree::HexTreeSet> {
    let file = fs::File::open(file.as_ref())?;
    let mut reader = GzDecoder::new(file);
    let mut vec = Vec::new();

    while let Ok(entry) = reader.read_u64::<byteorder::LittleEndian>() {
        vec.push(hextree::Cell::from_raw(entry)?);
    }

    Ok(vec.iter().collect())
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

/// Check membership of one or moreh3 indexes in all h3idz files in a given
/// folder
#[derive(Debug, clap::Args)]
pub struct Find {
    input: path::PathBuf,
    cells: Vec<h3ron::H3Cell>,
}

impl Find {
    pub fn run(&self) -> Result<()> {
        let paths = std::fs::read_dir(&self.input)?;
        let needles: Vec<(String, hextree::Cell)> = self
            .cells
            .iter()
            .map(|entry| hextree::Cell::from_raw(**entry).map(|cell| (entry.to_string(), cell)))
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

/// Compare index files against each other for H3 indices.
///
/// Returns a non-zero exit code if any overlaps are found.
#[derive(Debug, clap::Args)]
pub struct Overlaps {
    /// Region files to compare
    input: Vec<PathBuf>,
}

impl Overlaps {
    pub fn run(&self) -> Result<()> {
        let regions: Vec<(String, Vec<H3Cell>)> = {
            let mut regions = Vec::with_capacity(self.input.len());
            for path in self.input.iter() {
                let cells = crate::index::read_cells(path)?;
                regions.push((path.to_string_lossy().to_string(), cells))
            }
            regions
        };

        // Map of (region, region) to overlapping indices.
        let mut overlap_map: HashMap<(&str, &str), Vec<(H3Cell, H3Cell)>> = HashMap::new();

        for a in &regions {
            for b in &regions {
                // Normally the cartesian product of 'A B C D' and 'A B C D' would be:
                //         A     B     C     D
                //     A (A,A) (A,B) (A,C) (A,D)
                //     B (B,A) (B,B) (B,C) (B,D)
                //     C (C,A) (C,B) (C,C) (C,D)
                //     D (D,A) (D,B) (D,C) (D,D)

                // Obviously we don't want to compare index files to
                // themselves, so we can remove them:
                //         A     B     C     D
                //     A   -   (A,B) (A,C) (A,D)
                //     B (B,A)   -   (B,C) (B,D)
                //     C (C,A) (C,B)   -   (C,D)
                //     D (D,A) (D,B) (D,C)   -
                if a.0 == b.0 {
                    continue;
                };

                let [(region_lhs, polyfill_lhs), (region_rhs, polyfill_rhs)] = {
                    let mut region_pair = [a, b];
                    region_pair.sort_by(|a, b| a.1.cmp(&b.1));
                    region_pair
                };

                // Additionally, we don't want to check (x,y) if we've
                // already checked (y,x), so we can trim the set a
                // little further to only upper right set:
                //         A     B     C     D
                //     A   -   (A,B) (A,C) (A,D)
                //     B   -     -   (B,C) (B,D)
                //     C   -     -     -   (C,D)
                //     D   -     -     -     -
                if overlap_map.contains_key(&(region_lhs, region_rhs)) {
                    continue;
                }

                // TODO: configure logger to not print this by default.
                //       Or to print to stderr.
                debug!("Comparing '{}' against '{}'", region_lhs, region_rhs);

                let overlaps: Vec<(H3Cell, H3Cell)> = polyfill_rhs
                    .par_iter()
                    .flat_map(|target_cell| collect_relationships(polyfill_lhs, *target_cell))
                    .collect();

                overlap_map.insert((region_lhs, region_rhs), overlaps);
            }
        }

        type OverlapReport<'a> = HashMap<&'a str, HashMap<&'a str, Vec<(H3Cell, H3Cell)>>>;

        let overlap_report: OverlapReport = {
            let mut overlap_report = HashMap::new();
            for ((region_lhs, region_rhs), conflicting_indices) in
                overlap_map.into_iter().filter(|(_, v)| !v.is_empty())
            {
                overlap_report
                    .entry(region_lhs)
                    .or_insert(HashMap::new())
                    .insert(region_rhs, conflicting_indices);
            }
            overlap_report
        };

        if overlap_report.is_empty() {
            Ok(())
        } else {
            println!("{}", serde_json::to_string_pretty(&overlap_report)?);
            std::process::exit(1);
        }
    }
}

/// Returns a vector of any cells in the in `polyfill` that are
/// related `target_cell`.
///
/// See [are_related] for more info.
fn collect_relationships(polyfill: &[H3Cell], target_cell: H3Cell) -> Vec<(H3Cell, H3Cell)> {
    polyfill
        .iter()
        .copied()
        .zip(std::iter::repeat(target_cell))
        .filter(|(poly_cell, target_cell)| are_related(*poly_cell, *target_cell))
        .collect()
}

/// Returns `true` if any of the following are true:
///
/// - `lhs` and `rhs` the exact cell
/// - `lhs` is a parent cell of `rhs`
/// - `lhs` is a child cell of `rhs`
fn are_related(lhs: H3Cell, rhs: H3Cell) -> bool {
    if lhs.resolution() == rhs.resolution() {
        lhs == rhs
    } else if lhs.resolution() <= rhs.resolution() {
        lhs == rhs
            .get_parent(lhs.resolution())
            .expect("we already checked that rhs is promotable to lhs's res")
    } else {
        rhs == lhs
            .get_parent(lhs.resolution())
            .expect("we already checked that rhs is promotable to lhs's res")
    }
}

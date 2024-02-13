use anyhow::Result;
use byteorder::{ReadBytesExt, WriteBytesExt};
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use geojson::GeoJson;
use h3o::{
    geom::{Geometry, ToCells, ToGeo},
    CellIndex, Resolution,
};
use std::{fs, io::Write, path};

pub fn read_geojson<P: AsRef<path::Path>>(file: P) -> Result<Geometry<'static>> {
    log::info!("Starting geojson read");
    let json = GeoJson::from_reader(fs::File::open(file.as_ref())?)?;
    log::info!("End geojson read");
    Ok(Geometry::try_from(&json)?)
}

pub fn to_cells(geometry: Geometry, resolution: Resolution) -> Result<Vec<CellIndex>> {
    let collection = geometry.to_cells(resolution);
    Ok(collection.collect())
}

pub fn to_hextree_cell(cell_index: &CellIndex) -> Option<hextree::Cell> {
    hextree::Cell::from_raw((*cell_index).into()).ok()
}

pub fn to_geojson(cells: Vec<CellIndex>, resolution: Resolution) -> Result<geojson::GeoJson> {
    let cells: Vec<_> = CellIndex::uncompact(cells, resolution).collect();
    let geojson = cells.to_geojson()?;
    Ok(geojson::GeoJson::from(geojson))
}

pub fn sort_cells(mut cells: Vec<CellIndex>) -> Result<Vec<CellIndex>> {
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

pub fn dedup_cells(mut cells: Vec<CellIndex>) -> Result<Vec<CellIndex>> {
    cells.sort_unstable();
    cells.dedup();
    Ok(cells)
}

pub fn compact_cells(cells: Vec<CellIndex>) -> Result<Vec<CellIndex>> {
    let compacted = CellIndex::compact(cells)?;
    Ok(compacted.collect())
}

pub fn read_cells<P: AsRef<path::Path>>(file: P) -> Result<Vec<CellIndex>> {
    let file = fs::File::open(file.as_ref())?;
    let mut reader = GzDecoder::new(file);

    let mut vec = Vec::new();
    while let Ok(entry) = reader.read_u64::<byteorder::LittleEndian>() {
        vec.push(CellIndex::try_from(entry)?);
    }
    Ok(vec)
}

pub fn read_hexset<P: AsRef<path::Path>>(file: P) -> Result<hextree::HexTreeSet> {
    let file = fs::File::open(file.as_ref())?;
    let mut reader = GzDecoder::new(file);
    let mut vec = Vec::new();

    while let Ok(entry) = reader.read_u64::<byteorder::LittleEndian>() {
        vec.push(hextree::Cell::from_raw(entry)?);
    }

    Ok(vec.iter().collect())
}

pub fn write_cells<P: AsRef<path::Path>>(cells: Vec<CellIndex>, output: P) -> Result<()> {
    let file = fs::File::create(output.as_ref())?;
    let mut writer = GzEncoder::new(file, Compression::default());
    for cell in cells.iter() {
        writer.write_all(&u64::from(*cell).to_le_bytes())?;
    }
    Ok(())
}

pub trait HexMapValueReader: std::cmp::PartialEq + Clone {
    fn read_value(reader: &mut dyn std::io::Read) -> Result<Self>
    where
        Self: Sized;
}

pub trait HexMapValueWriter {
    fn write_value(&self, writer: &mut dyn std::io::Write) -> Result<()>;
}

pub fn write_hexmap<P: AsRef<path::Path>, V, C>(
    map: &hextree::HexTreeMap<V, C>,
    output: P,
) -> Result<()>
where
    V: HexMapValueWriter,
{
    let file = fs::File::create(output.as_ref())?;
    let mut writer = GzEncoder::new(file, Compression::default());
    for (cell, value) in map.iter() {
        writer.write_u64::<byteorder::LittleEndian>(cell.into_raw())?;
        value.write_value(&mut writer)?;
    }
    Ok(())
}

pub fn read_hexmap<P: AsRef<path::Path>, V>(
    path: P,
) -> Result<hextree::HexTreeMap<V, hextree::compaction::EqCompactor>>
where
    V: HexMapValueReader,
{
    let file = fs::File::open(path.as_ref())?;
    let mut reader = GzDecoder::new(file);
    let mut map = hextree::HexTreeMap::with_compactor(hextree::compaction::EqCompactor);
    while let Ok(cell) = reader.read_u64::<byteorder::LittleEndian>() {
        let value = V::read_value(&mut reader)?;
        map.insert(hextree::Cell::from_raw(cell)?, value);
    }
    Ok(map)
}

pub fn write_geojson<P: AsRef<path::Path>>(geojson: GeoJson, output: P) -> Result<()> {
    let mut writer = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(output.as_ref())?;
    writer.write_all(geojson.to_string().as_bytes())?;
    Ok(())
}

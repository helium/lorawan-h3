use anyhow::Result;
use geo_types::{Geometry, GeometryCollection, MultiPolygon, Polygon};
use h3ron::{to_h3::ToH3Cells, H3Cell};
use rayon::prelude::*;

pub(crate) trait ToPolygons {
    fn to_polygons(self, target: &mut Vec<Polygon>);
}

impl ToPolygons for Polygon<f64> {
    fn to_polygons(self, target: &mut Vec<Polygon>) {
        target.push(self)
    }
}

impl ToPolygons for MultiPolygon<f64> {
    fn to_polygons(mut self, target: &mut Vec<Polygon>) {
        target.append(&mut self.0)
    }
}

impl ToPolygons for Geometry<f64> {
    fn to_polygons(self, target: &mut Vec<Polygon>) {
        match self {
            Geometry::Polygon(poly) => poly.to_polygons(target),
            Geometry::MultiPolygon(mpoly) => mpoly.to_polygons(target),
            _ => panic!("Unhandled Geometry"),
        }
    }
}

impl ToPolygons for GeometryCollection<f64> {
    fn to_polygons(self, target: &mut Vec<Polygon>) {
        for geometry in self.0 {
            geometry.to_polygons(target)
        }
    }
}

pub(crate) fn to_h3_cells(
    collection: GeometryCollection<f64>,
    resolution: u8,
) -> Result<Vec<H3Cell>> {
    let mut polygons = vec![];
    collection.to_polygons(&mut polygons);

    let cells: Vec<H3Cell> = polygons
        .into_par_iter()
        .try_fold(Vec::new, |mut out, poly| {
            poly.to_h3_cells(resolution).map(|mut fills| {
                let mut fills = fills.drain().collect();
                out.append(&mut fills);
                out
            })
        })
        .try_reduce(Vec::new, |mut acc, mut fills| {
            acc.append(&mut fills);
            Ok(acc)
        })?;
    Ok(cells)
}

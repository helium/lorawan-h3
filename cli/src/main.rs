mod regions;
use geo_types::Coordinate;
use h3ron::{FromH3Index, H3Cell, Index};
use rayon::prelude::*;
use std::{env, process::exit, str::FromStr};

fn main() {
    let args: Vec<String> = env::args().into_iter().skip(1).collect();
    let h3 = match args.as_slice() {
        [cmd] if cmd == "overlaps" => {
            overlaps();
            return;
        }
        [index_str] => u64::from_hex_dec_bin(index_str)
            .map_err(|_| "u64")
            .and_then(|index| {
                let index = H3Cell::from_h3index(index);
                if index.is_valid() {
                    Ok(index)
                } else {
                    Err("H3 index")
                }
            })
            .unwrap_or_else(|e| {
                eprintln!("{} is not a valid {}", index_str, e);
                exit(1)
            }),
        [lat_str, lon_str] => {
            let xy = &[lat_str, lon_str]
                .iter()
                .map(|lat_or_lon| f64::from_str(lat_or_lon))
                .collect::<Result<Vec<f64>, _>>()
                .unwrap_or_else(|_| {
                    eprintln!("{} {} are not valid coordinates", lat_str, lon_str);
                    exit(1)
                });
            H3Cell::from_coordinate(&Coordinate { x: xy[1], y: xy[0] }, 12).unwrap()
        }
        _ => {
            usage();
            exit(1)
        }
    };
    if let Some((region, parent_index)) = lookup(h3) {
        println!("{} @ {}", region, parent_index.to_string());
    } else {
        exit(1);
    }
}

fn usage() {
    eprintln!("lwr <H3> | <LAT> <LON>");
}

fn lookup(target_index: H3Cell) -> Option<(&'static str, H3Cell)> {
    for (region, indices) in regions::REGIONS {
        eprintln!("searching {} for {}", region, target_index.to_string());
        if let Some(parent_index) = indices
            .iter()
            .map(|i| H3Cell::new(*i))
            .find(|i| i.contains(&target_index))
        {
            return Some((region, parent_index));
        }
    }
    None
}

// Check every hex in every region against every hex in every other region for overlap.
fn overlaps() {
    for (region_outer, indices_outer) in regions::REGIONS {
        for (region_inner, indices_inner) in regions::REGIONS
            .iter()
            .filter(|(region, _)| region != region_outer)
        {
            eprintln!("checking if {} contains {}", region_outer, region_inner);
            for index_outer in indices_outer.iter().map(|i| H3Cell::new(*i)) {
                indices_inner.par_iter().for_each(|i| {
                    let index_inner = H3Cell::new(*i);
                    if index_outer.contains(&index_inner) {
                        println!(
                            "\t{}:{:?}:res{} envelops {}:{:?}:res{}",
                            region_outer,
                            index_outer,
                            index_outer.resolution(),
                            region_inner,
                            index_inner,
                            index_inner.resolution()
                        );
                    }
                });
            }
        }
    }
}

trait FromHexDecBin: Sized {
    type Error;
    fn from_hex_dec_bin(s: &str) -> Result<Self, Self::Error>;
}

macro_rules! impl_from_hex_dec_bin {
    ($T:tt, $E:ty) => {
        impl FromHexDecBin for $T {
            type Error = $E;
            fn from_hex_dec_bin(s: &str) -> Result<$T, Self::Error> {
                if s.len() > 2 {
                    match s.split_at(2) {
                        ("0x", rest) => $T::from_str_radix(rest, 16),
                        ("0b", rest) => $T::from_str_radix(rest, 2),
                        _ => $T::from_str_radix(s, 10),
                    }
                } else {
                    $T::from_str_radix(s, 10)
                }
            }
        }
    };
}

impl_from_hex_dec_bin!(u64, ::std::num::ParseIntError);

use std::path::PathBuf;

use calamine::{open_workbook, Data, Error, Reader, Xlsx};
use clap::Parser;
use ndarray::Array2;
use twmap::{GameLayer, GameTile, TileFlags, TilemapLayer, TwMap};

#[derive(Parser)]
struct Args {
    in_path: PathBuf,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let mut workbook: Xlsx<_> = open_workbook(&args.in_path)?;
    let range = workbook.worksheet_range("map")?;

    let width = range.width();
    let height = range.height();

    let mut map = TwMap::parse(include_bytes!("../EMPTY.map")).expect("parsing failed");
    map.load().expect("loading failed");

    // get game layer
    let game_layer = map
        .find_physics_layer_mut::<GameLayer>()
        .unwrap()
        .tiles_mut()
        .unwrap_mut();

    *game_layer =
        Array2::<GameTile>::from_elem((height, width), GameTile::new(0, TileFlags::empty()));

    for (y, r) in range.rows().enumerate() {
        for (x, c) in r.iter().enumerate() {
            let v = match c {
                Data::Int(v) => Some(*v),
                Data::Float(v) => Some(*v as i64),
                Data::String(v) => Some(v.parse().unwrap()),
                _ => None,
            };
            if let Some(v) = v {
                game_layer[[y as usize, x as usize]] = GameTile::new(v as u8, TileFlags::empty());
            }
        }
    }
    let outpath = args.in_path.with_extension("map");
    println!("exporting map to {:?}", &outpath);
    map.save_file(outpath).expect("saving failed");
    Ok(())
}

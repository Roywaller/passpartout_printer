extern crate enigo;
extern crate failure;
extern crate passpartout_printer;

use std::time::Duration;
use enigo::Enigo;
use passpartout_printer::easel::Easel;
use passpartout_printer::colors::PaletteColor;
use failure::Error;
use passpartout_printer::coords::Coord;

fn app() -> Result<(), Error> {
    let enigo = Enigo::new();
    let mut easel = Easel::new(
        "coords.json".to_string(),
        enigo,
        Duration::from_millis(6),
    )?;

    // First, draw the background sky.
    easel.change_brush_size(16);
    let (easel_ul, easel_lr) = easel.get_bounds();
    let easel_size = &easel_lr - &easel_ul;
    let points = &[
        Coord::new(0, 0),
        Coord::new(0, easel_size.y),
        easel_size,
        Coord::new(easel_size.x, 0),
        Coord::new(easel_size.x / 2, 0),
        Coord::new(easel_size.x / 2, easel_size.y),
    ];
    easel.draw_shape(points, &PaletteColor::LightBlue, true, false)?;

    // Next, draw us a nice house.
    easel.change_brush_size(0);
    let house_ul = Coord::new(easel_size.x / 4, easel_size.y * 3 / 4);
    let house_ur = Coord::new(easel_size.x * 3 / 4, easel_size.y * 3 / 4);
    let points = &[
        house_ul,
        house_ur,
        Coord::new(easel_size.x * 3 / 4, easel_size.y),
        Coord::new(easel_size.x / 4, easel_size.y),
    ];
    easel.draw_shape(points, &PaletteColor::DarkRed, true, true)?;

    // Now draw us a roof.
    let points = &[
        house_ul,
        house_ur,
        Coord::new(house_ur.x - house_ul.x, house_ur.y - 150),
    ];
    easel.draw_shape(points, &PaletteColor::LightBrown, true, true)?;

    Ok(())
}

fn main() {
    app().unwrap();
}

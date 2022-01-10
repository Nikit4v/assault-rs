extern crate assault;

use assault::types::prelude::*;
use assault::render::surface::Surface;

fn main() {
    let mut surface = Surface::from_png("/Users/aleksejzmeevyh/Pictures/crhby.png").unwrap();
    surface.resize(Size::new(4000, 3000));
    surface.save_png("/Users/aleksejzmeevyh/Pictures/out.png");
}
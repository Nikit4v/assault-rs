use crate::render::DrawingOptions;
use crate::types::prelude::*;
use std::mem;
use std::ptr::replace;

pub trait Action {
    fn apply(surface: &mut PixelSet, object: &PixelSet, options: DrawingOptions);
}

pub struct Overlap;

impl Action for Overlap {
    fn apply(surface: &mut PixelSet, object: &PixelSet, options: DrawingOptions) {
        let mut mut_pixels: Vec<&mut Color> = Vec::new();
        for x_slice in options.offset.x..options.offset.x+object.size.x{
            mut_pixels.extend(surface.data[(options.offset.y*x_slice)..(options.offset.y+object.size.y*x_slice)].iter_mut());
        }

        for pixel in mut_pixels {
            let mut new_color = Color::default();


        }
    }
}
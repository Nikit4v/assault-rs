use crate::render::drawing_options::DrawingOptions;
use crate::types::prelude::*;

pub trait Action {
    fn apply(&self, surface: &mut PixelSet, object: &PixelSet, options: DrawingOptions);
}

pub struct Overlap;

impl Action for Overlap {
    fn apply(&self, _surface: &mut PixelSet, _object: &PixelSet, _options: DrawingOptions) {
        // unsafe {
        //     let mut mut_pixels: Vec<*mut Color> = Vec::new();
        //     for x_slice in options.offset.x..options.offset.x + object.size.x {
        //         let mut surface_data = surface.data[(options.offset.y * x_slice)..(options.offset.y + object.size.y * x_slice)].iter_mut().map(|x| { x as *mut Color });
        //         mut_pixels.extend(surface_data);
        //     }
        //
        //     for pixel_position in options.offset..options.limit {}
        // }
        todo!()
    }
}
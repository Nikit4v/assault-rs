use image::ImageResult;
use crate::render::backend::{Backend, ptr, Shape};
extern crate image;

pub struct LoadedImage {
    pub image_ptr: ptr,
    pub alpha_mask_ptr: Option<ptr>,
    pub shape: (usize, usize, usize),
}

impl LoadedImage {

    pub fn from_bytes<T: Backend>(bytes: &[u8], shape: Shape, mut_backend: &mut T) -> Self {
        let image_ptr = mut_backend.allocate_frame(shape);
        mut_backend.load_frame(image_ptr, bytes);
        Self {
            image_ptr,
            alpha_mask_ptr: None,
            shape
        }
    }

    pub fn add_alpha_mask<T: Backend>(&mut self, bytes: &[u8], mut_backend: &mut T) -> Result<(), &str>{
        let alpha_ptr = mut_backend.allocate_frame(self.shape);
        mut_backend.load_frame(alpha_ptr, bytes);
        self.alpha_mask_ptr = Some(alpha_ptr);
        Ok(())
    }
}

pub fn load_png<T: Backend>(path: &str, backend: &mut T) -> ImageResult<LoadedImage> {
    let basic_image = image::open(path)?.into_rgba8();
    let image_shape = (basic_image.dimensions().0 as usize, basic_image.dimensions().1 as usize, 3);
    let mut alphas: Vec<u8> = Vec::with_capacity(image_shape.0 * image_shape.1 * image_shape.2);
    basic_image.pixels().for_each(|x| {
        alphas.push(x.0[3]);
        alphas.push(x.0[3]);
        alphas.push(x.0[3]);
    });
    let mut colors: Vec<u8> = Vec::with_capacity(image_shape.0 * image_shape.1 * image_shape.2);
    basic_image.pixels().for_each(|x| {
        colors.push(x.0[0]);
        colors.push(x.0[1]);
        colors.push(x.0[2]);
    });
    colors.reverse();
    let mut image = LoadedImage::from_bytes(&colors, image_shape, backend);
    image.add_alpha_mask(&alphas, backend).unwrap_or_else(|x| {
        println!("Failed to add alpha with {x}", x=x)
    });
    Ok(image)
}
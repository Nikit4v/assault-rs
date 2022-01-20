use image::ImageResult;
use crate::render::backend::{Backend, ptr};
extern crate image;

pub struct LoadedImage<'a, T> where T: Backend {
    pub backend_ptr: ptr,
    pub alpha_mask_ptr: Option<ptr>,
    pub backend: &'a T,

    pub shape: (usize, usize, usize)
}

impl<'a, T: Backend> LoadedImage<'a, T> {

    pub fn from_bytes(bytes: &[u8], shape: (usize, usize, usize), backend: &'a mut T) -> Self {
        let image_ptr = backend.allocate_frame();
        backend.load_frame(image_ptr, bytes, shape);
        Self {
            backend_ptr: image_ptr,
            alpha_mask_ptr: None,
            backend: backend as &'a T,
            shape
        }
    }

    pub fn add_alpha_mask(&mut self, bytes: &[u8], mut_backend: &mut T) -> Result<(), &str>{
        let alpha_ptr = mut_backend.allocate_frame();
        mut_backend.load_frame(alpha_ptr, bytes, self.shape);
        self.alpha_mask_ptr = Some(alpha_ptr);
        Ok(())
    }

    pub fn load_png(path: &str, backend: &'a mut T) -> ImageResult<LoadedImage<'a, T>> {
        let basic_image = image::open(path)?.into_rgba8();

        let image_shape = (basic_image.height() as usize, basic_image.width() as usize, 3);
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
        let mut image = Self::from_bytes(&colors, image_shape, backend);
        image.add_alpha_mask(&alphas, backend);
        Ok(image)
    }
}
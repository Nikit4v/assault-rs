use crate::render::actions;
use crate::render::drawing_options::DrawingOptions;
use crate::types::prelude::*;
use image;
use image::{EncodableLayout, RgbaImage};
use rayon::prelude::*;

#[derive(Clone)]
pub struct Surface {
    pub data: Box<PixelSet>,
    size: Size
}

impl Surface {
    pub fn null() -> Self {
        Self {
            data: Box::new(PixelSet::null()),
            size: Size::null()
        }
    }

    pub fn new(size: Size, set: PixelSet) -> Self {
        Self {
            size,
            data: Box::new(set)
        }
    }

    pub fn from_png(path: &str) -> Result<Surface, &str> {
        return match image::open(path) {
            Ok(image) => {
                let image: RgbaImage = image.into_rgba8();
                let image_data = PixelSet::from(image.as_bytes().to_vec()).with_changed_size(Size {
                        x: image.dimensions().0 as usize,
                        y: image.dimensions().1 as usize,
                    });
                let self_ = Self {
                    data: Box::new(image_data),
                    size: Size {
                        x: image.dimensions().0 as usize,
                        y: image.dimensions().1 as usize,
                    }
                };
                Ok(self_)
            }
            Err(_) => {
                Err("Unable to open image!")
            }
        }
    }

    pub fn first_pixel(&self) {
        let fallback = Color::new(0, 0, 0, 1.);
        let pixel = self.data.data.first().unwrap_or_else(|| {
            println!("No pixel found. Fallbacking to zero...");
            &fallback
        });
        println!("({}, {}, {}, {})", pixel.r, pixel.g, pixel.b, pixel.a)
    }


    pub fn draw(&mut self, action: &dyn actions::Action, object: &dyn Drawable, options: DrawingOptions) {
        action.apply(self.data.as_mut(), &object.render(), options);
    }

    pub fn resize(&mut self, size: Size) {
        let scale_factor: (f64, f64) = (size.x as f64 / self.size.x as f64, size.y as f64 / self.size.y as f64);
        let positions: Vec<Position> = size.clone().iter_positions().collect();
        let new_pixelset: Box<PixelSet> = Box::new(PixelSet::from(positions.par_iter().map(|&position| -> Color {
            let pos_old: (f64, f64) = (position.x as f64 / scale_factor.0, position.y as f64 / scale_factor.1);
            // println!("({}, {}), {}, {}, {}", pos_old.0, pos_old.1, position, scale_factor.0, scale_factor.1);
            let pos_11: Position = Position::new(pos_old.0.floor() as usize, pos_old.1.floor() as usize).add_if_out_of_size(self.size, 1, 1);
            let pos_12: Position = Position::new(pos_old.0.floor() as usize, pos_old.1.ceil() as usize).add_if_out_of_size(self.size, 1, -1);
            let pos_21: Position = Position::new(pos_old.0.ceil() as usize, pos_old.1.floor() as usize).add_if_out_of_size(self.size, -1, 1);
            let pos_22: Position = Position::new(pos_old.0.ceil() as usize, pos_old.1.ceil() as usize).add_if_out_of_size(self.size, -1, -1);
            // println!("{}, {}, {}, {}", pos_11, pos_12, pos_21, pos_22);
            unsafe {
                let col_11: &Color = self.data.get_color_ref_unchecked(&pos_11);
                let col_12: &Color = self.data.get_color_ref_unchecked(&pos_12);
                let col_21: &Color = self.data.get_color_ref_unchecked(&pos_21);
                let col_22: &Color = self.data.get_color_ref_unchecked(&pos_22);
                let mxd_11_12: Color = col_11.mix(col_12, pos_old.0 - pos_11.x as f64);
                let mxd_21_22: Color = col_21.mix(col_22, pos_old.0 - pos_21.x as f64);
                let mxd_all: Color = mxd_11_12.mix(&mxd_21_22, pos_old.1 - pos_11.y as f64);
                return mxd_all
            }
        }).collect::<Vec<Color>>()).with_changed_size(size));
        self.data = new_pixelset;
        self.size = size
    }

    pub fn resized(&self, size: Size) -> Surface {
        let mut surface: Surface = (*self).clone();
        surface.resize(size);
        surface
    }

    pub fn fill(&mut self, color: Color) {
        self.data.fill(self.size, color);
    }

    pub fn filled(&mut self, color: Color) -> Surface {
        let mut surface: Surface = (*self).clone();
        surface.fill(color);
        surface
    }

    pub fn save_png(&self, path: &str) {
        let mut result: Vec<u8> = vec![];
        println!("Saving...");

        let result_: Vec<[u8; 4]> = self.data.data.iter().map(|c| {[c.r, c.g, c.b, (c.a*255.) as u8]}).collect();

        for item in result_.iter() {
            result.extend(item)
        }

        let result = result.as_bytes();

        image::save_buffer(path, result, self.size.x as u32, self.size.y as u32, image::ColorType::Rgba8).unwrap();
    }
}
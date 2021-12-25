use font_kit::family_name::FamilyName;
use font_kit::font::Font;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use super::representation::CACHE;

use super::prelude::*;


pub trait Effect {
    fn apply(object: &Vec<u8>);
}

pub trait Drawable {
    fn to_vec(&self) -> Vec<u8>;
    fn get_size(&self) -> (usize, usize);
    fn get_position(&self) -> (usize, usize);
    fn add_effect(effect: Box<dyn Effect>);
}

pub struct Text {
    pub text: String,
    pub font_family_name: String,
    pub font_size: usize,
}

pub struct Image {
    pub data: Vec<u8>,
    pub size: (usize, usize)
}

impl Drawable for Text {
    // Render text
    fn to_vec(&self) -> Vec<u8> {
        let output: Vec<u8> = Vec::new();
        let temp_heap: Vec<Vec<u8>> = Vec::new();
        for glyph in self.text.chars(){
            let font: Font = SystemSource::new()
                            .select_best_match(&[FamilyName::SansSerif], &Properties::new())
                            .unwrap()
                            .load()
                            .unwrap();
            if CACHE.contains_key(self.font_family_name.as_str()) {
                CACHE.insert(self.font_family_name.clone(), Vec::new());
            }
            let font_glyphs: &mut Vec<Vec<u8>> = CACHE.get_mut(self.font_family_name.as_str()).unwrap();
            let is_glyph_in_cache = font_glyphs.get(font.glyph_for_char(glyph).unwrap()).is_none();
            if !is_glyph_in_cache {
                font_glyphs.insert(0, vec![]);
            }
        }

        output
    }

    // Calculate size of rendered image
    fn get_size(&self) -> (usize, usize) {
        todo!()
    }

    fn get_position(&self) -> (usize, usize) {
        todo!()
    }

    fn add_effect(effect: Box<dyn Effect>) {
        todo!()
    }
}

impl Drawable for Image {
    fn to_vec(&self) -> Vec<u8> { self.data.copy() }
    fn get_size(&self) -> (usize, usize) { self.size.copy() }

    fn get_position(&self) -> (usize, usize) {
        todo!()
    }

    fn add_effect(effect: Box<dyn Effect>) {
        todo!()
    }
}


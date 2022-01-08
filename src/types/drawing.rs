use super::prelude::*;


pub trait Effect {
    fn apply(&self, object: PixelSet);
}

pub trait Drawable {
    fn render(&self) -> PixelSet;
    fn get_size(&self) -> Size;
    fn add_effect(&mut self, effect: Box<dyn Effect>);
}

pub struct Text {
    pub text: String,
    pub font_family_name: String,
    pub font_size: usize,
}

pub struct Image {
    data: Vec<u8>,
    size: Size,
    effects: Vec<Box<dyn Effect>>
}

impl Drawable for Text {
    /// Render text
    fn render(&self) -> PixelSet {
        todo!()
    }

    /// Calculate size of rendered image
    fn get_size(&self) -> Size {
        todo!()
    }

    fn add_effect(&mut self, _effect: Box<dyn Effect>) {
    }
}

impl Drawable for Image {
    fn render(&self) -> PixelSet {
        let mut set: PixelSet = self.data.clone().into();
        set.size = self.size;
        set
    }

    fn get_size(&self) -> Size {
        self.size
    }

    fn add_effect(&mut self, _effect: Box<dyn Effect>) {
        self.effects.append(&mut vec![_effect]);
    }
}
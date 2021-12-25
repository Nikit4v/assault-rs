use std::borrow::Borrow;
use std::collections::HashMap;
use std::iter::Sum;
use std::ops::{Index, Range};
use font_kit::family_name::FamilyName;
use crate::types::prelude::*;
use font_kit::font::Font;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;

pub(crate) static CACHE: &mut HashMap<String, Vec<Vec<u8>>> = HashMap::new().into();


/// Size in pixels.
/// x - vertical,
/// y - horizontal
#[derive(Copy, Clone, Debug)]
pub struct Size {
    pub x: usize,
    pub y: usize,
}

impl Size {
    pub(crate) fn new(x: usize, y: usize) -> Self {
        Size { x, y }
    }

    fn swap_xy(&self) -> Self {
        let mut result = self.clone();
        result.x += result.y;
        result.y = result.x - result.y;
        result.x -= result.y;
        result
    }

    pub fn into_linear(self) -> usize {
        self.x*self.y
    }

    pub fn null() -> Self {
        return Self::new(0, 0)
    }
}

impl From<(usize, usize)> for Size {
    fn from(item: (usize, usize)) -> Self {
        Self::new(item.0, item.1)
    }
}

impl Into<(usize, usize)> for Size {
    fn into(self) -> (usize, usize) {
        (self.x, self.y)
    }
}


/// Position in pixels.
///
/// x - vertical,
///
/// y - horizontal
#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn move_by_position(&mut self, position: Position) {
        self.x += position.x;
        self.y += position.y;
    }

    pub fn into_linear(self, size: Size) -> Result<usize, &'static str>{
        let linear = self.x*size.y+self.y;
        return if linear <= size.into_linear() {
            Ok(linear)
        } else {
            Err("Position is out of range")
        }
    }
}

impl From<(usize, usize)> for Position {
    fn from(item: (usize, usize)) -> Self {
        Self::new(item.0, item.1)
    }
}

impl Into<(usize, usize)> for Position {
    fn into(self) -> (usize, usize) {
        (self.x, self.y)
    }
}


/// Pixel color in rgba
#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

impl Color {
    fn new(r: u8, g: u8, b: u8, a: f32) -> Color {
        Color {r, g, b, a}
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(item: (u8, u8, u8)) -> Self {
        Color {r: item.0, g: item.1, b: item.2, a: 1 as f32 }
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(item: (u8, u8, u8, u8)) -> Self {
        Color {
            r: item.0,
            g: item.1,
            b: item.2,
            a: (item.3 / 256) as f32,
        }
    }
}

impl From<(u8, u8, u8, f32)> for Color {
    fn from(item: (u8, u8, u8, f32)) -> Self {
        Color {r: item.0, g: item.1, b: item.2, a: item.3 as f32 }
    }
}


#[derive(Clone, Debug)]
pub struct PixelSet {
    size: Size,
    data: Vec<Color>
}

impl PixelSet {
    pub fn empty() -> Self {
        return Self {
            size: Size::new(0, 0),
            data: vec![]
        }
    }
    pub fn new(size: Size, color: Color) -> Self {
        return Self {
            size: size.clone(),
            data: vec![color; size.into_linear()]
        }
    }

    pub fn get_size(&self) -> Size {
        self.size
    }

    pub fn fill(&mut self, size: Size, color: Color) {
        self.size = size.clone();
        self.data = vec![color; size.into_linear()];
    }
}

impl Index<Range<usize>> for PixelSet {
    type Output = Vec<Color>;

    fn index(&self, index: Range<usize>) -> &Self::Output {
        return self.data[index].into()
    }
}

impl Index<usize> for PixelSet {
    type Output = Color;

    fn index(&self, index: usize) -> &Self::Output {
        self.data[index].into()
    }
}

impl Into<Vec<Color>> for PixelSet {
    fn into(self) -> Vec<Color> {
        self.data
    }
}

impl Into<Vec<u8>> for PixelSet {
    fn into(self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        for pixel in self.data {
            result.extend([pixel.r, pixel.g, pixel.b, pixel.a])
        }
        return result
    }
}

impl From<Vec<Color>> for PixelSet {
    fn from(item: Vec<Color>) -> Self {
        return PixelSet {
            size: Size::null(),
            data: item
        }
    }
}

impl From<Vec<u8>> for PixelSet {
    fn from(item: Vec<u8>) -> Self {
        let iter = item.into_iter().enumerate();
        let r: Vec<u8> = iter.clone().filter(|&(i, _)| {i % 4 == 0}).map(|(_, e)| e).collect();
        let g: Vec<u8> = iter.clone().filter(|&(i, _)| {i % 4 == 1}).map(|(_, e)| e).collect();
        let b: Vec<u8> = iter.clone().filter(|&(i, _)| {i % 4 == 2}).map(|(_, e)| e).collect();
        let a: Vec<f32> = iter.clone().filter(|&(i, _)| {i % 4 == 3}).map(|(_, e)| e/256).collect();
        assert!((r.len() == g.len()) && (b.len() == a.len()) && (g.len() == b.len()));
        (0..r.len()).into_iter().map(|v| { Color::from((r[v], g[v], b[v], a[v])) }).collect()
    }
}
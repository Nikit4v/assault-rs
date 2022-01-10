use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut, Range, Sub};


static DEFAULT_RGBA: (u8, u8, u8, f32) = (0, 0, 0, 1_f32);


/// Size in pixels.
///
/// y - vertical,
///
/// x - horizontal
#[derive(Copy, Clone, Debug)]
pub struct Size {
    pub x: usize,
    pub y: usize,
}

impl Size {
    pub fn new(x: usize, y: usize) -> Self {
        Size { x, y }
    }

    pub fn null() -> Self {
        Self::new(0, 0)
    }

    pub fn swap_xy(&self) -> Self {
        #![allow(dead_code)]
        let mut result = *self;
        result.y += result.x;
        result.x = result.y - result.x;
        result.y -= result.x;
        result
    }

    pub fn into_linear(self) -> usize {
        self.y * self.x
    }

    pub fn iter_positions(&self) -> PositionIterator {
        PositionIterator::new(*self)
    }
}

impl From<(usize, usize)> for Size {
    fn from(item: (usize, usize)) -> Self {
        Self::new(item.0, item.1)
    }
}

impl From<Size> for (usize, usize) {
    fn from(item: Size) -> (usize, usize) {
        (item.y, item.x)
    }
}

impl From<(u32, u32)> for Size {
    fn from(item: (u32, u32)) -> Self {
        Self::new(item.0 as usize, item.1 as usize)
    }
}

impl From<Size> for (u32, u32) {
    fn from(item: Size) -> Self {
        (item.y as u32, item.x as u32)
    }
}

/// Position in pixels.
///
/// y - vertical,
///
/// x - horizontal
#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn move_by_position(&mut self, position: Position) {
        self.y += position.y;
        self.x += position.x;
    }

    pub fn into_linear(self, size: Size) -> Result<usize, &'static str> {
        let linear = self.y * size.x + self.x;
        if linear < size.into_linear() {
            Ok(linear)
        } else {
            Err("Position is out of range")
        }
    }

    pub fn add_if_out_of_size(&self, size: Size, x: i16, y: i16) -> Self {
        let mut new_pos = *self;
        if !(new_pos.x < size.x) {
            new_pos.x = (x + new_pos.x as i16) as usize;
        }
        if !(new_pos.y < size.y) {
            new_pos.y = (y + new_pos.y as i16) as usize;
        }
        return new_pos;
    }
}

impl From<(usize, usize)> for Position {
    fn from(item: (usize, usize)) -> Self {
        Self::new(item.0, item.1)
    }
}

impl From<Position> for (usize, usize) {
    fn from(item: Position) -> Self {
        (item.x, item.y)
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Position) -> Self::Output {
        Self {
            y: self.y - other.y,
            x: self.x - other.x
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
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
    pub fn new(r: u8, g: u8, b: u8, a: f32) -> Color {
        Color { r, g, b, a }
    }

    pub fn default() -> Color {
        Color::from(DEFAULT_RGBA)
    }


    pub fn mix(&self, other: &Color, coefficient: f64) -> Color {
        Self {
            r: (self.r as f64 * coefficient + other.r as f64 * (1. - coefficient)) as u8,
            g: (self.g as f64 * coefficient + other.g as f64 * (1. - coefficient)) as u8,
            b: (self.b as f64 * coefficient + other.b as f64 * (1. - coefficient)) as u8,
            a: self.a * coefficient as f32 + other.a * (1. - coefficient as f32)
        }
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(item: (u8, u8, u8)) -> Self {
        Color {
            r: item.0,
            g: item.1,
            b: item.2,
            a: 1_f32,
        }
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(item: (u8, u8, u8, u8)) -> Self {
        Color {
            r: item.0,
            g: item.1,
            b: item.2,
            a: (item.3 / 255) as f32,
        }
    }
}

impl From<(u8, u8, u8, f32)> for Color {
    fn from(item: (u8, u8, u8, f32)) -> Self {
        Color {
            r: item.0,
            g: item.1,
            b: item.2,
            a: item.3 as f32,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.r, self.g, self.b, (self.a*255.).round() as u8)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        return if (self.r == other.r) &&
            (self.g == other.g) &&
            (self.b == other.g) &&
            (self.a == other.a) {
            true
        } else {
            false
        }
    }
}


#[derive(Clone, Debug)]
pub struct PixelSet {
    pub size: Size,
    pub data: Vec<Color>,
}

impl PixelSet {
    pub fn null() -> Self {
        Self {
            size: Size::null(),
            data: vec![],
        }
    }

    pub fn new(size: Size, color: Color) -> Self {
        Self {
            size,
            data: vec![color; size.into_linear()],
        }
    }

    pub fn from_rgb8(rgb8: &[u8], size: Size) -> Self {
        let iter = rgb8.into_iter().enumerate();
        let r: Vec<u8> = iter
            .clone()
            .filter(|&(i, _)| i % 3 == 0)
            .map(|(_, e)| *e)
            .collect();
        let g: Vec<u8> = iter
            .clone()
            .filter(|&(i, _)| i % 3 == 1)
            .map(|(_, e)| *e)
            .collect();
        let b: Vec<u8> = iter
            .clone()
            .filter(|&(i, _)| i % 3 == 2)
            .map(|(_, e)| *e)
            .collect();
        assert_eq!(r.len(), g.len());
        assert_eq!(r.len(), b.len());
        let res: Vec<Color> = (0..r.len())
            .into_iter()
            .map(|v| Color::new(r[v], g[v], b[v], 1.))
            .collect();
        let mut res: Self = res.into();
        res.size = size;
        res
    }


    pub fn fill(&mut self, size: Size, color: Color) {
        self.size = size;
        self.data = vec![color; size.into_linear()];
    }

    pub fn get_color(&self, position: Position) -> Result<Color, &str> {
        Ok(self.data[position.into_linear(self.size)?])
    }

    pub fn get_color_ref(&self, position: Position) -> &Color {
        match position.into_linear(self.size.clone()) {
            Ok(linear_position) => {
                self.data.index(linear_position)
            }
            Err(_) => {
            panic!("{} is incorrect with size ({}, {})", position, self.size.x, self.size.y)
            }
        }
    }

    pub unsafe fn get_color_ref_unchecked(&self, position: &Position)-> &Color {
        self.data.index(position.y * self.size.x + position.x)
    }

    pub fn set_color(&mut self, position: Position, value: Color) -> Result<(), &str> {
        self.data[position.into_linear(self.size)?] = value; Ok(())
    }

    pub fn with_changed_size(&self, size: Size) -> PixelSet {
        let mut new = self.clone();
        new.size = size;
        return new;
    }
}

impl Index<Range<usize>> for PixelSet {
    type Output = [Color];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.data[index]
    }
}

impl Index<usize> for PixelSet {
    type Output = Color;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl From<Vec<Color>> for PixelSet {
    fn from(item: Vec<Color>) -> Self {
        PixelSet {
            size: Size::null(),
            data: item,
        }
    }
}

impl From<Vec<u8>> for PixelSet {
    fn from(item: Vec<u8>) -> Self {
        let iter = item.into_iter().enumerate();
        let r: Vec<u8> = iter
            .clone()
            .filter(|&(i, _)| i % 4 == 0)
            .map(|(_, e)| e)
            .collect();
        let g: Vec<u8> = iter
            .clone()
            .filter(|&(i, _)| i % 4 == 1)
            .map(|(_, e)| e)
            .collect();
        let b: Vec<u8> = iter
            .clone()
            .filter(|&(i, _)| i % 4 == 2)
            .map(|(_, e)| e)
            .collect();
        let a: Vec<f32> = iter
            .filter(|&(i, _)| i % 4 == 3)
            .map(|(_, e)| (e / 255) as f32)
            .collect();
        assert!((r.len() == g.len()) && (b.len() == a.len()) && (g.len() == b.len()));
        let res: Vec<Color> = (0..r.len())
            .into_iter()
            .map(|v| Color::from((r[v], g[v], b[v], a[v])))
            .collect();
        res.into()
    }
}

impl From<PixelSet> for Vec<Color> {
    fn from(item: PixelSet) -> Self {
        item.data
    }
}

impl From<PixelSet> for Vec<u8> {
    fn from(item: PixelSet) -> Self {
        let mut result: Vec<u8> = vec![];

        for pixel in item.data {
            result.extend([pixel.r, pixel.g, pixel.b, (pixel.a*255.).round() as u8])
        }
        result
    }
}

impl IndexMut<usize> for PixelSet {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}


pub struct PositionIterator(Size, usize, usize);

impl Iterator for PositionIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        let pos: Position = (self.1, self.2).into();  // Save value
        if self.2 >= self.0.y {  // return None if not exists
            return None;
        }
        self.1 += 1;  // move carriage
        if self.0.x == self.1 {
            self.1 = 0;  // revert carriage
            self.2 += 1;  // next line
        }
        return Some(pos);
    }
}

impl PositionIterator {
    fn new(size: Size) -> Self {
        Self(size, 0, 0)
    }
}

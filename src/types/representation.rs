use std::fmt::{Display, Formatter};
use std::ops::{Index, Range};


static DEFAULT_RGBA: (u8, u8, u8, f32) = (0, 0, 0, 1_f32);


/// Size in pixels.
/// x - vertical,
/// y - horizontal
#[derive(Copy, Clone, Debug)]
pub struct Size {
    pub x: usize,
    pub y: usize,
}

impl Size {
    pub fn new(x: usize, y: usize) -> Self {
        Size { x, y }
    }

    pub fn swap_xy(&self) -> Self {
        #![allow(dead_code)]
        let mut result = *self;
        result.x += result.y;
        result.y = result.x - result.y;
        result.x -= result.y;
        result
    }

    pub fn into_linear(self) -> usize {
        self.x * self.y
    }

    pub fn null() -> Self {
        Self::new(0, 0)
    }
}

impl From<(usize, usize)> for Size {
    fn from(item: (usize, usize)) -> Self {
        Self::new(item.0, item.1)
    }
}

impl From<Size> for (usize, usize) {
    fn from(item: Size) -> (usize, usize) {
        (item.x, item.y)
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
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn move_by_position(&mut self, position: Position) {
        self.x += position.x;
        self.y += position.y;
    }

    pub fn into_linear(self, size: Size) -> Result<usize, &'static str> {
        let linear = self.x * size.y + self.y;
        if linear <= size.into_linear() {
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

impl From<Position> for (usize, usize) {
    fn from(item: Position) -> Self {
        (item.x, item.y)
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

    pub fn fill(&mut self, size: Size, color: Color) {
        self.size = size;
        self.data = vec![color; size.into_linear()];
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
use crate::types::prelude::*;

#[derive(Copy, Clone)]
pub struct DrawingOptions {
    pub fill: Color,
    pub offset: Position,
    pub limit: Size,
}

impl DrawingOptions {
    pub fn generate(size: Size) -> DrawingOptions {
        DrawingOptions {
            fill: Color::new(0, 0, 0, 1_f32),
            offset: Position::new(0, 0),
            limit: size,
        }
    }
}
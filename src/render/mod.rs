use crate::render::Instruction::Overlap;
use crate::types::prelude::*;

#[derive(Copy, Clone)]
pub struct DrawingOptions {
    pub fill: Color,
    pub offset: Position,
    pub limit: Size,
}

impl DrawingOptions {
    fn generate(size: Size) -> DrawingOptions {
        return DrawingOptions {
            fill: Color::from((0, 0, 0)),
            offset: Position::from((0, 0)),
            limit: size
        }
    }
}

pub enum Instruction {
    Overlap,
    Mask,
}

pub trait Surface {
    fn draw(&mut self, instruction: Instruction, object: &impl Drawable, options: DrawingOptions);

    fn map_subsurface(&self, size: Size, position: Position) -> ProxySurface;
}

pub struct FrameSurface {
    size: Size,
    data: PixelSet,
}

impl Surface for FrameSurface {
    fn draw(&self, instruction: Instruction, object: &impl Drawable, options: DrawingOptions) {
        todo!()
    }

    fn map_subsurface(&self, size: Size, position: Position) -> ProxySurface {
        return ProxySurface::new(Box::new(self), size, position)
    }
}

impl FrameSurface {
    fn generate(size: Size, fill: Color) -> FrameSurface {
        let mut set = PixelSet::empty();
        set.fill(size, fill);
        return FrameSurface {
            size,
            data: set
        }
    }
}

pub struct ProxySurface {
    parent_surface: Box<dyn Surface>,
    size: Size,
    position: Position,
}

impl ProxySurface {
    fn new(parent_surface: Box<dyn Surface>, size: Size, position: Position) -> Self {
        return ProxySurface {
            parent_surface,
            size,
            position,
        }
    }
}

impl Surface for ProxySurface {
    fn draw(&mut self, instruction: Instruction, object: &impl Drawable, options: DrawingOptions) {
        let mut modified_option = options.clone();
        modified_option.offset.move_by_position(self.position);

        if modified_option.limit.x > self.size.x {
            modified_option.limit.x = self.size.x
        }
        if modified_option.limit.y > self.size.y {
            modified_option.limit.y = self.size.y
        }

        self.parent_surface.draw(instruction, object, options)
    }

    fn map_subsurface(&self, size: Size, position: Position) -> ProxySurface {
        return ProxySurface::new(Box::new(self), size, position)
    }
}

fn render_scene(objects: &[impl Drawable], scene_size: Size) {
    let mut scene = FrameSurface::generate(scene_size, Color::from((0, 0, 0)));
    for object in objects {
        scene.draw(Overlap, object, DrawingOptions::generate(scene_size));
    }
}
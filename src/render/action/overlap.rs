use crate::render::action::Action;
use crate::render::backend::Backend;

pub struct Overlap {
    position: (usize, usize),
    align: (i8, i8),
}

impl Action for Overlap {
    fn apply(&self, backend: &mut dyn Backend) {
        // backend.overlap()
        todo!()
    }
}
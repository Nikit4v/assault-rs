mod overlap;

use crate::render::backend::Backend;

pub trait Action {
    fn apply(&self, backend: &mut dyn Backend);
}
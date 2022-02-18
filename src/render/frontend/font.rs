use crate::render::backend::{Backend, ptr, Shape};

pub enum LoadingType {
    System,
    File
}

fn load_font<T: Backend>(path: &str) -> LoadedFont {
    LoadedFont {}
}


struct LoadedFont {

}
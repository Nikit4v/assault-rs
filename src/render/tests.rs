use crate::render::actions::{Action, Overlap};
use crate::render::DrawingOptions;
use crate::types::prelude::*;


#[test]
fn test_overlap_action() {
    let mut surface = PixelSet::new(Size::new(64, 64), Color::default());
    let mut obj_vec: Vec<Color> = Vec::new();
    for i in 0..surface.size.into_linear() {
        if i % 2 == 0 {
            obj_vec.append(&mut vec![Color::new(255, 255, 255, 0.5)]);
        } else {
            obj_vec.append(&mut vec![Color::new(255, 255, 255, 0.)])
        }
    }

    let mut expected_result: Vec<Color> = Vec::new();
    for i in 0..surface.size.into_linear() {
        if i % 2 == 0 {
            expected_result.append(&mut vec![Color::new(255/2, 255/2, 255/2, 0.5)]);
        } else {
            expected_result.append(&mut vec![Color::new(0, 0, 0, 1.)])
        }
    }

    Overlap::apply(&mut surface, &(obj_vec.into()), DrawingOptions::generate(surface.size.clone()));
    assert_eq!(surface.data, expected_result);
}
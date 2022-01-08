use crate::types::prelude::*;

#[test]
fn test_pixel_set_from_vec_u8() {
    let size = Size::new(64, 64);
    let pixel_set: PixelSet = vec![0; size.into_linear()].into();
    println!("{}", pixel_set[0]);
}

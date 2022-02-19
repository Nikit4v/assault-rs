extern crate assault;

use image::ColorType;
use assault::render::backend::Backend;
use assault::render::backend::cpu_backend::CpuBackend;
use assault::render::frontend::image::load_png;


fn main() {
    let mut backend = CpuBackend::new();
    let image = load_png(r"/Users/aleksejzmeevyh/Downloads/image_2022-02-02_00-59-52.png", &mut backend).expect("Cannot load image");


    let image_ptr = backend.allocate_frame((image.shape.0-100, image.shape.1-100, image.shape.2));
    let alpha_ptr = backend.allocate_frame((image.shape.0-100, image.shape.1-100, image.shape.2));


    let empty_ptr = backend.allocate_frame((image.shape.0-100, image.shape.1-100, image.shape.2));

    let image2_ptr= backend.allocate_frame((image.shape.0-100, image.shape.1-100, image.shape.2));
    let alpha2_ptr = backend.allocate_frame((image.shape.0-100, image.shape.1-100, image.shape.2));

    let output_ptr = backend.allocate_frame((image.shape.0-100, image.shape.1-100, image.shape.2));

    backend.resize_canvas(image.image_ptr, image_ptr, (0, 0, 0)).unwrap();
    // backend.
    if let Some(ptr) = image.alpha_mask_ptr {
        backend.resize_canvas(ptr, alpha_ptr, (0, 0, 0)).unwrap();
    }
    let helper = backend.allocate_frame((image.shape.0-100, image.shape.1-100, image.shape.2));

    backend.overlap(empty_ptr, image_ptr, alpha_ptr, output_ptr).unwrap();


    backend.resize_canvas(image.image_ptr, image2_ptr, (-100, -100, 0)).unwrap();
    if let Some(ptr) = image.alpha_mask_ptr {
        backend.resize_canvas(ptr, alpha2_ptr, (-100, -100, 0)).unwrap();
    }
    // backend.map(helper, helper, |(x, y, _), _| { if (x/8 % 2 == 0) ^ (y/8 % 2 == 0) {255} else {0} }).unwrap();
    // backend.overlap(alpha2_ptr, helper, helper, alpha2_ptr).unwrap();
    let (out_frame, shape) = backend.export_frame(alpha2_ptr);
    image::save_buffer("./alpha.png", out_frame.as_slice(), shape.0 as u32, shape.1 as u32, ColorType::Rgb8).unwrap();
    backend.overlap(
        output_ptr,
        image2_ptr,
        alpha2_ptr,
        output_ptr
    ).unwrap();
    let (out_frame, shape) = backend.export_frame(image_ptr);
    image::save_buffer("./img.png", out_frame.as_slice(), shape.0 as u32, shape.1 as u32, ColorType::Rgb8).unwrap();
}

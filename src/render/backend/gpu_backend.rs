use crate::render::backend::Shape;
use super::{Backend, TransactionResult, ptr};
static OVERLAP_SHADER: &str = "";

struct GpuBackend {

}
//
// impl Backend for GpuBackend {
//     fn allocate_frame(&mut self, shape: Shape) -> ptr {
//         todo!()
//     }
//
//     fn load_frame(&mut self, output_frame_ptr: ptr, raw_data: &[u8]) {
//         todo!()
//     }
//
//     fn export_frame(&self, source_frame_ptr: ptr) -> (Vec<u8>, Shape) {
//         todo!()
//     }
//
//     fn overlap(&mut self, source_frame_ptr: ptr, target_frame_ptr: ptr, alpha_frame_ptr: ptr, output_frame_ptr: ptr) -> TransactionResult {
//         todo!()
//         // let shader = shader!("shader text" in device as "Shader");
//     }
//
//     fn resize(&mut self, source_frame_ptr: ptr, output_frame_ptr: ptr) -> TransactionResult {
//         // let shader = shader!("shader text" in device as "Shader");
//         todo!()
//     }
//
//     fn extend(&mut self, source_frame_ptr: ptr, output_frame_ptr: ptr, offset: Shape) -> TransactionResult {
//         todo!()
//     }
// }

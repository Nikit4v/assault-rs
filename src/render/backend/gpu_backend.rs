use crate::render::backend::Shape;
use super::{Backend, TransactionResult, ptr};

struct GpuBackend {

}

impl Backend for GpuBackend {
    fn allocate_frame(&mut self, shape: Shape) -> ptr {
        todo!()
    }

    fn load_frame(&mut self, output_frame_ptr: ptr, raw_data: &[u8]) {
        todo!()
    }

    fn overlap(&mut self, source_frame_ptr: ptr, target_frame_ptr: ptr, alpha_frame_ptr: ptr, output_frame_ptr: ptr) -> TransactionResult {
        todo!()
    }

    fn resize(&mut self, source_frame_ptr: ptr, output_frame_ptr: ptr, output_shape: Shape) -> TransactionResult {
        todo!()
    }

    fn extend(&mut self, source_frame_ptr: ptr, output_frame_ptr: ptr, output_shape: Shape, offset: Shape) -> TransactionResult {
        todo!()
    }
}
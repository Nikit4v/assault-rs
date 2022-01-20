use super::{Backend, TransactionResult, ptr};

struct GpuBackend {

}

impl Backend for GpuBackend {
    fn allocate_frame(&mut self) -> ptr {
        todo!()
    }

    fn load_frame(&mut self, output_frame_ptr: ptr, raw_data: &[u8], shape: (usize, usize, usize)) {
        todo!()
    }

    fn overlap(&mut self, source_frame_ptr: ptr, target_frame_ptr: ptr, alpha_frame_ptr: ptr, output_frame_ptr: ptr) -> TransactionResult {
        todo!()
    }

    fn resize(&mut self, source_frame_ptr: ptr, output_frame_ptr: ptr, output_resolution: (usize, usize)) -> TransactionResult {
        todo!()
    }
}
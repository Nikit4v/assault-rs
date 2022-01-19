use super::{Backend, TransactionResult, ptr};

struct GpuBackend {

}

impl Backend for GpuBackend {
    fn load_frame(&mut self, raw_data: &[u8], shape: (usize, usize, usize)) -> ptr {
        todo!()
    }

    fn overlap(&mut self, source_frame_ptr: ptr, target_frame_ptr: ptr, output_frame_ptr: ptr, coefficient: f64) -> TransactionResult {
        todo!()
    }

    fn resize(&mut self, source_frame_ptr: ptr, output_frame_ptr: ptr, output_resolution: (usize, usize)) -> TransactionResult {
        todo!()
    }
}
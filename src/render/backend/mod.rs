pub mod gpu_backend;
pub mod cpu_backend;

pub(crate) type TransactionResult = Result<(), String>;

#[allow(non_camel_case_types)]
pub(crate) type ptr = usize;

pub(crate) type Shape = (usize, usize, usize);

pub trait Backend {
    fn allocate_frame(&mut self, shape: Shape) -> ptr;

    fn load_frame(&mut self, output_frame_ptr: ptr, raw_data: &[u8]);

    fn export_frame(&self, source_frame_ptr: ptr) -> (Vec<u8>, Shape);

    fn overlap(&mut self, source_frame_ptr: ptr, target_frame_ptr: ptr, alpha_frame_ptr: ptr, output_frame_ptr: ptr) -> TransactionResult;

    fn resize_content(&mut self, source_frame_ptr: ptr, output_frame_ptr: ptr) -> TransactionResult;

    fn resize_canvas(&mut self, source_frame_ptr: ptr, output_frame_ptr: ptr, offset: (i128, i128, i128)) -> TransactionResult;
}
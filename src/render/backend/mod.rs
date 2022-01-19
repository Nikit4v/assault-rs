pub mod gpu_backend;
pub mod cpu_backend;

type TransactionResult = Result<(), String>;

#[allow(non_camel_case_types)]
type ptr = usize;

pub trait Backend {
    fn allocate_frame(&mut self) -> ptr;

    fn load_frame(&mut self, output_frame_ptr: ptr, raw_data: &[u8], shape: (usize, usize, usize));

    fn overlap(&mut self, source_frame_ptr: ptr, target_frame_ptr: ptr, output_frame_ptr: ptr, coefficient: f64, align: (i8, i8), position: (usize, usize)) -> TransactionResult;

    fn resize(&mut self, source_frame_ptr: ptr, output_frame_ptr: ptr, output_resolution: (usize, usize)) -> TransactionResult;
}
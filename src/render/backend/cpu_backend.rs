use ndarray::Array3;
use super::{
    TransactionResult,
    Backend,
    ptr
};

pub struct CpuBackend {
    data: Vec<Array3<u8>>
}

impl Backend for CpuBackend {
    fn allocate_frame(&mut self) -> ptr {
        todo!()
    }

    fn load_frame(&mut self, output_frame_ptr: ptr, raw_data: &[u8], shape: (usize, usize, usize)) {
        let vec = Vec::from(raw_data);
        let ndar = Array3::<u8>::from_shape_vec(shape, vec).unwrap();
        self.data[output_frame_ptr] = ndar;
    }

    fn overlap(&mut self, source_frame_ptr: ptr, target_frame_ptr: ptr, output_frame_ptr: ptr, coefficient: f64, align: (i8, i8), position: (usize, usize)) -> TransactionResult {
        todo!()
    }


    fn resize(&mut self, source_frame_ptr: usize, output_frame_ptr: usize, output_resolution: (usize, usize)) -> TransactionResult {
        todo!()
    }
}
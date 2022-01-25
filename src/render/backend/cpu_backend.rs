use ndarray::{Array3, s, Zip};
use crate::render::backend::Shape;

use super::{
    TransactionResult,
    Backend,
    ptr
};

pub struct CpuBackend {
    data: Vec<Array3<u8>>
}

impl Backend for CpuBackend {
    fn allocate_frame(&mut self, shape: Shape) -> ptr {
        self.data.push(Array3::default(shape));
        self.data.len()-1
    }

    fn load_frame(&mut self, output_frame_ptr: ptr, raw_data: &[u8]) {
        let vec = Vec::from(raw_data);
        let ndar = Array3::<u8>::from_shape_vec(self.data[output_frame_ptr].dim(), vec).unwrap();
        self.data[output_frame_ptr] = ndar;
    }

    fn overlap(&mut self, source_frame_ptr: ptr, target_frame_ptr: ptr, alpha_frame_ptr: ptr, output_frame_ptr: ptr) -> TransactionResult {
        assert_eq!(self.data[source_frame_ptr].shape(), self.data[target_frame_ptr].shape());
        let source_frame = self.data[output_frame_ptr].view();
        let target_frame = self.data[target_frame_ptr].view();
        let alpha_frame = self.data[alpha_frame_ptr].view();
        let mut output_frame = Array3::<u8>::default(self.data[source_frame_ptr].raw_dim());
        let zip = Zip::from(&mut output_frame).and(&source_frame).and(&target_frame).and(&alpha_frame);
        zip.par_apply(|o, &s, &t, &a| {
            if a != 0 {
                *o = (s as f32 * (a as f32/255.) + t as f32 * (1. - a as f32/255.)) as u8;
            } else {
                *o = s;
            }
        });
        self.data[output_frame_ptr] = output_frame;
        Ok(())
    }

    fn resize(&mut self, source_frame_ptr: ptr, output_frame_ptr: ptr) -> TransactionResult {
        todo!()
    }

    fn extend(&mut self, source_frame_ptr: ptr, output_frame_ptr: ptr, output_alpha_frame_ptr: ptr, offset: Shape) -> TransactionResult {
        todo!()
    }
}
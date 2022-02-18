use ndarray::{Array3, ArrayBase, OwnedRepr, s, Zip};
use rayon::prelude::*;
use crate::render::backend::Shape;

use super::{
    TransactionResult,
    Backend,
    ptr
};

pub struct CpuBackend {
    data: Vec<Array3<u8>>
}

impl CpuBackend {
    pub fn new() -> Self {
        Self {
            data: vec![]
        }
    }

    pub fn export_to_ndarray(&self, source_frame_ptr: ptr) -> Array3<u8> {
        self.data[source_frame_ptr].clone()
    }

    pub fn map(&mut self, source_frame_ptr: ptr, output_frame_ptr: ptr, predicate: fn((usize, usize, usize), u8) -> u8) -> TransactionResult {
        let x: Vec<u8> = self.data[source_frame_ptr].indexed_iter().map(|((x, y, z), v)| {predicate((x, y, z), *v)}).collect();
        self.data[output_frame_ptr] = Array3::from_shape_vec(self.data[output_frame_ptr].raw_dim(),x).expect("Cannot map with source and output frames with not matching shapes");
        Ok(())
    }
}

impl Default for CpuBackend {
    fn default() -> Self {
        Self::new()
    }
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

    fn export_frame(&self, source_frame_ptr: ptr) -> (Vec<u8>, Shape) {
        let shape = self.data[source_frame_ptr].shape();
        (Vec::from(self.data[source_frame_ptr].as_slice().unwrap()), (shape[0], shape[1], shape[2]))
    }

    fn overlap(&mut self, source_frame_ptr: ptr, target_frame_ptr: ptr, alpha_frame_ptr: ptr, output_frame_ptr: ptr) -> TransactionResult {
        assert_eq!(self.data[source_frame_ptr].shape(), self.data[target_frame_ptr].shape());
        let source_frame = self.data[output_frame_ptr].view();
        let target_frame = self.data[target_frame_ptr].view();
        let alpha_frame = self.data[alpha_frame_ptr].view();
        let mut output_frame = Array3::<u8>::default(self.data[source_frame_ptr].raw_dim());
        let zip = Zip::from(&mut output_frame).and(&source_frame).and(&target_frame).and(&alpha_frame);
        zip.par_apply(|o, &s, &t, &a| {
            *o = (s as f32 * (1. - a as f32/255.) + t as f32 * (a as f32/255.)) as u8;
        });
        self.data[output_frame_ptr] = output_frame;
        Ok(())
    }

    fn resize_content(&mut self, source_frame_ptr: ptr, output_frame_ptr: ptr) -> TransactionResult {
        todo!()
    }

    fn resize_canvas(&mut self, source_frame_ptr: ptr, output_frame_ptr: ptr, offset: (i128, i128, i128)) -> TransactionResult {
        let iter = self.data[source_frame_ptr].clone();
        for ((idx, idy, idz), item) in iter.indexed_iter() {
            if idx as i128 + offset.0 >= self.data[output_frame_ptr].shape()[0] as i128 || idx as i128 +offset.0 < 0 {
                continue
            }
            if idy as i128 + offset.1 >= self.data[output_frame_ptr].shape()[0] as i128 || idy as i128 +offset.1 < 0 {
                continue
            }
            if idz as i128 + offset.2 >= self.data[output_frame_ptr].shape()[0] as i128 || idz as i128 +offset.2 < 0 {
                continue
            }
            self.data[output_frame_ptr][[(idx as i128 + offset.0) as usize, (idy as i128 + offset.1) as usize, (idz as i128 + offset.2) as usize]] = *item
        }
        Ok(())
    }
}
use ndarray::Array3;

pub trait Backend {

}


pub struct CpuBackend {
    data: Array3<u8>
}
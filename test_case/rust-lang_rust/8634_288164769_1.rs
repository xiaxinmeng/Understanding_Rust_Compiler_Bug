rust
pub trait Blas: Vector<f32> + Vector<f64> + MatrixVector<f32> ... { }

impl<I: Vector<f32> + Vector<f64> + MatrixVector<f32> ...> Blas for I { } 

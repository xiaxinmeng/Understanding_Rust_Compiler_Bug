 rust
fn approx_eq_eps(&self, other: &f32, absolute_epsilon: &f32, relative_epsilon: &f32) -> bool {
        match (*self - *other).abs() {
            // check absolute error
            n if n < *absolute_epsilon => true,
            // check relative error
            n => n < self.abs().max(&other.abs()) * (*relative_epsilon),
        }
    }
}


fn approx_eq_eps(&self, other: &f32, approx_epsilon: &f32) -> bool {
    // check absolute error.
    if (*self - *other).abs() < *approx_epsilon { true }    
    else {
        // check relative error.
        (*self - *other).abs() < fmax(self.abs(), other.abs()) * (*approx_epsilon)
    }
}

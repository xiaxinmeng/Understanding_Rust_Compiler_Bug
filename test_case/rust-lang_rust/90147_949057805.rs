rust
#![warn(rust_2021_incompatible_closure_captures)]

struct Complex<T>(T);
pub struct FeynmanState<P> {
    _substate: Vec<Vec<Complex<P>>>,
}
impl<P> FeynmanState<P> {
    pub fn calculate_amplitude(&self) -> () {
        todo!()
    }
    pub fn into_state(self) {
        || self.calculate_amplitude();
    }
}

fn main() {
}

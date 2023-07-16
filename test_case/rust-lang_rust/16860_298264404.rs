rust
pub trait Any {
    fn a(&self) {}

    fn b(&mut self) {
        // Fails
        self.a();

        // Works
        (*self).a();
    }
}

impl<T: 'static> Any for T {}

fn main() {}

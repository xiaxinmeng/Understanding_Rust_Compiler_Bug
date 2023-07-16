rust
use std::cell::Cell;

struct X { 
    inner: Cell<u8>,
}

impl X { 
    fn func1(&mut self) -> &u8 {
        // We return the &mut u8 from get_mut as &u8.
        self.inner.get_mut()
    }   
    fn func2(&self, v: &u8) {
        // When this function is called from main,
        // v points to the u8 in the cell self.inner.

        println!("Before: {}", v);

        // The cell type allows us to set its inner data
        // using an immutable reference to the cell.
        self.inner.set(6);

        // Oops.
        println!("After: {}", v); 
    }   
}

fn main() {
    let mut x = X { inner: Cell::new(5) };
    let v: &u8 = x.func1(); // Here, we borrow x mutably...
    x.func2(v); // ... and here immutably.
}

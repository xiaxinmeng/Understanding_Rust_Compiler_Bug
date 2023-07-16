 rust
// this `y` will never run Drop
let y = unsafe {
        let mut y: X = std::mem::uninitialized();
        y.x = 1;
        y
    };

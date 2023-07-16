rust
union TransmuteHack<T: Copy> {
    from: T,
    to: usize,
}

pub static VALUE: usize = 42;
pub static ADDRESS: usize = unsafe { TransmuteHack { from: &VALUE }.to };

pub static AD2: usize = ADDRESS/2; // Error: Cannot divide a pointer by 2

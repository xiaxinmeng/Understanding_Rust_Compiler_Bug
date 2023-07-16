rust

pub fn create_boxed_array_deref() -> Box<[u16; 100]> {
    const A: [u16; 100] = [0;100];
    Box::new(A)
}


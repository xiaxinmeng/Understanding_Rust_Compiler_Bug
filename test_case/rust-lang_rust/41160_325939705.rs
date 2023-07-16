rust

pub fn create_boxed_array_deref() -> Box<[u16; 100]> {
    Box::new(*&[0u16; 100])
}


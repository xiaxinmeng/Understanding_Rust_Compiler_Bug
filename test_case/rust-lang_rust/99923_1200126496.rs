rust
union Transmute<T: Copy, U: Copy> {from:T,to:U}

pub static PTR_TO_USIZE: usize = unsafe { Transmute{from:&0}.to };

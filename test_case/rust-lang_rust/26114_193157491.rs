 rust
﻿﻿#[repr(u64)]
enum Eu64NonCLike<T> {
    _None,
    _Some(T),
}

fn main() {
    println!("{}", std::mem::size_of::<Eu64NonCLike<u8>>());
}

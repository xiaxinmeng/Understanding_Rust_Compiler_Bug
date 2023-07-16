rust
#![feature(const_generics, const_evaluatable_checked)]

fn q<T, const N: usize>(_: T) -> [u8; N + 1] {
    todo!()
}

fn supplier<T>() -> T {
    todo!()
}

fn catch_me() {
    let mut x = supplier();
    x = q(x);
}

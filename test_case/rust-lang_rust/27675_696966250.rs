rust
#![feature(unsize)]

trait Setup<T> {
    type From: std::marker::Unsize<[T]>;
}

fn unsize<T, U: Setup<T> + ?Sized>(from: &U::From) -> &[T] {
    from
}

fn unsize_to_any<T, U>(t: &T) -> &U {
    &unsize::<U, dyn Setup<U, From=[T]>>(core::slice::from_ref(t))[0]
}

fn main() {
    let trust_me_im_64: u8 = 0;
    println!("{}", unsize_to_any::<u8, u64>(&trust_me_im_64))
}

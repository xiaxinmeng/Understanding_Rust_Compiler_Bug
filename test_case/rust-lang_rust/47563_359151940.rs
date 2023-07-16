rust
#![feature(never_type)]

enum Helper<T, U> {
    T(T, [!; 1]),
    #[allow(dead_code)]
    U(U),
}

fn make_the_array() -> [!; 1] {
    panic!("whoops!");
}

fn transmute<T, U>(t: T) -> U {
    let Helper::U(u) = Helper::T(t, make_the_array()); // ??
    u
}

fn main() {
    println!("{:?}", transmute::<&str, (*const u8, u64)>("type safety"));
}

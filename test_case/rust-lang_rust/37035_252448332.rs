 rust
struct S<T, U = u16> {
    a: T,
    b: U,
}

type Alias<T> = S<T>;

fn main() {
    // Everything below is legal
    let s = S::<u8, u8> { a: 0, b: 1 }; // desugared into S::<u8, u8> { a: 0, b: 1 };
    let s = S::<u8> { a: 0, b: 1 }; // desugared into S::<u8, u16> { a: 0, b: 1 };
    let s = S { a: 0, b: 1 }; // desugared into S::<_, _> { a: 0, b: 1 };
    let s = Alias::<u16> { a: 0, b: 1 }; // desugared into S::<u16, u16> { a: 0, b: 1 };
}

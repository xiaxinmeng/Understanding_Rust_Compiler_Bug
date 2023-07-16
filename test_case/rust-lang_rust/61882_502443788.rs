rust
struct A<T>(T);

impl A<bool> {
    const B: A<u8> = Self::<u8>(0);
}

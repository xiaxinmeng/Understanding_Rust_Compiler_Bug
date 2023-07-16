 rust
fn main() {
    type T<'a> = &'a &'static i8;
    fn s(m: T) -> &i8 { m }
    static S: i8 = 1;
    static S2: &'static i8 = &S;
    s(&S2);
}

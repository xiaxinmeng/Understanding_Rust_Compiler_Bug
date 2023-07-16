 rust
mod a {
    pub struct Aliased1 { pub x: u32 }
    pub type Public2 = Aliased1;

    pub struct Unaliased3;

    pub struct Aliased4;
    pub type Public5 = Aliased4;
    #[allow(non_upper_case_globals)]
    pub const Public5: Aliased4 = Aliased4;
}
fn main() {
    use a;
    let _p = a::Public2 { x: 10 };
    let _q = a::Unaliased3;
    let _r = a::Public5;
}

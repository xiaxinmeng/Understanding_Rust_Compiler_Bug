rust
// `main` scope
//      vvvvvvv
pub mod foo_mod {
    // `foo!` scope
    //        vvv
    pub const BAR: u32 = 123;
}


pub reexport a::f;

#[unstable(feature = "foo", ...)]
pub mod a {
    #[unstable(feature = "bar", ...)]
    pub fn f() { }
}

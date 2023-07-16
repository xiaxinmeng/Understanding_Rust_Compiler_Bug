
pub reexport a::f;

#[unstable(feature = "foo", ...)]
pub mod a {
    #[stable(feature = "bar", ...)]
    pub fn f() { }
}

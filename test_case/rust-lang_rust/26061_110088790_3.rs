
pub reexport a::f;

#[unstable(feature = "foo", ...)]
#[deprecated(...)]
pub mod a {
    #[stable(feature = "bar", ...)]
    pub fn f() { }
}

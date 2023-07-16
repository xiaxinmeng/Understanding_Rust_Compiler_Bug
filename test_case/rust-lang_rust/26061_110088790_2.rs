
pub reexport a::f;

#[unstable(feature = "foo", ...)]
#[deprecated(...)]
pub mod a {
    #[unstable(feature = "bar", ...)]
    pub fn f() { }
}

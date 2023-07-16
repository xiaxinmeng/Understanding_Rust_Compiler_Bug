
mod a {
    mod b {
        pub fn f() -> S { S }
        pub struct S;
    }
    pub use a::b::f;
}

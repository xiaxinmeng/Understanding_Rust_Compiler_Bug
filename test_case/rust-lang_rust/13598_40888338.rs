 rust
mod a {
    use b::c;

    use self::c::foo;
    use a::c::foo;
}

mod b {
    pub mod c {
        pub fn foo() {}
    }
}

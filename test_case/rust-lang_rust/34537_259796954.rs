
mod m {
    struct S;
    mod n {
        pub fn f() -> super::S { super::S }
    }

    fn g() {
        les s = n::f();
    }
}

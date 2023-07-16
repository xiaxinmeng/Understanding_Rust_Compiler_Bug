rust
const hello: u32 = 0;

mod a {
    pub macro get_hello() {
        super::hello
    }
}

mod b {
    const hello: u32 = 1;

    mod c {
        fn foo() -> u32 {
            super::super::a::get_hello!()
        }
    }
}

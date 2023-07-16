rust
    mod m {
        macro define_get_hello() {
            const hello: u32 = 0;

            pub macro get_hello() {
                hello
            }
        }

        define_get_hello!();
        pub const hello: u32 = 1;
    }

    fn main() {
        println!("hello is: {}", m::get_hello!());
    }
    
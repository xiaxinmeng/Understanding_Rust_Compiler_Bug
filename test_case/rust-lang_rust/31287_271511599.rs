rust
    fn main() {
        let foo = String::from("FOO");

        // The addition of this line prevents the subsequent `println!` from
        // printing out garbage data.
        println!("1: {:?}", foo);

        let foo2 = match 0 {
            0 if {
                some_func(foo) // foo is freed here
            } => unreachable!(),
            _ => {
                // Use After Free - we return freed memory
                foo
            }
        };

        println!("2: {:?}", foo2); // And here we access the invalid memory
    }

    fn some_func(foo: String) -> bool {
        drop(foo);
        false
    }

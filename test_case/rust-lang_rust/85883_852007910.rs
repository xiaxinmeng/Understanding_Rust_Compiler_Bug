rust
use foo::bbb;

fn main() {
    let bar = bbb(); // cannot specify any type signature for `bar`
    println!("{}", bar.some_inner_field);
    bar.some_method();
}

mod foo {
    use bar::Bar;

    pub fn bbb() -> Bar {
        Bar {
            some_inner_field: 42
        }
    }

    mod bar {
        pub struct Bar {
            pub some_inner_field: u8
        }
        
        impl Bar {
            pub fn some_method(&self) {
                println!("called a public method of an unnameable type");
            }
        }
    }
}

rust
struct S<T>(T);

default impl<T> S<T> { fn f() { println!("Hello world!") } }
impl S<u8> { fn f() { println!("Goodbye world!") } } // This currently errors

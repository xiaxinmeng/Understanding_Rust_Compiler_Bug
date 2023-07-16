rust
#[derive(Debug)]
struct Foo<T>(T);

fn main() {
    let f = Foo(42);
    
    let cls = |foo| {
        // println!("{:?}", foo); // Works
        foo.0; // Does not work
    };
    
    cls(f);
}

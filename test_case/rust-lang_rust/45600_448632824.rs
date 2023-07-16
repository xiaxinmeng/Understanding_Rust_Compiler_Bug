rust
struct Foo(String, String);
fn x(f: Foo) {
    match f {
        Foo(a, ref b) => {} 
    }
}

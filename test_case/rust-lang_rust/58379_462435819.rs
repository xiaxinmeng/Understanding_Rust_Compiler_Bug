rust
//#[derive(Copy, Clone)]
struct Alpha;

struct Beta;

struct Foo(pub Alpha, pub Beta);

//impl Drop for Foo { fn drop(&mut self) {} }

fn new(x: Foo) {
    let _a = x.0;
}

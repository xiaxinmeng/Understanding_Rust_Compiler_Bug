rust
struct Foo;

struct Bar<'a> { v: &'a Foo }

impl Foo {
    fn bar(&self) -> Bar<'_>{
        Bar{ v: self }
    }
}

fn call_static<T: 'static>(_: T) {}


fn main() {
    let foo = Foo;
    let bar = foo.bar();
    
    //This is actual problem 
    call_static(bar)
}

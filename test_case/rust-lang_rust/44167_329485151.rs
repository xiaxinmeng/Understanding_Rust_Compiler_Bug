rust
struct Foo {
    field: i32, 
}

fn foo2<'a, 'b>(a: &'a Foo, x: &'b i32) -> &'a i32 {
    if true {
        let p: &i32 = &a.field;
        &*p
    } else {
        &*x
    }
}

fn main() { }

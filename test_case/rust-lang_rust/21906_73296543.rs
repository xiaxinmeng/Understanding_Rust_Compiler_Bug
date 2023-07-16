 rust
struct Foo { data: Option<i32> }

fn main() {
    let mut x = Foo{data: Some(1)};

    foo(&mut x);
}

fn foo(x: &mut Foo) -> Option<&mut i32> {
    if let Some(y) = x.data.as_mut() {
        return Some(y);
    }

    println!("{:?}", x.data); 
    None
}

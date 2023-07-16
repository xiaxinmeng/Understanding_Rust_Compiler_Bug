 rust
enum Foo<'self> {
    V(&'self str)
}

fn f(arr: &[&Foo]) {
    for &f in arr.iter() {
        println!("{:?}", f);
    }
}

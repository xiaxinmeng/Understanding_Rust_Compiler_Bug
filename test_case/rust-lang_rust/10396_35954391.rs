 rust
enum Foo<'s> {
    V(&'s str)
}

fn f(arr: &[&Foo]) {
    for &f in arr.iter() {
        println!("{:?}", f);
    }
}

fn main() {}

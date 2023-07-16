 rust
enum Foo {
    Bar(~[int])
}

fn main() {
    let bar = Bar(~[1,2,3]);
    match bar {
        Bar(mut x) => {
            println!("x = {}", x.len());
            x = ~[1];
            println!("x = {}", x.len());
        }
    }
}

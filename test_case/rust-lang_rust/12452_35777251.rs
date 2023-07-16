 rust
enum Foo {
    Bar(~[int])
}

fn main() {
    let bar = Bar(~[1,2,3]);
    match bar {
        Bar(x) => {
            println!("x = {}", x.len());
            x.push(4);
            println!("x = {}", x.len());
        }
    }
}

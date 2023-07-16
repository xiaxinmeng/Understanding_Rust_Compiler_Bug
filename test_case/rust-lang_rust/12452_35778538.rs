 rust
enum Foo {
    Bar(int)
}

fn main() {
    let bar = Bar(1);
    match bar {
        Bar(x) => { // shouldn't x be immutable?
            println!("x = {}" ,x);
            x += 1;
            x += 2;
            println!("x = {}" ,x);
        }
    }
}

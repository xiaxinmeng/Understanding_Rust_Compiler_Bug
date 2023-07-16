rust
#[derive(Debug)]
struct Foo{}

const Foo: Foo = Foo{};

fn main(){
    let repeated = [Foo; 4];
    println!("{:?}", repeated);
}

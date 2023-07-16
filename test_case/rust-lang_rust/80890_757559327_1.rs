rust
#[derive(Debug)]
struct Foo;

fn main(){
    let repeated = [Foo; 4];
    println!("{:?}", repeated);
}

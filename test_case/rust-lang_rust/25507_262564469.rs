rust
#[derive(Debug)]
struct X;  // not Copy

struct Y{field: Option<X>}

fn main() {
    let v: Vec<Y> = Vec::new();
    for y in v.iter() {
        println!("{:?}", y.field.unwrap_or(X));
    }
}

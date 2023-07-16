rust
struct Outer;

mod foo {
    use super::Outer;
    
    mod test {
        use super::*;
        
        struct Dummy(Outer);
    }
}

fn main() {}

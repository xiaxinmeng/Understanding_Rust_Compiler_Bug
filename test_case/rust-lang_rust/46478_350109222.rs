rust
fn foo() { 
    let x = 10; 
    struct Bar; 
    macro_rules! foo { () => { x } }; 
    let y = foo!(); 
}

fn main() { }

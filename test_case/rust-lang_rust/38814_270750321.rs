rust
struct S { x: i32, y: i32 }
fn main() {
    let s = S { x: 10, y: 20 };
    match s {
        S {
            x: 11, // this is a field in a pattern ...
            y: the_y // ... and this is a field in a binding pattern
        } => println!("the_y: {}", the_y),
        
        S { x: the_x, y: 20 } => println!("the_x: {}", the_x),
        
        S { .. } => {}
    }
}

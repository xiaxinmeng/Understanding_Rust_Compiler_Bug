rust
fn main() {
    fn consumer(var: &mut i32) {
        *var += 1;
    }
    let closure = |var| {
        consumer(var);
        consumer(var);
    };
    
    let mut x: i32 = 0;
    closure(&mut x);
}

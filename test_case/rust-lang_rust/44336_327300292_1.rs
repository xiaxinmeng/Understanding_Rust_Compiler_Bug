rust
fn main() {
     let vec = vec![1,2,3];
     let f = move || vec.len();
     f(); // fine
     f(); // fine (f implements FnMut)
     vec.len(); // ERROR: value used after move (it belongs to the closure now)
}

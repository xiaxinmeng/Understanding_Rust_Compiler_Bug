 Rust
extern "C" { fn rand() -> i32; }

fn q() { //~ WARNING function cannot return without recurring
    if unsafe { rand()%2 == 0 } {
        q(); // this path recurs
    } else {
        panic!(); // this path does not terminate
    }
}
fn main() { q(); }

rust
//#[cfg(all())]  // unnecessary_cast goes away if uncommented
type T = u8;

fn main() {
    let _ = 0 as T;
}

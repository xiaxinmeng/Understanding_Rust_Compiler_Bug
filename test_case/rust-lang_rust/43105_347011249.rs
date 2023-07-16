rust
fn xyz() -> u8 { 42 }

const NUM: u8 = xyz();

fn main() {
    match 1 {
        NUM => unimplemented!(),
        _ => unimplemented!(),
    }
}

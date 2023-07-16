 Rust
extern mod a;
use a::*;

struct D;
impl Dummy for D;

fn main() {
    let x = D;
    x.message_box();
}

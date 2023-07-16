rust
trait U { type A; }
impl U for &u8 { type A = u32; }

fn main() {
    let _x: fn(<&u8 as U>::A) = |_| {};
}

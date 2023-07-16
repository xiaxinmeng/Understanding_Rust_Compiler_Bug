rust
pub enum MirPhase {
    Build = 0,
    Const = 1,
    Validated = 2,
}

fn main() {
    let _val = MirPhase::Build as usize;
}

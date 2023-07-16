
struct PrivateContext;

mod internals {
    pub fn entry_point(x: super::PrivateContext) { ... }
}

fn main() {
    let pc = PrivateContext;
    internals::entry_point(pc);
}

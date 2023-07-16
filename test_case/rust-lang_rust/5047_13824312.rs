
enum Variant { A, B(~u16) }

pub struct Blob {
    desc: Variant,
    sig: Variant
}

#[inline(never)]
fn bar() -> Variant { A }

fn main() {
    let a = bar();
    let _b = Blob {
        desc: B(~5),
        sig: copy a
    };
}

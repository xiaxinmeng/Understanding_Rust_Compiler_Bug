rust
#[allow(unconditional_recursion)]
fn recurse(f: impl Fn()) {
    recurse(&f)
}

fn main() {
    recurse(|| {});
}

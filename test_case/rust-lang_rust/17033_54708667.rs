 rust
fn f<'r>(p: &'r mut fn(p: &mut ())) {
    p(())
}

fn main() {}

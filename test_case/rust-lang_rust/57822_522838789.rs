
fn main() {
    let _: Vec<_> = [()].iter().map(break_here).collect();
}

fn break_here(x: &()) -> () {
    *x
}

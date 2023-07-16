 rust
fn foo(x: ||) {}

fn main() {
    'outer: loop {
        foo(|| {
            loop {
                break 'outer;
            }
        });
    }
}

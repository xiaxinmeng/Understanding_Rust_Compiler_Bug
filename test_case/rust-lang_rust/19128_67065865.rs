 rust
fn is_copy<T: Copy>(_: T) {}

fn main() {
    let i = 5i;
    is_copy(move |:| i);
    is_copy(|&:| {});
    is_copy(|&mut :| {});
    is_copy(|:| i);
    is_copy(|:| {});
}

rs
fn main() {
    foo::<Vec<usize>>();
}

fn foo<T> () {
    let _: [T; 64] = unsafe { std::mem::zeroed() }; // No warning
}

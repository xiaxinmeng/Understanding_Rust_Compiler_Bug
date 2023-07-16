rust
fn main() {
    let b = [1, 2, 3, 4, 5];
    let ptr = cast_slice::<u8, u64>(&b);
    for x in unsafe { &*ptr } {
        println!("{:016x}", x)
    }
}

fn cast_slice<T, U>(s: *const [T]) -> *const [U] {
    s as _
}

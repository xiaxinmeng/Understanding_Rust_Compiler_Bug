rust
#![feature(relaxed_struct_unsize)]
struct A<T, U: ?Sized + 'static>(T, B<T, U>);
struct B<T, U: ?Sized>(T, U);

fn main() {
    let x: A<[u32; 1], [u32; 1]> = A([0; 1], B([0; 1], [0; 1]));
    // This previously did not work as the last field of `A` also mentions `T`,
    // as only `U` changes this is now allowed thanks to this feature.
    let _y: &A<[u32; 1], [u32]> = &x;
}

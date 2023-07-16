rust
#[derive(Clone, Default)]
struct MaybeCopy<T>(T);

impl Copy for MaybeCopy<u8> {}

fn is_copy<T: Copy>(x: T) {
    println!("{}", std::any::type_name::<T>());
} 

fn main() {
    is_copy(MaybeCopy::default())
    // [MaybeCopy::default(); 13]
    // ^ doesn't work cause `Copy` is only checked in the mir here
}

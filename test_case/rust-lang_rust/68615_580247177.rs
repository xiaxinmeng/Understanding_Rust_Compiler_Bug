rust
#![feature(const_generics)]

struct Const<const V: [usize; 0]> {}
type MyConst = Const<{ [] }>;

fn main() {
    let _x = Const::<{ [] }> {}; // ERROR
    let _y = MyConst {}; // ERROR
}

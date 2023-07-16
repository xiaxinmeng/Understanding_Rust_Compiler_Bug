rust
#![feature(const_generics)] // <- Will not panic if removed

fn main() {
    use core::ops::Index;

    trait MyTrait {
        type ArrayType: Index<usize, Output = u8>;

        const SIZE: usize;
        const ARRAY: Self::ArrayType;
    }

    struct MyStruct;

    impl MyTrait for MyStruct {
        type ArrayType = [u8; Self::SIZE];

        const SIZE: usize = 4;
        const ARRAY: [u8; Self::SIZE] = [1, 2, 3, 4];
    }

    let res = <MyStruct as MyTrait>::ARRAY[2]; // PANICS HERE
    assert_eq!(res, 3);
}

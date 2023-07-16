rust
struct MyStruct<const N: usize>;

fn fact<const N: usize>() {
    MyStruct::<{ N - 1 }>;
}

fn main() {}

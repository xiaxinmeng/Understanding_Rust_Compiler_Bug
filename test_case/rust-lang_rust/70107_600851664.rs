rust
struct MyStruct<const N: usize>;

fn fact<const N: usize>() {
    let x: MyStruct<{ N - 1 }> = MyStruct;
}

fn main() {}

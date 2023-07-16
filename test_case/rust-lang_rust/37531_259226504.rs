 rust
enum TestEnum {
    Item(i32),
}

fn test(x: &mut i32) {
    println!("hi! {:?}", x);
}

fn main() {
    let mut x = TestEnum::Item(10);
    match x {
        TestEnum::Item(ref mut x) => {
            test(&mut x);
        }
    }
}

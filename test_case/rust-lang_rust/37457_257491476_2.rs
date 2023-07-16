 rust
#[no_mangle]
pub fn test() {
    let stack = OsStack::new(1 << 16).unwrap();
    let mut gen = Generator::new(stack, move |yielder, mut index| {
        let values = [1, 2, 3];
        loop { index = yielder.suspend(values[index]) }
    });

    println!("{:?}", gen.resume(5));
}

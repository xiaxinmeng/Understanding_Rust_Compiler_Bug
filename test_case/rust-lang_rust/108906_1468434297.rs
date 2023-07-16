rust
fn main() {
    let foo = async { 123 };
    let foo_size = std::mem::size_of_val(&foo);

    let bar0 = async move { foo.await };
    let bar0_size = std::mem::size_of_val(&bar0);

    let bar1 = async move { bar0.await };
    let bar1_size = std::mem::size_of_val(&bar1);

    let bar2 = async move { bar1.await };
    let bar2_size = std::mem::size_of_val(&bar2);

    let bar3 = async move { bar2.await };
    let bar3_size = std::mem::size_of_val(&bar3);

    dbg!(foo_size, bar0_size, bar1_size, bar2_size, bar3_size);

    // These are the sizes I would expect assuming 1-byte state tag:
    assert_eq!(foo_size,  1); // ok
    assert_eq!(bar0_size, 2); // fails with `3 != 2`
    assert_eq!(bar1_size, 3); // fails with `7 != 3`
    assert_eq!(bar2_size, 4); // fails with `15 != 4`
    assert_eq!(bar3_size, 5); // fails with `31 != 5`
}

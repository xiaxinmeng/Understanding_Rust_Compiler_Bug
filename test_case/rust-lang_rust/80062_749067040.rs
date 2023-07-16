rust
fn sof<T>() -> T { todo!() }
fn test<T>() {
    let _: [u8; sof::<T>()];
}

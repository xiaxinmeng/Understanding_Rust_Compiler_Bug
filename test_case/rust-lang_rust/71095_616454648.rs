rust
pub fn no_box() {
    let boxed_slice = Box::new([0; 33]) as Box<[i32]>;
    let boxed_array = <Box<[i32; 33]>>::try_from(boxed_slice);
    //~^ ERROR the trait bound `std::boxed::Box<[i32; 33]>: std::convert::From<std::boxed::Box<[i32]>>` is not satisfied
    //~^^ ERROR the trait bound `std::boxed::Box<[i32; 33]>: std::convert::TryFrom<std::boxed::Box<[i32]>>` is not satisfied
    // XXX below is new
    let boxed_array = <Box<[i32; 33]>>::from([0; 33]);
    //~^ ERROR the trait bound `std::boxed::Box<[i32; 33]>: std::convert::From<[i32; 33]>` is not satisfied
}

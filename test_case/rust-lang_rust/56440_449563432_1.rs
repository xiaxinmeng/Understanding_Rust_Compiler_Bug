rust
let mut data = Foo { empty: () };

// initialize first element
let first_elem_1 = &mut data.value as *mut _ as *mut T;
ptr::write(first_elem_1, T::default());

// read first element
let first_elem_2 = &data.value as *const _ as *const T;
println!("{}", *first_elem_2);

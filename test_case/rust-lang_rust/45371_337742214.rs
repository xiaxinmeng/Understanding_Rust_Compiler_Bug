rust

// rest of the trait impls are needed from the previous code.

#[no_mangle]
pub fn foo(region: OwnedRegion<usize>, my_array: &mut [usize; 128]) {
    assert!(region.len() > 128 );

    unsafe {  assume(region.len() > 128) };
    for i in 0..128 {
       my_array[i] = region[i];
    } 
}

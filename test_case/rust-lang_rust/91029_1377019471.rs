rust
enum MyEnum {
    V1 {
        discr: Box<u32>,
        other_value: Option<Box<u32>>,
    },
    V2,
}
fn main() {
    let mut val = MyEnum::V1 {
        discr: Box::new(0),
        other_value: Some(Box::new(1))
    };
    let raw_ptr_to_discr: *mut usize = match val {
        MyEnum::V2 => panic!(),
        MyEnum::V1 { ref mut discr, .. } => discr as *mut _ as *mut _
    };
    match val {
        MyEnum::V1 { other_value, .. } => drop(other_value),
        _ => unreachable!(),
    };
    // `change the variant`
    unsafe { *raw_ptr_to_discr = 0; }
    // end of scope
}

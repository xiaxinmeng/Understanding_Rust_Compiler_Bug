
#[repr(C)] #[repr(packed(2))] 
pub struct Testme {
    field: u32,
    field2: u8,
    field3: u64,
}

fn main() {
    use std::mem::{align_of, size_of};
    println!("A: {}, S: {}", align_of::<Testme>(), size_of::<Testme>());
}


#[repr(align(64))]
pub struct Foo([i32; 16]);

pub fn test(x: &mut Foo) {
    x.0 = [17; 16];
}

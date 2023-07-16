
fn return_borrowed<'a>(x: Box<(&'a mut u32, &mut u32)>) -> &'a mut u32 {
    &mut *x.0
}

fn main() {
}

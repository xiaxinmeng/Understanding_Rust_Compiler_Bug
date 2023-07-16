rust
struct WrapperWithDrop<'a>(&'a mut bool);
impl<'a> Drop for WrapperWithDrop<'a> {
    fn drop(&mut self) {}
}

fn main() {
    let mut base = true;
    let mut wrapper = WrapperWithDrop(&mut base);
    loop {
        drop(wrapper);
        
        base = false;
        wrapper = WrapperWithDrop(&mut base);
    }
}


#![feature(const_vec_new)]

pub struct PackBytes;

static mut HANDLERS: Vec<Box<dyn for<'a> FnMut(&'a mut PackBytes) -> ()>> = Vec::new();

fn main() {
    
    let mut pack = PackBytes;
    unsafe { (HANDLERS[0].as_mut())(&mut pack); }
}

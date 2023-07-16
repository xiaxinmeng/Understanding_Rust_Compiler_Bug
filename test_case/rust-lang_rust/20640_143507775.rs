
struct Timer<'a> {
    x: &'a i32,
}

impl<'b> Timer<'b> {
    pub fn new<'a>(delay: usize, callback: Box<(FnMut() -> usize) + 'a>, remove_on_drop: bool) -> Timer<'a> {
        unimplemented!()
    }
}

fn main() {}

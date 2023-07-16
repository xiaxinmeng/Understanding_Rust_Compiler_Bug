rust
struct Wayland {
   // ...
   draw: Option<NonNull<dyn crate::Draw>>,
}

impl Wayland {
    fn connect(&mut self, draw: &mut Box<dyn crate::Draw>) {
        self.draw = NonNull::new(Box::into_raw(unsafe {
            std::mem::transmute_copy(draw)
        }));
        // ...
    }
}

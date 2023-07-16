rust
pub struct Event1 {
    a: *mut u32,
}

impl Event1 {
    pub fn a<'a>(&'a self) -> &'a u32 {
        unsafe { std::mem::transmute::<_, &'a _>(self.a) }
    }

    pub fn a_mut<'a>(&'a mut self) -> &'a mut u32 {
        unsafe { std::mem::transmute::<_, &'a mut _>(self.a) }
    }
}

rust
#![crate_type = "lib"]

use std::rc::Rc;

pub struct A {
    s: Rc<String>,
}

extern {
    fn escape(s: *const u8, n: usize);
}

impl A {
    pub fn f(&mut self, s: Rc<String>) {
        // The address of allocation escapes. Function might read the 
        // content of the string, but that doesn't mean that it reads the `A::s` field.
        unsafe { escape(s.as_ptr(), s.len()) };
        self.s = s;
    }
}

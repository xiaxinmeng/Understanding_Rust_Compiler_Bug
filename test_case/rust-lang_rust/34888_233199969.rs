 rust
use std::ptr::drop_in_place;

struct SomeStruct {
    pub destructor_ran: bool,
};

impl Drop for SomeStruct {
    fn drop(&mut self) {
        self.destructor_ran = true;
    }
}

{
    let mut some_struct = SomeStruct {
        destructor_ran: false,
    };
    assert!(!some_struct.destructor_ran);
    unsafe {
        drop_in_place(&mut some_struct);
    }
    assert!(some_struct.destructor_ran);
}

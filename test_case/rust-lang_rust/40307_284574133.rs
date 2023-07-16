rust
// will swap x and y provided the memory addresses are distinct
// in rust they always will be!
fn swap(x: &mut i32, y: &mut i32) {
    assert!(*x != 0 && *y != 0, "invalid inputs");
    *x ^= *y; *y ^= *x; *x ^= *y;
    assert!(*x != 0 && *y != 0, "can't happen");
}

struct Struct<'a> {
    reference: Option<&'a mut i32>,
    data: i32,
}

impl<'a> Struct<'a> {
    fn get(&'a mut self) -> Option<&'a i32> {
        match self.reference {
            Some(ref s) => Some(s),
            None => {
                self.reference = Some(&mut self.data);
                None
            }
        }
    }
    
    fn do_something(&mut self) {
        let x = &mut self.data;
        if let Some(ref mut y) = self.reference {
            swap(x, y);
        }
    }
    
    fn violate(&'a mut self) -> &'a i32 {
        // needed to overcome the borrow checker
        let self_ = unsafe { &mut *(self as *mut Struct<'a>) };
        match self.get() {
            Some(s) => s,
            None => {
                // we assume everything is OK to take another borrow
                // of self here, since None was returned
                self_.do_something();
                &self_.data
            }
        }
    }
}

fn main() {
    let mut s = Struct {
        reference: None,
        data: 42,
    };
    s.violate(); // uh-oh 
}

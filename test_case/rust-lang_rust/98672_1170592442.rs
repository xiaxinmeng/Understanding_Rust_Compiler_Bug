Rust
// run-pass
#![feature(let_else)]
#![allow(irrefutable_let_patterns)]

use std::cell::RefCell;

thread_local! {
    pub static LAST_DROPPED: RefCell<u32> = RefCell::new(0);
}

fn reset_last_dropped() {
    LAST_DROPPED.with(|d| {
        *d.borrow_mut() = 0;
    });
}

fn get_last_dropped() -> u32 {
    LAST_DROPPED.with(|d| *d.borrow())
}

struct Droppy(u32);

impl Drop for Droppy {
    fn drop(&mut self) {
        LAST_DROPPED.with(|d| {
            *d.borrow_mut() = self.0;
        });
    }
}

fn test_simple() {
    {
        reset_last_dropped();
        const LINE: u32 = line!();
        let _x = Droppy(LINE).0;
        assert_eq!(get_last_dropped(), LINE);
    }
    {
        reset_last_dropped();
        const LINE: u32 = line!();
        let _x = Droppy(LINE).0 else { return };
        assert_eq!(get_last_dropped(), LINE);
    }
}

fn test_borrow() {
    {
        reset_last_dropped();
        const LINE: u32 = line!();
        let _x = &Droppy(LINE).0;
        assert_eq!(get_last_dropped(), LINE);
    }
    {
        reset_last_dropped();
        const LINE: u32 = line!();
        let _x = &Droppy(LINE).0 else { return };
        assert_eq!(get_last_dropped(), LINE);
    }
}

fn main() {
    test_simple();
    test_borrow();
}

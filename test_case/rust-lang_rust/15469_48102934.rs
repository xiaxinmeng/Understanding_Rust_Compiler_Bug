 rust
extern crate libc;

use libc::{c_void, c_int};

struct LivelyPenData { pen: uint, cb: extern "C" fn() }
struct LivelyPenEvent { id: c_int, data: Box<LivelyPenData> }

extern "C" fn pen_lively_callback() {}
extern "C" fn cb() {}

mod c {
    extern {
        pub fn tickit_pen_bind_event(x: uint, ev: (), fun: Option<extern "C" fn()>, ptr: *mut ::libc::c_void) -> ::libc::c_int;
    }
}

fn main() {
    let ev = ();

        unsafe
        {
            let fun = Some(pen_lively_callback);
            let data = box LivelyPenData{pen: 0, cb: cb};
            let mut id: c_int = 0;
            {
                let raw_data_ptr: *const *const LivelyPenData = std::mem::transmute(&data);
                let raw_data = *raw_data_ptr;
                let raw_data: *mut c_void = std::mem::transmute(raw_data);
                id = c::tickit_pen_bind_event(0, ev, fun, raw_data);
            };
            LivelyPenEvent{id: id, data: data}
        };

}

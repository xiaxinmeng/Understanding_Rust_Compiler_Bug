rust
type ReadFn = extern fn(*mut u8, usize) -> isize;

#[repr(C)]
#[derive(Clone)]
pub struct mp4parse_io {
    pub read: ReadFn,
}

unsafe extern fn mp4parse_new(io: *const mp4parse_io) -> *mut mp4parse_io {
    if ((*io).read as *mut std::os::raw::c_void).is_null() {
        return std::ptr::null_mut();
    }
    let parser = Box::new( (*io).clone() );

    Box::into_raw(parser)
}

fn boom() {
    let parser = unsafe {
        let io = mp4parse_io {
            read: std::mem::transmute::<*const (), ReadFn>(std::ptr::null()),
        };
        mp4parse_new(&io)
    };
    assert!(parser.is_null());
}

fn main() {
    println!("Testing null read callback... ");
    boom();
    println!("Ok!");
}

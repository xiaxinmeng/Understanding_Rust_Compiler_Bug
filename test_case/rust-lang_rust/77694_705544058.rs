Rust
const unsafe fn const_deref() {
    use core::convert::Infallible;
    const INFALLIBLE: *const Infallible = [].as_ptr();
    
    match *INFALLIBLE {
        n => {}, // <--- rightfully claims I need #![feature(const_raw_ptr_deref)]
        //_ => {}, // <--- compiles fine
    };
}

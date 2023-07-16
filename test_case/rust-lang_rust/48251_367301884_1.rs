
#[link(name = "a", kind = "static")]
extern "C" {
    fn do_catch(c: unsafe extern "C" fn());
    fn do_throw() -> !;
}

fn main() {
    unsafe fn inner() {
        do_throw()
    }

    unsafe extern "C" fn callback() {
        inner()
    }

    unsafe { do_catch(callback); }
}

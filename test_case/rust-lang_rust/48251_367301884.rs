
#[link(name = "a", kind = "static")]
extern "C" {
    fn do_catch(c: unsafe extern "C" fn());
    fn do_throw() -> !;
}

fn main() {
    unsafe extern "C" fn callback() {
        do_throw()
    }

    unsafe { do_catch(callback); }
}

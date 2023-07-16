
use std::os::raw::c_int;

#[link(name = "Project1", kind="static")]
extern {
    pub fn lib_fun(i: c_int) -> c_int;
}

struct Foo {}

impl Drop for Foo {
    fn drop(&mut self) {
        unsafe {
            lib_fun(23);
        }
    }
}

thread_local! {
    static FOO: Foo = Foo {};
}

fn main() {
    FOO.with(|_| {
        unsafe { lib_fun(77); }
    });
}


fn main() {
    let a = @"toast";
    {|finally| try(finally) {||
        log(error, "test");
    } } {||
        log(error, a);
    }
}

fn try(finally: fn(), try: fn()) {
    resource r(finally: fn@()) {
        finally();
    }
    unsafe {
        let finally: *u8 = unsafe::reinterpret_cast(ptr::addr_of(finally));
        let wrapper = fn@() {
            (*unsafe::reinterpret_cast::<*u8, *fn()>(finally))();
        };
        let _r = r(wrapper);
        try();
    }
}

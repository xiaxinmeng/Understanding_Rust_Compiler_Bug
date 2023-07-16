rust
extern "C" {
    static X: i32;
}

static mut FOO: *const &'static i32 = [unsafe { &X }].as_ptr();

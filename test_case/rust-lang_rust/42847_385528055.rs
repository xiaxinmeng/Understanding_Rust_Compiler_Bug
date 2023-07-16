rust
extern {
    type foo_t;
}

fn fake_foo_new() -> *const foo_t {
    std::ptr::null()
}

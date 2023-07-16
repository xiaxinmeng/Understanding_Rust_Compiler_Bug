rust
#[link_name = "check_static_recursion_foreign_helper"]
extern "C" {
    #[allow(dead_code)]
    static test_static: c_int;
}

static B: &'static c_int = unsafe { &test_static };

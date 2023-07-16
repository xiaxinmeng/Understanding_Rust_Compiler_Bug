
#[inline(never)]
//#[no_mangle]
fn test_fn() {
    println!("test_fn");
}

//#[no_mangle]
extern "C" fn run_test_fn() {
    let _ = std::panic::catch_unwind(|| test_fn());
}

#[used]
#[link_section = ".init_array"]
static S_TEST_FN: unsafe extern "C" fn() = run_test_fn;

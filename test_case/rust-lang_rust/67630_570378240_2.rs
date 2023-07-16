rust
extern "C" {
    static X: i32;
}

static mut FOO: *const &'static i32 = {
    let _tmp1: &'static i32 = unsafe { &X };
	let _tmp2: [&'static i32; 1] = [_tmp1];
    let _tmp3: &[&'static i32; 1] = &_tmp2;
    let _tmp4: *const &'static i32 = _tmp3.as_ptr();
    _return_location = _tmp4;
    StorageDead(_tmp4);
    StorageDead(_tmp3);
    // by not emitting a StorageDead here, we leak the value
    // to the const evaluator, which will intern all memory
    // reachable from the `_return_location` value and then drop
    // all leftover memory.
    StorageDead(_tmp1);
};

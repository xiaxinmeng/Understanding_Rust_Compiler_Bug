rust
extern "C" {
    static X: i32;
}

static mut FOO: *const &'static i32 = {
    const CONST_PTR_TO_X: *const i32 = &X;
    let _tmp0: *const i32 = CONST_PTR_TO_X;
    let _tmp1: &'static i32 = unsafe { &*X };
	let _tmp2: [&'static i32; 1] = [_tmp1];
    let _tmp3: &[&'static i32; 1] = &_tmp2;
    let _tmp4: *const &'static i32 = _tmp3.as_ptr();
    _return_location = _tmp4;
    StorageDead(_tmp4);
    StorageDead(_tmp3);
    StorageDead(_tmp2); // this is where the pointer becomes dangling
    StorageDead(_tmp1);
};


    let mut result = SmartPtr(std::ptr::null_mut());
    let x : &mut &mut _ = &mut &mut *result;
    let x : *mut &mut _ = x;
    let x: *mut *mut _ = x;
    make_foo(x); 

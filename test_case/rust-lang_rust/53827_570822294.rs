
let a = unsafe {
        let mut a = Box::<[[i32; 2048]; 2048]>::new_uninit();
        ptr::write_bytes(a.as_mut_ptr(), 0, 1);
        a.assume_init()
};

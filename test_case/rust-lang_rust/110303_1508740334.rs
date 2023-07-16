rust
#![feature(const_pointer_byte_offsets)]
#![feature(const_ptr_read)]
#![feature(pointer_byte_offsets)]

const _: i32 = {
    let a = [1, 2];
    unsafe { a.as_ptr().byte_add(1).read() }
};

const _: i32 = {
    unsafe { core::ptr::null::<i32>().read() }
};

rust
unsafe {
    *raw_ptr_to_discr = Box::new(0u32).leak() as *const _ as usize;
}

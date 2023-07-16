`
vec.reserve(1);
unsafe {
    let i = vec.len();
    let p = vec.as_mut_ptr();
    // Right now `vec[i]` is uninitialized and should never be observed by user code
    vec.set_len(vec.len() + 1);
    /*
     * Keeping `vec[i]` uninitlaized is safe as long as untrusted user code
     * can never read the uninitialized memory because it's never called.
     * Since we never directly invoke untrusted code,
     * our only risk is from panicking causing user destructors to run.  
     * Because `ptr::copy_nonoverlapping` and `ptr.offset` never panic we
     * should be safe and users will never see our uninitialized memory.
     */
    ptr::copy_nonoverlapping(existingPointer, p.offset(i), 1));
}

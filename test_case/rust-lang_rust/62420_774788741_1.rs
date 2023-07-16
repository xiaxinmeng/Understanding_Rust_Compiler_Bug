rust
extern "C" {
    pub fn kernel_end();
}

const unsafe fn get_ext_symb_add(f: unsafe extern "C" fn()) -> *const usize {
    f as *const usize
 }
 
 pub const fn get_kernel_end() -> *const usize {
    unsafe { get_ext_symb_add(kernel_end) }
}
 
 pub static KERNEL_HEAP: Locked<KernelHeap> = Locked::new(KernelHeap::new(get_kernel_end()));

rs
#[repr(align(32))]
#[derive(Debug)]
struct AlignedSlice([u8]);

impl AlignedSlice {
    pub unsafe fn new_uninit(size: usize) -> Box<MaybeUninit<AlignedSlice>> {
        unsafe {
            let layout = Layout::from_size_align_unchecked(size, 32);
            let ptr = std::alloc::System.alloc(layout);
            Box::from_raw(std::slice::from_raw_parts_mut(ptr, size) as *mut [MaybeUninit<u8>] as *mut MaybeUninit<AlignedSlice>)
        }
    }

    // TODO use MaybeUninit::assume_init when stable for this
    pub unsafe fn assume_init(this: Box<MaybeUninit<AlignedSlice>>) -> Box<AlignedSlice> {
        unsafe {
            Box::from_raw(this.into_raw() as *mut AlignedSlice)
        }
    }

    pub fn new(data: &[u8]) -> Box<AlignedSlice> {
        let mut this = Self::new_uninit(data.len());
        std::ptr::copy_nonoverlapping(data.as_ptr(), this.as_ptr_mut(), data.len());
        Self::assume_init(this)
    }
}

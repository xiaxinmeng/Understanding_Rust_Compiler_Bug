 rust
struct BoxBuild<T> {
    progress: Option<std::raw::Slice<T>>,
    total: usize,
}

impl<T> Drop for BoxBuild<T> {
    fn drop(&mut self) {
        use std::mem::{size_of, min_align_of};
        unsafe {
            if let Some(slice) = self.progress.take() {
                for n in 0..slice.len {
                    std::ptr::read(slice.data.offset(n as isize));
                }
                std::rt::heap::deallocate(
                    slice.data as *mut u8,
                    size_of::<T>() * self.total,
                    min_align_of::<T>());
            }
        }
    }
}

fn box_clone<T: Clone>(old: &Box<[T]>) -> Box<[T]> {
    use std::mem::{size_of, min_align_of, transmute};
    unsafe {
        // Ignore OOM. #yolo
        let mut alloc = (std::rt::heap::allocate(
                    size_of::<T>() * old.len(),
                    min_align_of::<T>())) as *mut T;
        let mut new = BoxBuild{
            progress: Some(std::raw::Slice {
                data: alloc as *const T,
                len: 0,
            }),
            total: old.len()
        };
        for o in old.iter() {
            std::ptr::write(alloc, o.clone());
            alloc = alloc.offset(1);
            new.progress.as_mut().map(|s|s.len += 1);
        }
        let result = new.progress.take().unwrap();
        transmute(result)
    }
}

rust
pub trait WrappingOffsetExt {
    fn wrapping_offset_ext(self, offset: isize) -> Self;
}

impl<T> WrappingOffsetExt for *const T {
    fn wrapping_offset_ext(self, offset: isize) -> Self {
       ((self as isize).wrapping_add(offset * (::std::mem::size_of::<T>() as isize)) as *const T)
    }
}

#[no_mangle]
pub fn via_wrapping_offset(v: &[u32]) -> u32 {
    let mut p = v.as_ptr();
    let end = v.as_ptr().wrapping_offset(v.len() as isize);
    let mut sum = 0;
    while p != end {
        sum += unsafe { *p };
        p = p.wrapping_offset(1);
    }
    sum
}

#[no_mangle]
pub fn via_integer(v: &[u32]) -> u32 {
    let mut p = v.as_ptr();
    let end = v.as_ptr().wrapping_offset_ext(v.len() as isize);
    let mut sum = 0;
    while p != end {
        sum += unsafe { *p };
        p = p.wrapping_offset_ext(1);
    }
    sum
}

fn main() {}

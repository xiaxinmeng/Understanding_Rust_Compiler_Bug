rust
unsafe fn tailValueIs<T: Copy + Eq, U>(p: *const U, v: T) -> bool {
    let count = mem::size_of::<U>() / mem::size_of::<T>();
    let p = p as *const _;
    count & 1 == 0 || v == *p.offset((count - 1) as isize)
}

impl<T: Copy> SpecFromElem for T {
    default fn from_elem(elem: Self, n: usize) -> Vec<Self> {
        let size = mem::size_of::<T>();
        let p = &elem as *const T;

        let isZero = unsafe {
            let zeroHead = {
                let p = p as *const u64;
                let count = size / mem::size_of::<u64>();
                std::slice::from_raw_parts(p, count).iter().fold(0, |state, x| state | x) == 0
            };

            zeroHead & tailValueIs(p, 0u32) & tailValueIs(p, 0u16) & tailValueIs(p, 0u8)
        };

        if isZero {
            return Vec {
                buf: RawVec:: with_capacity_zeroed(n),
                len: n,
            };
        }

        let mut v = Vec::with_capacity(n);
        if size == 1 {
            unsafe {
                let p = p as *const _;
                ptr::write_bytes(v.as_mut_ptr(), *p, n);
                v.set_len(n);
            }
        } else {
            v.extend_with_element(n, elem);
        }
        v
    }
}

rust
#![allow(incomplete_features)]
#![feature(
    box_syntax,
    const_evaluatable_checked,
    const_generics,
    const_intrinsic_copy,
    const_maybe_uninit_as_ptr,
    const_maybe_uninit_assume_init,
    const_mut_refs,
    const_panic,
    const_precise_live_drops,
    const_ptr_offset,
    const_raw_ptr_deref,
    // This feature should have a more general name considering what it actually does IMO.
    const_refs_to_cell,
    const_slice_from_raw_parts,
    const_trait_impl,
    slice_ptr_get,
)]

use core::mem::{self, MaybeUninit};
use core::ptr;

#[inline(always)]
const unsafe fn ptr_slice_to_array_mut<'a, T, const N: usize>(slice: *mut [T]) -> &'a mut [T; N] {
    &mut *(slice.as_mut_ptr() as *mut [T; N])
}

#[inline(always)]
const unsafe fn ptr_slice_to_array_ref<'a, T, const N: usize>(slice: *const [T]) -> &'a [T; N] {
    &*(slice.as_ptr() as *const [T; N])
}

// This is just a stand-in for `[T; N]`, to simulate how `N` *must* be initially declared somewhere
// entirely outside of `split_array` for it to work as it does below.
pub trait ArrayHelper<T, const N: usize> {
    fn split_array<const M: usize>(self) -> ([T; M], [T; N - M]);
    fn split_array_ref<const M: usize>(&self) -> (&[T; M], &[T; N - M]);
    fn split_array_mut<const M: usize>(&mut self) -> (&mut [T; M], &mut [T; N - M]);
}

impl<T, const N: usize> const ArrayHelper<T, N> for [T; N] {
    // It seems that `const_evaluatable_checked` itself makes these "just work" without even needing
    // a `where` clause.
    #[inline]
    fn split_array<const M: usize>(self) -> ([T; M], [T; N - M]) {
        assert!(M <= N, "Bounds check failure in `[T; N]::split_array'!");
        let mut left = MaybeUninit::<[T; M]>::uninit();
        let mut right = MaybeUninit::<[T; N - M]>::uninit();
        let self_ptr = self.as_ptr();
        let left_ptr = left.as_mut_ptr() as *mut T;
        let right_ptr = right.as_mut_ptr() as *mut T;
        unsafe {
            self_ptr.copy_to_nonoverlapping(left_ptr, M);
            self_ptr.add(M).copy_to_nonoverlapping(right_ptr, N - M);
            mem::forget(self);
            (left.assume_init(), right.assume_init())
        }
    }

    #[inline]
    fn split_array_ref<const M: usize>(&self) -> (&[T; M], &[T; N - M]) {
        assert!(M <= N, "Bounds check failure in `[T; N]::split_array_ref'!");
        let self_ptr = self.as_ptr();
        unsafe {
            // `ptr::slice_from_raw_parts` is a `const fn`. `slice::from_raw_parts` is not.
            let left = ptr::slice_from_raw_parts(self_ptr, M);
            let right = ptr::slice_from_raw_parts(self_ptr.add(M), N - M);
            (ptr_slice_to_array_ref(left), ptr_slice_to_array_ref(right))
        }
    }

    #[inline]
    fn split_array_mut<const M: usize>(&mut self) -> (&mut [T; M], &mut [T; N - M]) {
        assert!(M <= N, "Bounds check failure in `[T; N]::split_array_mut'!");
        let self_ptr = self.as_mut_ptr();
        unsafe {
            // `ptr::slice_from_raw_parts_mut` is a `const fn`. `slice::from_raw_parts_mut` is not.
            let left = ptr::slice_from_raw_parts_mut(self_ptr, M);
            let right = ptr::slice_from_raw_parts_mut(self_ptr.add(M), N - M);
            (ptr_slice_to_array_mut(left), ptr_slice_to_array_mut(right))
        }
    }
}

const X1: ([usize; 2], [usize; 4]) = [1, 2, 3, 4, 5, 6].split_array::<2>();
const Y1: ([usize; 0], [usize; 6]) = [1, 2, 3, 4, 5, 6].split_array::<0>();
const Z1: ([usize; 6], [usize; 0]) = [1, 2, 3, 4, 5, 6].split_array::<6>();
const X2: (&[usize; 2], &[usize; 4]) = [1, 2, 3, 4, 5, 6].split_array_ref::<2>();
const Y2: (&[usize; 0], &[usize; 6]) = [1, 2, 3, 4, 5, 6].split_array_ref::<0>();
const Z2: (&[usize; 6], &[usize; 0]) = [1, 2, 3, 4, 5, 6].split_array_ref::<6>();

fn main() {
    // Boxing everything ensures that Miri will catch anything being done incorrectly.
    let a = [box 1, box 2, box 3, box 4, box 5, box 6].split_array::<2>();
    println!("{:?}", a.0);
    println!("{:?}\n", a.1);
    let b = [box 1, box 2, box 3, box 4, box 5, box 6].split_array::<0>();
    println!("{:?}", b.0);
    println!("{:?}\n", b.1);
    let c = [box 1, box 2, box 3, box 4, box 5, box 6].split_array::<6>();
    println!("{:?}", c.0);
    println!("{:?}\n", c.1);
    let d = [box 1, box 2, box 3, box 4, box 5, box 6];
    let e = d.split_array_ref::<2>();
    println!("{:?}", e.0);
    println!("{:?}\n", e.1);
    let f = d.split_array_ref::<0>();
    println!("{:?}", f.0);
    println!("{:?}\n", f.1);
    let g = d.split_array_ref::<6>();
    println!("{:?}", g.0);
    println!("{:?}\n", g.1);
    let mut h = [box 1, box 2, box 3, box 4, box 5, box 6];
    let i = h.split_array_mut::<2>();
    for (l, r) in i.0.iter_mut().zip(i.1.iter_mut()) {
        *l.as_mut() += *r.as_ref();
        *r.as_mut() += *l.as_ref();
    }
    println!("{:?}", i.0);
    println!("{:?}\n", i.1);
    let j = h.split_array_mut::<0>();
    for (l, r) in j.0.iter_mut().zip(j.1.iter_mut()) {
        *l.as_mut() += *r.as_ref();
        *r.as_mut() += *l.as_ref();
    }
    println!("{:?}", j.0);
    println!("{:?}\n", j.1);
    let k = h.split_array_mut::<6>();
    for (l, r) in k.0.iter_mut().zip(k.1.iter_mut()) {
        *l.as_mut() += *r.as_ref();
        *r.as_mut() += *l.as_ref();
    }
    println!("{:?}", k.0);
    println!("{:?}\n", k.1);
    // Make sure constant usage works properly also
    println!("{:?}", X1.0);
    println!("{:?}\n", X1.1);
    println!("{:?}", Y1.0);
    println!("{:?}\n", Y1.1);
    println!("{:?}", Z1.0);
    println!("{:?}\n", Z1.1);
    println!("{:?}", X2.0);
    println!("{:?}\n", X2.1);
    println!("{:?}", Y2.0);
    println!("{:?}\n", Y2.1);
    println!("{:?}", Z2.0);
    println!("{:?}\n", Z2.1);
}

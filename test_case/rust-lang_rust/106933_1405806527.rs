rs
// the indices must all be in bounds and must not overlap.
// swaps elements around so the median of the 5 elements ends up in `v[c]`
unsafe fn median_of_five<T, F: FnMut(&T, &T) -> bool>(
    v: &mut [T],
    is_less: &mut F,
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    e: usize,
) {
    let [a, b, c, d, e] = unsafe { v.get_many_unchecked_mut([a, b, c, d, e]) };
    let sort = |a: &mut T, b: &mut T, is_less: &mut F| {
        if is_less(b, a) {
            mem::swap(a, b);
        }
    };

    sort(a, c, is_less);
    sort(b, d, is_less);

    if is_less(c, d) {
        mem::swap(c, d);
        mem::swap(a, b);
    }

    sort(b, e, is_less);

    if is_less(c, e) {
        mem::swap(c, e);
        sort(a, c, is_less);
    } else {
        sort(b, c, is_less);
    }
}

pub fn select_linear<T, F: FnMut(&T, &T) -> bool>(mut v: &mut [T], is_less: &mut F, mut k: usize) {
    fn select_pivot<T, F: FnMut(&T, &T) -> bool>(v: &mut [T], is_less: &mut F) -> usize {
        debug_assert!(v.len() >= 5);

        let mut j = 0;
        let mut i = 0;
        while i + 4 < v.len() {
            unsafe { median_of_five(v, is_less, i, i + 1, i + 2, i + 3, i + 4) };
            unsafe { v.swap_unchecked(i + 2, j) };
            i += 5;
            j += 1;
        }

        select_linear(unsafe { v.get_unchecked_mut(..j) }, is_less, j / 2);
        partition(v, j / 2, is_less).0
    }

    loop {
        if v.len() <= 10 {
            insertion_sort(v, is_less);
            return;
        }

        let p = select_pivot(v, is_less);

        if p == k {
            return;
        } else if p > k {
            v = unsafe { v.get_unchecked_mut(..p) };
        } else {
            k -= p + 1;
            v = unsafe { v.get_unchecked_mut(p + 1..) };
        }
    }
}

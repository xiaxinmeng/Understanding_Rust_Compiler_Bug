 rust
fn merge_sort_safe<T>(v: &mut [T], less_eq: |&T, &T| -> bool) {
    if v.is_empty() { return }
    let len = v.len();
    let mut idx: ~[uint] = vec::with_capacity(len);

    for start in iter::range_step(0, len, INSERTION) {
        for i in range(start, cmp::min(start + INSERTION, len)) {
            let mut j = i as int;
            while j > start as int && !less_eq(&v[idx[j - 1]], &v[i]) {
                j -= 1;
            }
            idx.insert(j as uint, i);
        }
    }

    let mut width = INSERTION;
    let mut next_idx = vec::with_capacity(len);
    while width < len {
        for start in iter::range_step(0, len, 2 * width) {
            // the end of the first run/start of the second
            let left_end = cmp::min(start + width, len);
            // end of the second
            let right_end = cmp::min(start + 2 * width, len);
            let mut left = start;
            let mut right = left_end;
            for _ in range(start, right_end) {
                let choose_left = left < left_end && (right >= right_end ||
                                                      less_eq(&v[idx[left]], &v[idx[right]]));
                next_idx.push(if choose_left {
                    let l = idx[left];
                    left += 1;
                    l
                } else {
                    let r = idx[right];
                    right += 1;
                    r
                })
            }
        }
        swap(&mut idx, &mut next_idx);
        next_idx.truncate(0);
        width *= 2;
    }

    // trickery to put all the elements in the correct place.
    let mut inverse_idx = next_idx;
    inverse_idx.push_all(idx); // get the appropriate length; we overwrite it later.
    for (i, &j) in idx.iter().enumerate() {
        inverse_idx[j] = i
    }

    for i in range(0, len - 1) {
        let idx_i = idx[i];
        if idx_i == i { continue }
        v.swap(i, idx_i);
        idx.swap(i, inverse_idx[i]);
        inverse_idx.swap(i,idx_i);
    }
}

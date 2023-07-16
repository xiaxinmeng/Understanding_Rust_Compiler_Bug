
extern mod std;

use std::treemap;

struct Iterator<T> {
    vector: &[T],
    index: uint
}

fn iter<T>(v: &v/[T]) -> Iterator/&v<T> {
    Iterator {vector: v, index: uint::max_value}
}

fn next<T>(iter: &mut Iterator/&a<T>) -> bool {
    iter.index += 1; // Relies on rollover
    iter.index < iter.vector.len()
}

fn get<T>(iter: &v/Iterator<T>) -> &v/T {
    &iter.vector[iter.index]
}

fn is_disjoint(t1: &[int], t2: &[int]) -> bool {
    let mut x = iter(t1);
    let mut y = iter(t2);

    if !next(&mut x) { return true; }
    if !next(&mut y) { return true; }

    loop {
        let advance_x = {
            let a = get(&x);
            let b = get(&y);
            if *a < *b {
                true
            } else if *b < *a {
                false
            } else {
                return false;
            }
        };

        if advance_x {
            if !next(&mut x) {
                return true;
            }
        } else if !next(&mut y) {
            return true;
        }
    }
}

fn main() {
    assert is_disjoint([1, 3, 5], [2, 4, 6]);
    assert !is_disjoint([1, 3, 5], [2, 4, 5]);
    assert is_disjoint([], []);
    assert is_disjoint([1, 2], []);
    assert is_disjoint([], [1, 2]);
}

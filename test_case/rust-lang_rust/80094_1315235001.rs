rust
//! Intended use if `Zip` were part of the stdlib.

use std::iter::{self, Zip};

let s1 = iter::repeat(0).take(5);
let s2 = iter::repeat(1).take(5);
let s3 = iter::repeat(2).take(5);

// zips N iterators together.
for [n1, n2, n3] in [s1, s2, s3].zip() {
    assert_eq!([n1, n2, n3], [0, 1, 2]);
}

// it works on tuples too!
for (n1, n2, n3) in (s1, s2, s3).zip() {
    assert_eq!((n1, n2, n3), (0, 1, 2));
}

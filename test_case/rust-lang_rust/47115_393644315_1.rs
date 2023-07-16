rust
let s = &[1, 2, 3, 4, 5];

let mut chunks = s.exact_chunks(2);

// the `remaining` method gives the part of the slice
// that will not be iterated over by the `ExactChunks` iterator.
let rem = chunks.remaining();
assert_eq!(rem, &[5]);

assert_eq!(chunks.next(), Some(&[1, 2]));
assert_eq!(chunks.next(), Some(&[3, 4]));
assert_eq!(chunks.next(), None);

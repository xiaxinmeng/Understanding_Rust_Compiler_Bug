
error: lifetime may not live long enough
  --> src/main.rs:10:28
   |
10 | let lambda = |s: &[i32;5]| s.split_at(3);
   |                  -       - ^^^^^^^^^^^^^ returning this value requires that `'1` must outlive `'2`
   |                  |       |
   |                  |       return type of closure is (&'2 [i32], &[i32])
   |                  let's call the lifetime of this reference `'1`

error[E0308]: mismatched types
  --> src/main.rs:11:20
   |
11 | let (begin, end) = Ref::map_split(borrow, lambda);
   |                    ^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected tuple `(&[i32], &[i32])`
              found tuple `(&[i32], &[i32])`

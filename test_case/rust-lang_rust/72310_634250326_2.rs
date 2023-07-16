rust
error: lifetime may not live long enough
    --> src/libcore/iter/adapters/mod.rs:1639:29
     |
1630 |     pub fn next_if_eq<'a, R>(
     |                       -- lifetime `'a` defined here
...
1639 |         self.next_if(|next| next == expected)
     |                       ----  ^^^^ requires that `'1` must outlive `'a`
     |                       |
     |                       has type `&'1 <I as iter::traits::iterator::Iterator>::Item`

error: aborting due to previous error

text
error[[E0506]](https://doc.rust-lang.org/nightly/error-index.html#E0506): cannot assign to `*xref` because it is borrowed
  --> src/main.rs:10:5
   |
6  |     let xaref = AtomicUsize::from_mut(xref);
   |                                       ---- borrow of `*xref` occurs here
...
10 |     *xref = 53;
   |     ^^^^^^^^^^ assignment to borrowed `*xref` occurs here
11 |     
12 |     dbg!(xaref);
   |          ----- borrow later used here

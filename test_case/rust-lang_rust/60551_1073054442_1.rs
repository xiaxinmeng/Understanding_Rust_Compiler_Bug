
error[[E0599]](https://doc.rust-lang.org/nightly/error-index.html#E0599): no function or associated item named `new` found for struct `Solver<'_, Physics<P>>` in the current scope
  [--> src/main.rs:76:50
](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=f15f265d88a25cfa9fc8822cf3c5950e#)   |
12 | / struct Solver<'a, M>
13 | | where
14 | |     M: MathTrait + Sized,
15 | |     [f64; M::NDIM]: Sized,
...  |
18 | |     problem: &'a mut M,
19 | | }
   | |_- function or associated item `new` not found for this
...
76 |           let mut solver = Solver::<Physics::<P>>::new(&mut phys);
   |                                                    ^^^ function or associated item cannot be called on `Solver<'_, Physics<P>>` due to unsatisfied trait bounds

error: unconstrained generic constant
  [--> src/main.rs:76:26
](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=f15f265d88a25cfa9fc8822cf3c5950e#)   |
76 |         let mut solver = Solver::<Physics::<P>>::new(&mut phys);
   |                          ^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: try adding a `where` bound using this expression: `where [(); M::NDIM]:`
note: required by a bound in `Solver`
  [--> src/main.rs:15:11
](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=f15f265d88a25cfa9fc8822cf3c5950e#)   |
12 | struct Solver<'a, M>
   |        ------ required by a bound in this
...
15 |     [f64; M::NDIM]: Sized,
   |           ^^^^^^^ required by this bound in `Solver`

For more information about this error, try `rustc --explain E0599`.

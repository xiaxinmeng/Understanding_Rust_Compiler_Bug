
error[E0275]: overflow evaluating the requirement `&BTreeSet<_>: Sub<&BTreeSet<_>>`
  --> src/main.rs:28:30
   |
17 | fn test<T, const N: usize>(p: &Vector<T, N>) -> Vector<T, N>
   |    ---- required by a bound in this
18 | where
19 |     for<'a> &'a T: Sub<&'a T, Output = T>,
   |                    ---------------------- required by this bound in `test`
...
28 |     println!("Output: {:?}", test(&a)); // This fails
   |                              ^^^^
   |
   = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`playground`)
note: required because of the requirements on the impl of `Sub<&Vector<BTreeSet<_>, {_: usize}>>` for `&Vector<BTreeSet<_>, {_: usize}>`
  --> src/main.rs:6:33
   |
6  | impl<'a, 'b, T, const N: usize> Sub<&'a Vector<T, N>> for &'b Vector<T, N>
   |                                 ^^^^^^^^^^^^^^^^^^^^^     ^^^^^^^^^^^^^^^^
   = note: 127 redundant requirements hidden
   = note: required because of the requirements on the impl of `for<'a> Sub` for `&'a Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<Vector<BTreeSet<_>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>, {_: usize}>`

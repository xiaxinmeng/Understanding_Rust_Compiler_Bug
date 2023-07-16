

error[E0275]: overflow evaluating the requirement `&HashSet<_, _>: Sub`
  --> src/lib.rs:11:22
   |
3  | fn test<T>(p: T) -> T
   |    ---- required by a bound in this
4  | where
5  |     for<'a> &'a T: std::ops::Sub<&'a T, Output = T>,
   |                    -------------------------------- required by this bound in `test`
...
11 |     println!("{:?}", test(0.0_f64));
   |                      ^^^^
   |
   = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`error_case`)
   = note: required because of the requirements on the impl of `Sub` for `&OrderedFloat<HashSet<_, _>>`
   = note: 127 redundant requirements hidden
   = note: required because of the requirements on the impl of `for<'a> Sub` for `&'a OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<OrderedFloat<HashSet<_, _>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`

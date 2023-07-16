plain
   Compiling cc v1.0.73
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0277]: can't compare `Q` with `Q`
     |
     |
2192 |         cmp::SliceContains::slice_contains(x, self)
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `Q == Q`
     |
note: required for `Q` to implement `SliceContains`
    --> library/core/src/slice/cmp.rs:233:9
     |
233  | impl<T> SliceContains for T
help: consider further restricting type parameter `Q`
     |
     |
2190 |         T: PartialEq<Q>, Q: cmp::PartialEq

error[E0308]: mismatched types
    --> library/core/src/slice/mod.rs:2192:47
     |
     |
114  | impl<T> [T] {
     |      - found type parameter
...
2188 |     pub fn contains<Q>(&self, x: &Q) -> bool
     |                     - expected type parameter
...
2192 |         cmp::SliceContains::slice_contains(x, self)
     |         ----------------------------------    ^^^^ expected type parameter `Q`, found type parameter `T`
     |         arguments to this function are incorrect
     |
     = note: expected reference `&[Q]`
                found reference `&[T]`
                found reference `&[T]`
     = note: a type parameter was expected, but a different one was found; you might be missing a type parameter or trait bound
     = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
    --> library/core/src/slice/cmp.rs:230:8
     |
     |
230  |     fn slice_contains(&self, x: &[Self]) -> bool;


error[E0369]: binary operation `==` cannot be applied to type `&[Q]`
     |
     |
2222 |         self.len() >= n && needle == &self[..n]
     |                            ------ ^^ ---------- &[T]
     |                            &[Q]
     |
help: consider further restricting type parameter `Q`
     |
     |
2219 |         T: PartialEq<Q>, Q: cmp::PartialEq<T>


error[E0369]: binary operation `==` cannot be applied to type `&[Q]`
     |
     |
2252 |         m >= n && needle == &self[m - n..]
     |                   ------ ^^ -------------- &[T]
     |                   &[Q]
     |
help: consider further restricting type parameter `Q`
     |
     |
2249 |         T: PartialEq<Q>, Q: cmp::PartialEq<T>

Some errors have detailed explanations: E0277, E0308, E0369.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `core` due to 4 previous errors

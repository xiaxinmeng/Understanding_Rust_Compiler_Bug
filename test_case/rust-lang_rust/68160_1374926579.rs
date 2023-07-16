
error[E0521]: borrowed data escapes outside of function
  --> src/main.rs:16:5
   |
15 | fn call_b_on_a<'a>(a: &'a mut dyn A) {
   |                --  - `a` is a reference that is only valid in the function body
   |                |
   |                lifetime `'a` defined here
16 |     a.b();
   |     ^^^^^
   |     |
   |     `a` escapes the function body here
   |     argument requires that `'a` must outlive `'static`
   |
note: the used `impl` has a `'static` requirement
  --> src/main.rs:13:16
   |
4  |     fn b(&mut self) {}
   |        - calling this method introduces the `impl`'s 'static` requirement
...
13 | impl B for dyn A {}
   |                ^ this has an implicit `'static` lifetime requirement
   = note: requirement occurs because of a mutable reference to `dyn A`
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
help: consider relaxing the implicit `'static` requirement
   |
13 | impl B for dyn A + '_ {}
   |                  ++++

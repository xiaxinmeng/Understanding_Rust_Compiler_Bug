
error[E0631]: type mismatch in closure arguments
  --> /home/gh-compiler-errors/test.rs:19:5
   |
19 |     needs_a(|x| {});
   |     ^^^^^^^ --- found signature defined here
   |     |
   |     expected due to this
   |
   = note: expected closure signature `fn(i32) -> _`
              found closure signature `fn(u32) -> _`
note: required by a bound in `needs_a`
  --> /home/gh-compiler-errors/test.rs:15:24
   |
15 | fn needs_a(t: impl A + Fn(i32)) {
   |                        ^^^^^^^ required by this bound in `needs_a`

error: aborting due to previous error

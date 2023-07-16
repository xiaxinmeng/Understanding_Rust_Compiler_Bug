rust
error[E0521]: borrowed data escapes outside of closure
  --> src/main.rs:7:9
   |
3  |       ::crossbeam::thread::scope(|scope| {
   |                                   -----
   |                                   |
   |                                   `scope` declared here, outside of the closure body
   |                                   `scope` is a reference that is only valid in the closure body
...
7  | /         scope.spawn(|_| {
8  | |             scope.spawn(|_| { /* … */ }) // <- ADDED!
9  | |             // …
10 | |         });
   | |__________^ `scope` escapes the closure body here
   |
   = note: requirement occurs because of the type Scope<'_>, which makes the generic argument '_ invariant
   = note: the struct Scope<'env> is invariant over the parameter 'env
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

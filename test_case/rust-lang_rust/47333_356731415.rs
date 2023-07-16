
[00:03:08] error[E0502]: cannot borrow `self` as immutable because `*self` is also borrowed as mutable
[00:03:08]     --> libcore/slice/mod.rs:1242:32
[00:03:08]      |
[00:03:08] 1242 |                 self.try_rfold(self.len(), move |i, x| {
[00:03:08]      |                 ----           ^^^^ immutable borrow occurs here
[00:03:08]      |                 |
[00:03:08]      |                 mutable borrow occurs here
[00:03:08] ...
[00:03:08] 1246 |                 }).err()
[00:03:08]      |                  - mutable borrow ends here
[00:03:08] ...
[00:03:08] 1422 | iterator!{struct Iter -> *const T, &'a T, make_ref}
[00:03:08]      | --------------------------------------------------- in this macro invocation

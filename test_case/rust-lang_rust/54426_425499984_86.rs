\nfn bar(xunexpectedly panicked. this is a bug.
[00:48:10] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:48:10] 
[00:48:10] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[00:48:10] 
[00:48:10] 
[00:48:10] note: compiler flags: -Z ui-testing -Z unstable-options -Z borrowck=mir -C prefer-dynamic -C rpath
[00:48:10] 
[00:48:10] ------------------------------------------
[00:48:10] 
[00:48:10] thread '[ui] ui/cannot-mutate-captured-non-mut-var.rs#mir' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3258:9
[00:48:10] thread '[ui] ui/cannot-mutate-captured-non-mut-var.rs#mir' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3258:9
[00:48:10] 
[00:48:10] ---- [ui] ui/coercion/coerce-overloaded-autoderef.rs#mir stdout ----
[00:48:10] diff of stderr:
[00:48:10] 
[00:48:10] - error[E0499]: cannot borrow `*x` as mutable more than once at a time
[00:48:10] + error[E0499]: cannot borrow `x*` as mutable more than once at a time
[00:48:10] 2   --> $DIR/coerce-overloaded-autoderef.rs:22:24
[00:48:10] 3    |
[00:48:10] 4 LL |     let y = borrow_mut(x);
[00:48:10] 
[00:48:10] 9 LL |     drop((y, z));
[00:48:10] 11 
[00:48:10] 11 
[00:48:10] - error[E0506]: cannot assign to `**x` because it is borrowed
[00:48:10] + error[E0506]: cannot assign to `x**` because it is borrowed
[00:48:10] 13   --> $DIR/coerce-overloaded-autoderef.rs:31:5
[00:48:10] 14    |
[00:48:10] 15 LL |     let y = borrow(x);
[00:48:10] 
[00:48:10] -    |                    - borrow of `**x` occurs here
[00:48:10] +    |                    - borrow of `x**` occurs here
[00:48:10] 17 LL |     let z = borrow(x);
[00:48:10] 18 LL |     **x += 1;
[00:48:10] -    |     ^^^^^^^^ assignment to borrowed `**x` occurs here
[00:48:10] +    |     ^^^^^^^^ assignment to borrowed `x**` occurs here
[00:48:10] 20 ...
[00:48:10] 21 LL |     drop((y, z));
[00:48:10] 
[00:48:10] 23 
[00:48:10] 23 
[00:48:10] - error[E0499]: cannot borrow `*x` as mutable more than once at a time
[00:48:10] + error[E0499]: cannot borrow `x*` as mutable more than once at a time
[00:48:10] 25   --> $DIR/coerce-overloaded-autoderef.rs:38:20
[00:48:10] 26    |

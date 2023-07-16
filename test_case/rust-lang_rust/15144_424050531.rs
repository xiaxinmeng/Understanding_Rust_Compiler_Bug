text
> cargo +nightly build
   Compiling foo v0.1.0 (C:\Users\Steve Klabnik\tmp\foo)
error[E0495]: cannot infer an appropriate lifetime for borrow expression due to conflicting requirements
  --> src\lib.rs:25:21
   |
25 |         bar.set_foo(&mut foo);
   |                     ^^^^^^^^
   |
note: first, the lifetime cannot outlive the lifetime  as defined on the body at 24:14...
  --> src\lib.rs:24:14
   |
24 |     with_bar(|bar: &mut Bar| {
   |              ^^^^^^^^^^^^^^^
note: ...so that closure can access `foo`
  --> src\lib.rs:25:21
   |
25 |         bar.set_foo(&mut foo);
   |                     ^^^^^^^^
note: but, the lifetime must be valid for the anonymous lifetime #3 defined on the body at 24:14...
  --> src\lib.rs:24:14
   |
24 |       with_bar(|bar: &mut Bar| {
   |  ______________^
25 | |         bar.set_foo(&mut foo);
26 | |     });
   | |_____^
   = note: ...so that the types are compatible:
           expected &mut Bar<'_>
              found &mut Bar<'_>


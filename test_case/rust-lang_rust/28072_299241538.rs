


rustc 1.17.0 (56124baa9 2017-04-24)
error[E0433]: failed to resolve. Use of undeclared type or module `std`
  --> <anon>:10:17
   |
10 | f! { fn bar(z: &std::io::Read); }
   | ----------------^^^^^^^^^^^^^----
   | |               |
   | |               Use of undeclared type or module `std`
   | in this macro invocation

error: aborting due to previous error

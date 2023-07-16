
warning: unused variable: `filter`
 --> src/lib.rs:5:27
  |
5 | async fn walk<F: 'static>(filter: F)
  |                           ^^^^^^ help: if this is intentional, prefix it with an underscore: `_filter`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: function is never used: `walk`
 --> src/lib.rs:5:10
  |
5 | async fn walk<F: 'static>(filter: F)
  |          ^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: 2 warnings emitted

error: internal compiler error: broken MIR in DefId(0:6 ~ playground[caf5]::walk[0]) (NoSolution): could not prove Binder(OutlivesPredicate(<F as Trait<&'a u32>>::Output, ReLateBound(DebruijnIndex(0), BrNamed(DefId(0:9 ~ playground[caf5]::walk[0]::'a[1]), 'a))))
  --> src/lib.rs:9:1
   |
9  | / {
10 | | }
   | |_^
   |
   = note: delayed at src/librustc_mir/borrow_check/type_check/mod.rs:258:27

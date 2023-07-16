
DEBUG 2019-08-06T11:38:07Z: rustc::ty::print::pretty: try_print_visible_def_path: def_id=DefId(0:0 ~ issue_62517[317d])
error[E0391]: cycle detected when processing `foo::{{opaque}}#0`
  --> src/test/ui/async-await/issues/issue-62517.rs:10:32
   |
10 | async fn foo<'a>(_: &'a ()) -> impl Alpha<dyn Object> {
   |                                ^^^^^^^^^^^^^^^^^^^^^^
   |
note: ...which requires processing `foo::{{opaque}}#0`...
  --> src/test/ui/async-await/issues/issue-62517.rs:10:32
   |
10 | async fn foo<'a>(_: &'a ()) -> impl Alpha<dyn Object> {
   |                                ^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing `foo::{{opaque}}#0`...
  --> src/test/ui/async-await/issues/issue-62517.rs:10:32
   |
10 | async fn foo<'a>(_: &'a ()) -> impl Alpha<dyn Object> {
   |                                ^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which again requires processing `foo::{{opaque}}#0`, completing the cycle
note: cycle used when processing `foo`
  --> src/test/ui/async-await/issues/issue-62517.rs:10:1
   |
10 | async fn foo<'a>(_: &'a ()) -> impl Alpha<dyn Object> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

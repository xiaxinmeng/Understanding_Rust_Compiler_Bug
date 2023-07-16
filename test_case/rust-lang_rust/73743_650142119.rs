
$ rustdoc +stage1 src/test/rustdoc/doc-cfg.rs
error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at /home/joshua/src/rust/src/librustc_session/session.rs:436:9

error: internal compiler error: cat_expr Errd
  --> src/test/rustdoc/doc-cfg.rs:51:37
   |
51 |   pub unsafe fn uses_target_feature() {
   |  _____________________________________^
52 | |     content::should::be::irrelevant();
53 | | }
   | |_^
   |
   = note: delayed at /home/joshua/src/rust/src/librustc_session/session.rs:436:9

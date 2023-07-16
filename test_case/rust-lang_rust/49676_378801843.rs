plain
[00:00:48] configure: rust.quiet-tests     := True
---
[00:07:08] error[E0412]: cannot find type `Item` in this scope
[00:07:08]    --> librustc/middle/liveness.rs:188:39
[00:07:08]     |
[00:07:08] 188 |     fn visit_item(&mut self, i: &'tcx Item) {
[00:07:08]     |                                       ^^^^ not found in this scope
[00:07:08] help: possible candidates are found in other modules, you can import them into scope
---
[00:07:42] Makefile:28: recipe for target 'all' failed
[00:07:42] make: *** [all] Error 1

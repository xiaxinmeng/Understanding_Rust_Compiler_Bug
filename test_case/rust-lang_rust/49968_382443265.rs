plain
[00:00:47] configure: rust.quiet-tests     := True
---
[00:34:35] error: this feature has been stable since 1.27.0. Attribute no longer needed
[00:34:35]   --> librustdoc/lib.rs:26:12
[00:34:35]    |
[00:34:35] 26 | #![feature(dyn_trait)]
---
[00:34:36] Makefile:28: recipe for target 'all' failed
[00:34:36] make: *** [all] Error 1

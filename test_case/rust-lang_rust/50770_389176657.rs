plain
[00:37:38]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:37:43] error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
[00:37:43]     --> librustdoc/clean/mod.rs:2188:13
[00:37:43]      |
[00:37:43] 2188 |             hir::ImplItemKind::Type(ref ty) => TypedefItem(Typedef {
[00:37:43] 
[00:37:44] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:37:44]     --> librustdoc/clean/mod.rs:2674:30
[00:37:44]      |
[00:37:44]      |
[00:37:44] 2674 |                 if let Some(&hir::ItemTy(ref ty, ref generics)) = alias {
[00:37:44] 
[00:37:52] error: aborting due to 3 previous errors
[00:37:52] 
[00:37:52] For more information about this error, try `rustc --explain E0023`.
---
[00:37:52] 
[00:37:52] 
[00:37:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:37:52] Build completed unsuccessfully in 0:32:35
[00:37:52] make: *** [all] Error 1
[00:37:52] Makefile:28: recipe for target 'all' failed
79196 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu
79192 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release
75948 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
71660 ./.git/modules/src/tools

plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/be4ed1c0-fd7e-4b08-914d-c1bb79967565.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71814/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71814/merge:refs/remotes/pull/71814/merge
---
 ---> cb2676f08729
Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
 ---> Using cache
 ---> df25ce111862
Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
 ---> 599b9ac96b27
Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
 ---> Using cache
 ---> 091087e35a36
---
   Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
   Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
.................i.................................................................................. 1800/9966
.................................................................................................... 1900/9966
.................................i.................................................................. 2000/9966
.................................................................................................... 2100/9966
.......................iiiii........................................................................ 2200/9966
.................................................................................................... 2400/9966
.................................................................................................... 2500/9966
.................................................................................................... 2600/9966
.................................................................................................... 2700/9966
---
.......i...............i............................................................................ 5100/9966
.................................................................................................... 5200/9966
.....................................................i.............................................. 5300/9966
............................................i....................................................... 5400/9966
..............................................ii.ii........i...i.................................... 5500/9966
.............................................................................................i...... 5700/9966
.................................................................................................... 5800/9966
............................ii.....................................i................................ 5900/9966
.................................................................................................... 6000/9966
.................................................................................................... 6000/9966
.................................................................................................... 6100/9966
...............................................................ii..i..ii...........i................ 6200/9966
.................................................................................................... 6400/9966
.................................................................................................... 6500/9966
.................................................................................................... 6500/9966
..............................................................................................i..ii. 6600/9966
.................................................................................................... 6800/9966
...............................................................................................i.... 6900/9966
.................................................................................................... 7000/9966
.................................................................................................... 7100/9966
---
.................................................................................................... 7900/9966
.................................................................................................... 8000/9966
.................................................................................................... 8100/9966
.....i.............................................................................................. 8200/9966
......................................................iiiiii.iiiii.i................................ 8300/9966
.......i............................................................................................ 8500/9966
.................................................................................................... 8600/9966
.................................................................................................... 8700/9966
.................................................................................................... 8800/9966
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 186 tests
iiii......i.............ii.i..........i.............................i..i..................i....i.... 100/186
........i.i.i...iii..iiiiiii.iiiiiiiii......................iii...............ii......

 finished in 5.991
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 9 tests
iiiiiiiii

 finished in 0.163
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 115 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
...iiii.....ii.

 finished in 15.833
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 112.599
Set({"src/libcore"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling core v0.0.0 (/checkout/src/libcore)
error[E0277]: `*mut mem::test_discriminant_send_sync::Regular` cannot be sent between threads safely
   --> src/libcore/../libcore/tests/mem.rs:129:5
    |
127 |     fn is_send_sync<T: Send + Sync>() {}
128 | 
128 | 
129 |     is_send_sync::<Discriminant<Regular>>();
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `*mut mem::test_discriminant_send_sync::Regular` cannot be sent between threads safely
    |
    = help: within `std::mem::Discriminant<mem::test_discriminant_send_sync::Regular>`, the trait `std::marker::Send` is not implemented for `*mut mem::test_discriminant_send_sync::Regular`
    = note: required because it appears within the type `std::marker::PhantomData<*mut mem::test_discriminant_send_sync::Regular>`
    = note: required because it appears within the type `std::mem::Discriminant<mem::test_discriminant_send_sync::Regular>`

error[E0277]: `*mut mem::test_discriminant_send_sync::Regular` cannot be shared between threads safely
   --> src/libcore/../libcore/tests/mem.rs:129:5
    |
127 |     fn is_send_sync<T: Send + Sync>() {}
128 | 
128 | 
129 |     is_send_sync::<Discriminant<Regular>>();
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `*mut mem::test_discriminant_send_sync::Regular` cannot be shared between threads safely
    |
    = help: within `std::mem::Discriminant<mem::test_discriminant_send_sync::Regular>`, the trait `std::marker::Sync` is not implemented for `*mut mem::test_discriminant_send_sync::Regular`
    = note: required because it appears within the type `std::marker::PhantomData<*mut mem::test_discriminant_send_sync::Regular>`
    = note: required because it appears within the type `std::mem::Discriminant<mem::test_discriminant_send_sync::Regular>`

error[E0277]: `*mut mem::test_discriminant_send_sync::NotSendSync` cannot be sent between threads safely
   --> src/libcore/../libcore/tests/mem.rs:130:5
    |
127 |     fn is_send_sync<T: Send + Sync>() {}
...
...
130 |     is_send_sync::<Discriminant<NotSendSync>>();
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `*mut mem::test_discriminant_send_sync::NotSendSync` cannot be sent between threads safely
    |
    = help: within `std::mem::Discriminant<mem::test_discriminant_send_sync::NotSendSync>`, the trait `std::marker::Send` is not implemented for `*mut mem::test_discriminant_send_sync::NotSendSync`
    = note: required because it appears within the type `std::marker::PhantomData<*mut mem::test_discriminant_send_sync::NotSendSync>`
    = note: required because it appears within the type `std::mem::Discriminant<mem::test_discriminant_send_sync::NotSendSync>`

error[E0277]: `*mut mem::test_discriminant_send_sync::NotSendSync` cannot be shared between threads safely
   --> src/libcore/../libcore/tests/mem.rs:130:5
    |
127 |     fn is_send_sync<T: Send + Sync>() {}
...
...
130 |     is_send_sync::<Discriminant<NotSendSync>>();
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `*mut mem::test_discriminant_send_sync::NotSendSync` cannot be shared between threads safely
    |
    = help: within `std::mem::Discriminant<mem::test_discriminant_send_sync::NotSendSync>`, the trait `std::marker::Sync` is not implemented for `*mut mem::test_discriminant_send_sync::NotSendSync`
    = note: required because it appears within the type `std::marker::PhantomData<*mut mem::test_discriminant_send_sync::NotSendSync>`
    = note: required because it appears within the type `std::mem::Discriminant<mem::test_discriminant_send_sync::NotSendSync>`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0277`.
error: could not compile `core`.
---
  local time: Sat May  2 23:26:59 UTC 2020
  network time: Sat, 02 May 2020 23:26:59 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71814/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71814/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3642) (python)
##[section]Finishing: Finalize Job

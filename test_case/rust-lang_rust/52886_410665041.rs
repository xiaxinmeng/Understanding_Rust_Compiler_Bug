plain
[00:02:45]       Memory: 8 GB
[00:02:45]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:02:45]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:45]       SMC Version (system): 2.8f0
[00:02:45]       Serial Number (system): VMnQhsrx9K1S
[00:02:45] 
[00:02:45] hw.ncpu: 4
[00:02:45] hw.byteorder: 1234
[00:02:45] hw.memsize: 8589934592
---
[00:54:39] [RUSTC-TIMING] cargo_metadata test:false 10.580
[00:54:40]    Compiling clippy v0.0.212 (file:///Users/travis/build/rust-lang/rust/src/tools/clippy)
[00:54:40]    Compiling clippy_lints v0.0.212 (file:///Users/travis/build/rust-lang/rust/src/tools/clippy/clippy_lints)
[00:54:40]    Compiling url v1.7.1
[00:54:45] error[E0599]: no variant named `GlobalAsm` found for type `rustc::hir::def::Def` in the current scope
[00:54:45]     |
[00:54:45]     |
[00:54:45] 942 |         Def::GlobalAsm(id) => Some(id),
[00:54:45]     |         ^^^^^^^^^^^^^^^^^^ variant not found in `rustc::hir::def::Def`
[00:54:49] error: aborting due to previous error
[00:54:49] 
[00:54:49] For more information about this error, try `rustc --explain E0599`.
[00:54:49] [RUSTC-TIMING] clippy_lints test:false 8.551
[00:54:49] [RUSTC-TIMING] clippy_lints test:false 8.551
[00:54:49] error: Could not compile `clippy_lints`.
[00:54:49] 
[00:54:49] Caused by:
[00:54:49]   process didn't exit successfully: `/Users/travis/build/rust-lang/rust/build/bootstrap/debug/rustc --edition=2018 --crate-name clippy_lints tools/clippy/clippy_lints/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=5a4fa5052c728121 -C extra-filename=-5a4fa5052c728121 --out-dir /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps --target x86_64-apple-darwin -L dependency=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps -L dependency=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/release/deps --extern cargo_metadata=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libcargo_metadata-ddc020498027754b.rlib --extern if_chain=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libif_chain-d177d129174e74d0.rlib --extern itertools=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libitertools-5be0af0e0b808680.rlib --extern lazy_static=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/liblazy_static-b331bc07e0989c12.rlib --extern matches=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libmatches-98a2ea457f2dd030.rlib --extern pulldown_cmark=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libpulldown_cmark-8d7cbbaeb42cc719.rlib --extern quine_mc_cluskey=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libquine_mc_cluskey-3ed807006ac2e562.rlib --extern regex_syntax=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libregex_syntax-42dcc37fdece991f.rlib --extern semver=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libsemver-979d71b6d9eaabae.rlib --extern serde=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libserde-a4ed9556d08e792e.rlib --extern serde_derive=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/release/deps/libserde_derive-a01660846e2dc714.dylib --extern toml=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libtoml-d8c27d975b5fab61.rlib --extern unicode_normalization=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/libunicode_normalization-da8b377af04bed79.rlib --extern url=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/deps/liburl-d4bf32e265b8f5dd.rlib` (exit code: 1)
[00:54:49] warning: build failed, waiting for other jobs to finish...
[00:54:52] error[E0599]: no variant named `GlobalAsm` found for type `rustc::hir::def::Def` in the current scope
[00:54:52]     |
[00:54:52]     |
[00:54:52] 942 |         Def::GlobalAsm(id) => Some(id),
[00:54:52]     |         ^^^^^^^^^^^^^^^^^^ variant not found in `rustc::hir::def::Def`
[00:54:56] error: aborting due to previous error
[00:54:56] 
[00:54:56] For more information about this error, try `rustc --explain E0599`.
[00:54:56] error: Could not compile `clippy_lints`.
---
[00:56:04]    Compiling crossbeam-channel v0.2.3
[00:56:05]    Compiling clippy_lints v0.0.212 (file:///Users/travis/build/rust-lang/rust/src/tools/clippy/clippy_lints)
[00:56:06] [RUSTC-TIMING] crossbeam_channel test:false 1.263
[00:56:06]    Compiling rls-data v0.16.0
[00:56:09] error[E0599]: no variant named `GlobalAsm` found for type `rustc::hir::def::Def` in the current scope
[00:56:09]     |
[00:56:09]     |
[00:56:09] 942 |         Def::GlobalAsm(id) => Some(id),
[00:56:09]     |         ^^^^^^^^^^^^^^^^^^ variant not found in `rustc::hir::def::Def`
[00:56:10] [RUSTC-TIMING] languageserver_types test:false 12.263
[00:56:10]    Compiling rls-vfs v0.4.6
[00:56:10] [RUSTC-TIMING] rls_data test:false 4.263
[00:56:10]    Compiling rayon v1.0.1
---
[01:11:53] travis_fold:start:stage2-clippy-driver
travis_time:start:stage2-clippy-driver
Building stage2 tool clippy-driver (x86_64-apple-darwin)
[01:11:53]    Compiling clippy_lints v0.0.212 (file:///Users/travis/build/rust-lang/rust/src/tools/clippy/clippy_lints)
[01:11:58] error[E0599]: no variant named `GlobalAsm` found for type `rustc::hir::def::Def` in the current scope
[01:11:58]     |
[01:11:58]     |
[01:11:58] 942 |         Def::GlobalAsm(id) => Some(id),
[01:11:58]     |         ^^^^^^^^^^^^^^^^^^ variant not found in `rustc::hir::def::Def`
[01:11:58] 
[01:11:58] error[E0599]: no variant named `GlobalAsm` found for type `rustc::hir::def::Def` in the current scope
[01:11:58]     |
[01:11:58]     |
[01:11:58] 942 |         Def::GlobalAsm(id) => Some(id),
[01:11:58]     |         ^^^^^^^^^^^^^^^^^^ variant not found in `rustc::hir::def::Def`
[01:12:01] error: aborting due to previous error
[01:12:01] 
[01:12:01] For more information about this error, try `rustc --explain E0599`.
[01:12:01] error: aborting due to previous error
---
[01:12:02]    Compiling crates-io v0.16.0
[01:12:06] [RUSTC-TIMING] rustc_data_structures test:false 4.223
[01:12:06]   [RUSTC-TIMING] rustc_data_structures test:false 4.226
[01:12:06]  Compiling rls-analysis v0.14.0
[01:12:06] error[E0599]: no variant named `GlobalAsm` found for type `rustc::hir::def::Def` in the current scope
[01:12:06]     |
[01:12:06]     |
[01:12:06] 942 |         Def::GlobalAsm(id) => Some(id),
[01:12:06]     |         ^^^^^^^^^^^^^^^^^^ variant not found in `rustc::hir::def::Def`
[01:12:07]    Compiling rustc-ap-arena v211.0.0
[01:12:07] [RUSTC-TIMING] arena test:false 0.768
[01:12:07]    Compiling rustc-ap-arena v209.0.0
[01:12:08] [RUSTC-TIMING] arena test:false 0.772
---
$ rvm $(travis_internal_ruby) --fuzzy do ruby -S gem install dpl
Warning, new version of rvm available '1.29.4', you are using older version '1.29.3'.
You can disable this warning with:    echo rvm_autoupdate_flag=0 >> ~/.rvmrc
You can enable  auto-update  with:    echo rvm_autoupdate_flag=2 >> ~/.rvmrc
ERROR:  Could not find a valid gem 'dpl' (>= 0), here is why:
          Unable to download data from https://rubygems.org/ - no such name (https://rubygems.org/specs.4.8.gz)


The command "rvm $(travis_internal_ruby) --fuzzy do ruby -S gem install dpl" failed and exited with 2 during .
Your build has been stopped.

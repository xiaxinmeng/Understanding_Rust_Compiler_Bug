plain
[00:33:54]    Compiling aho-corasick v0.6.4
[00:34:01]    Compiling tempdir v0.3.7
[00:34:35]    Compiling minifier v0.0.11
[00:34:37]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:34:37] error: expected one of `.`, `;`, `?`, or an operator, found `let`
[00:34:37]     |
[00:34:37] 436 |     }
[00:34:37] 436 |     }
[00:34:37]     |      - expected one of `.`, `;`, `?`, or an operator here
[00:34:37] 437 | 
[00:34:37] 438 |     let test_args = matches.opt_strs("test-args");
[00:34:37]     |     ^^^ unexpected token
[00:34:39] error: unused import: `Path`
[00:34:39]   --> librustdoc/lib.rs:59:17
[00:34:39]    |
[00:34:39]    |
[00:34:39] 59 | use std::path::{Path, PathBuf};
[00:34:39]    |
[00:34:39]    = note: `-D unused-imports` implied by `-D warnings`
[00:34:39] 
[00:34:39] 
[00:34:39] error: unused import: `externalfiles::ExternalHtml`
[00:34:39]    |
[00:34:39]    |
[00:34:39] 64 | use externalfiles::ExternalHtml;
[00:34:39] 
[00:34:39] error: unused import: `build_codegen_options`
[00:34:39]   --> librustdoc/lib.rs:68:47
[00:34:39]    |
[00:34:39]    |
[00:34:39] 68 | use rustc::session::config::{nightly_options, build_codegen_options};
[00:34:39] 
[00:34:48] error[E0308]: mismatched types
[00:34:48]    --> librustdoc/html/render.rs:642:62
[00:34:48]     |
[00:34:48]     |
[00:34:48] 642 |                                           extern_location(e, extern_url, &cx.dst)));
[00:34:48]     |                                                              ^^^^^^^^^^ expected &str, found enum `std::option::Option`
[00:34:48]     = note: expected type `&str`
[00:34:48]     = note: expected type `&str`
[00:34:48]                found type `std::option::Option<&str>`
[00:34:48] error[E0308]: mismatched types
[00:34:48]     --> librustdoc/html/render.rs:1100:12
[00:34:48]      |
[00:34:48]      |
[00:34:48] 1100 |     if let Some(url) = extern_url {
[00:34:48]      |            ^^^^^^^^^ expected str, found enum `std::option::Option`
[00:34:48]      = note: expected type `str`
[00:34:48]                 found type `std::option::Option<_>`
[00:34:48] 
[00:34:52] error: aborting due to 6 previous errors
---
[00:34:52] 
[00:34:52] 
[00:34:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:34:52] Build completed unsuccessfully in 0:29:42
[00:34:52] make: *** [all] Error 1
[00:34:52] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00af419b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0da85186:start=1528833911277631734,finish=1528833911285533885,duration=7902151
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:094cecee
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.v

plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d5c307b1-ebd1-48fb-8fa6-7953d12cc3b2.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71690/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71690/merge:refs/remotes/pull/71690/merge
---
 ---> f7353ccad5b1
Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Using cache
 ---> ed38efbaa060
Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
 ---> c5008ef7ae8e
Successfully built c5008ef7ae8e
Successfully tagged rust-ci:latest
Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
Looks like docker image is the same as before, not uploading
[CI_JOB_NAME=mingw-check]
[CI_JOB_NAME=mingw-check]
== clock drift check ==
  local time: Wed Apr 29 19:30:35 UTC 2020
  network time: Wed, 29 Apr 2020 19:30:35 GMT
Starting sccache server...
configure: processing command line
configure: 
configure: rust.parallel-compiler := True
---
    Checking proc_macro v0.0.0 (/checkout/src/libproc_macro)
    Checking unicode-width v0.1.6
    Checking getopts v0.2.21
    Checking test v0.0.0 (/checkout/src/libtest)
{"reason":"build-finished","success":true}
Checking compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Checking cfg-if v0.1.10
   Compiling libc v0.2.69
   Compiling semver-parser v0.7.0
---
    Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
    Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
    Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
    Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
    Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
    Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
    Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
    Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
    Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
    Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
    Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
    Checking rustc-main v0.0.0 (/checkout/src/rustc)
{"reason":"build-finished","success":true}
Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Checking cfg-if v0.1.10
   Compiling libc v0.2.69
   Compiling semver-parser v0.7.0
---
   Compiling serde_derive v1.0.81
    Checking serde_json v1.0.40
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
    Checking rustdoc-tool v0.0.0 (/checkout/src/tools/rustdoc)
{"reason":"build-finished","success":true}
Build completed successfully in 0:05:25
configure: processing command line
configure: 
configure: rust.channel         := nightly
---
configure: rust.codegen-units-std := 1
configure: rust.verify-llvm-ir  := True
configure: build.cargo-native-static := True
configure: rust.dist-src        := False
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
Hugepagesize:       2048 kB
DirectMap4k:      157632 kB
DirectMap2M:     5085184 kB
DirectMap1G:     4194304 kB
+ python3 ../x.py test src/tools/expand-yaml-anchors
Ensuring the YAML anchors in the GitHub Actions config were expanded
Ensuring the YAML anchors in the GitHub Actions config were expanded
Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
   Compiling unicode-xid v0.2.0
   Compiling syn v1.0.11
   Compiling linked-hash-map v0.5.2
   Compiling lazy_static v1.4.0
   Compiling lazy_static v1.4.0
   Compiling yaml-rust v0.4.3
   Compiling quote v1.0.2
   Compiling thiserror-impl v1.0.5
   Compiling thiserror v1.0.5
   Compiling yaml-merge-keys v0.4.0
   Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
    Finished release [optimized] target(s) in 26.64s
{"reason":"build-finished","success":true}
+ python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
    Finished dev [unoptimized] target(s) in 0.18s
Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
Checking std artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
    Checking proc_macro v0.0.0 (/checkout/src/libproc_macro)
    Checking unicode-width v0.1.6
    Checking getopts v0.2.21
    Checking test v0.0.0 (/checkout/src/libtest)
 {"reason":"build-finished","success":true}
Checking compiler artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
    Checking cfg-if v0.1.10
   Compiling semver-parser v0.7.0
   Compiling winapi-i686-pc-windows-gnu v0.4.0
---
    Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
    Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
    Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
    Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
    Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
    Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
    Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
    Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
    Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
    Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
    Checking rustc-main v0.0.0 (/checkout/src/rustc)
    Finished release [optimized] target(s) in 2m 17s
{"reason":"build-finished","success":true}
    Checking cfg-if v0.1.10
   Compiling semver-parser v0.7.0
   Compiling proc-macro2 v0.4.30
    Checking lazy_static v1.4.0
---
    Checking serde_json v1.0.40
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
    Checking rustdoc-tool v0.0.0 (/checkout/src/tools/rustdoc)
    Finished release [optimized] target(s) in 58.97s
{"reason":"build-finished","success":true}
+ python3 ../x.py build --stage 0 src/tools/build-manifest
    Finished dev [unoptimized] target(s) in 0.17s
Building stage0 tool build-manifest (x86_64-unknown-linux-gnu)
   Compiling proc-macro2 v0.4.30
---
   Compiling toml v0.5.3
   Compiling serde_json v1.0.40
   Compiling build-manifest v0.1.0 (/checkout/src/tools/build-manifest)
    Finished release [optimized] target(s) in 50.49s
{"reason":"build-finished","success":true}
+ python3 ../x.py test --stage 0 src/tools/compiletest
    Finished dev [unoptimized] target(s) in 0.19s
   Compiling memchr v2.3.2
   Compiling log v0.4.8
---
   Compiling serde_json v1.0.40
   Compiling cargo_metadata v0.9.1
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 23.51s
{"reason":"build-finished","success":true}
* 596 error codes
* highest error code: E0753
* 283 features
Checking which error codes lack tests...
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
error: expected `;`, found ``assert_eq``
    |
272 |     }
272 |     }
    |      ^ help: add `;` here
273 |     assert_eq!(panic_msg, "bomb limit exceeded");
    |     --------- unexpected token

Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/libcore/tests/array.rs"` failed.
If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:38
== clock drift check ==
  local time: Wed Apr 29 19:43:48 UTC 2020
  local time: Wed Apr 29 19:43:48 UTC 2020
  network time: Wed, 29 Apr 2020 19:43:48 GMT


##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71690/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71690/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3858) (python)
##[section]Finishing: Finalize Job

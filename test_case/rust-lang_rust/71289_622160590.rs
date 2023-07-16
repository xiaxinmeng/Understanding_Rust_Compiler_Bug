plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/584e5434-ffb1-4969-a3c0-2b567adf7cad.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71289/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71289/merge:refs/remotes/pull/71289/merge
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
  local time: Thu Apr 30 21:46:46 UTC 2020
  network time: Thu, 30 Apr 2020 21:46:46 GMT
Starting sccache server...
configure: processing command line
configure: 
configure: rust.parallel-compiler := True
---
    Checking unicode-width v0.1.6
    Checking getopts v0.2.21
    Checking test v0.0.0 (/checkout/src/libtest)
    Finished release [optimized] target(s) in 23.01s
{"reason":"build-finished","success":true}
    Checking cfg-if v0.1.10
   Compiling libc v0.2.69
   Compiling semver-parser v0.7.0
    Checking lazy_static v1.4.0
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
    Finished release [optimized] target(s) in 2m 04s
{"reason":"build-finished","success":true}
    Checking cfg-if v0.1.10
   Compiling libc v0.2.69
   Compiling semver-parser v0.7.0
   Compiling proc-macro2 v0.4.30
---
   Compiling serde_derive v1.0.81
    Checking serde_json v1.0.40
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
    Checking rustdoc-tool v0.0.0 (/checkout/src/tools/rustdoc)
    Finished release [optimized] target(s) in 47.81s{"reason":"build-finished","success":true}
Build completed successfully in 0:04:42
configure: processing command line
configure: 
configure: rust.channel         := nightly
---
configure: rust.dist-src        := False
configure: build.locked-deps    := True
configure: build.submodules     := False
configure: build.cargo-native-static := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
Hugepagesize:       2048 kB
DirectMap4k:      124864 kB
DirectMap2M:     4069376 kB
DirectMap1G:     5242880 kB
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
    Finished release [optimized] target(s) in 22.32s
{"reason":"build-finished","success":true}
+ python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
    Finished dev [unoptimized] target(s) in 0.17s
Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
Checking std artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
    Checking unicode-width v0.1.6
    Checking getopts v0.2.21
    Checking test v0.0.0 (/checkout/src/libtest)
    Finished release [optimized] target(s) in 22.29s
{"reason":"build-finished","success":true}
    Checking cfg-if v0.1.10
   Compiling semver-parser v0.7.0
   Compiling winapi-i686-pc-windows-gnu v0.4.0
   Compiling winapi v0.3.8
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
Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
    Checking cfg-if v0.1.10
   Compiling semver-parser v0.7.0
   Compiling proc-macro2 v0.4.30
---
    Checking serde_json v1.0.40
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
    Checking rustdoc-tool v0.0.0 (/checkout/src/tools/rustdoc)
    Finished release [optimized] target(s) in 49.67s
{"reason":"build-finished","success":true}
+ python3 ../x.py build --stage 0 src/tools/build-manifest
    Finished dev [unoptimized] target(s) in 0.16s
Building stage0 tool build-manifest (x86_64-unknown-linux-gnu)
   Compiling proc-macro2 v0.4.30
---
   Compiling serde_derive v1.0.81
   Compiling toml v0.5.3
   Compiling serde_json v1.0.40
   Compiling build-manifest v0.1.0 (/checkout/src/tools/build-manifest)
{"reason":"build-finished","success":true}
Build completed successfully in 0:00:46
+ python3 ../x.py test --stage 0 src/tools/compiletest
    Finished dev [unoptimized] target(s) in 0.16s
   Compiling memchr v2.3.2
---
   Compiling semver v0.9.0
   Compiling cargo_metadata v0.9.1
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 20.14s
{"reason":"build-finished","success":true}
tidy check
tidy error: /checkout/src/librustdoc/passes/collect_intra_doc_links.rs:486: TODO is deprecated; use FIXME
tidy error: /checkout/src/librustdoc/passes/collect_intra_doc_links.rs:490: TODO is deprecated; use FIXME
tidy error: /checkout/src/librustdoc/clean/types.rs:78: TODO is deprecated; use FIXME
tidy error: /checkout/src/test/rustdoc/intra-link-self.rs:31: TODO is deprecated; use FIXME
tidy error: /checkout/src/test/rustdoc/intra-link-self.rs:40: line longer than 100 chars
tidy error: /checkout/src/test/rustdoc/intra-link-self.rs:74: line longer than 100 chars
tidy error: /checkout/src/test/rustdoc/intra-link-self.rs:100: TODO is deprecated; use FIXME
tidy error: /checkout/src/test/rustdoc/intra-link-self.rs:102: line longer than 100 chars
some tidy checks failed
Found 492 error codes
Found 0 error codes with no tests
Done!
Done!


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
Build completed unsuccessfully in 0:00:29
Build completed unsuccessfully in 0:00:29
== clock drift check ==
  local time: Thu Apr 30 21:58:17 UTC 2020
  network time: Thu, 30 Apr 2020 21:58:17 GMT


##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71289/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71289/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3243) (python)
##[section]Finishing: Finalize Job

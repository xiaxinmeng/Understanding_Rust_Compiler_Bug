plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4a3b0c5d-cb2e-4f81-bbd3-b2e7b1311687.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71724/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71724/merge:refs/remotes/pull/71724/merge
---
 ---> 3adb0605cc65
Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Using cache
 ---> 28dbc326cb7f
Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            python3 ../x.py doc --stage 0 src/libstd &&            /scripts/validate-toolstate.sh
 ---> 537a01811900
Successfully built 537a01811900
Successfully tagged rust-ci:latest
Built container sha256:537a018119009dc218456238dec90b5530050db1e2a1e166550c218003f6159d
---
    Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
    Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
    Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
    Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
    Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
    Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
configure: llvm.ccache          := sccache
configure: rust.codegen-units-std := 1
configure: rust.verify-llvm-ir  := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
Hugepagesize:       2048 kB
DirectMap4k:      149440 kB
DirectMap2M:     2996224 kB
DirectMap1G:     6291456 kB
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
Build completed successfully in 0:00:27
+ python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
    Finished dev [unoptimized] target(s) in 0.20s
Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
    Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
    Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
    Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
    Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
    Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
    Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
   Compiling cargo_metadata v0.9.1
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 23.18s
tidy check
tidy error: /checkout/src/test/rustdoc-js/doc-alias.js:36: tab character
tidy error: /checkout/src/test/rustdoc-js/doc-alias.js:47: tab character
tidy error: /checkout/src/test/rustdoc-js/doc-alias.js:58: tab character
tidy error: /checkout/src/test/rustdoc-js/doc-alias.js:77: tab character
tidy error: /checkout/src/test/rustdoc-js/doc-alias.js:79: tab character
tidy error: /checkout/src/test/rustdoc-js/doc-alias.js:88: tab character
tidy error: /checkout/src/test/rustdoc-js/doc-alias.js:99: tab character
tidy error: /checkout/src/test/rustdoc-js/doc-alias.js:110: tab character
tidy error: /checkout/src/test/rustdoc-js/doc-alias.js:121: tab character
tidy error: /checkout/src/test/rustdoc-js/doc-alias.js:132: tab character
tidy error: /checkout/src/test/rustdoc-js/doc-alias.js:143: tab character
tidy error: /checkout/src/test/rustdoc-js/doc-alias.js:154: tab character
tidy error: /checkout/src/test/rustdoc-js/doc-alias.js:165: tab character
tidy error: /checkout/src/test/rustdoc-js/doc-alias.js:176: tab character
tidy error: /checkout/src/test/rustdoc-js/doc-alias.js:187: tab character
tidy error: /checkout/src/test/rustdoc-js/doc-alias.js:198: tab character
tidy error: /checkout/src/test/rustdoc-js/doc-alias.js:209: tab character
tidy error: /checkout/src/test/rustdoc-js/doc-alias.js:220: tab character
tidy error: /checkout/src/test/rustdoc-js/doc-alias.js:237: tab character
tidy error: /checkout/src/test/rustdoc-js/doc-alias.js:248: tab character
tidy error: /checkout/src/test/rustdoc-js/doc-alias.js:259: tab character
some tidy checks failed
Found 492 error codes
Found 0 error codes with no tests
Done!
Done!


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
Build completed unsuccessfully in 0:00:35
Build completed unsuccessfully in 0:00:35
== clock drift check ==
  local time: Thu May  7 20:19:52 UTC 2020
  network time: Thu, 07 May 2020 20:19:52 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71724/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71724/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (13424) (python)
##[section]Finishing: Finalize Job

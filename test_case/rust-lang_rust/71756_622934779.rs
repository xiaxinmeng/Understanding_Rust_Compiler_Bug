plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/bb6e2e13-1eb3-4afd-adb2-ee0fa7ff8fcc.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71756/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71756/merge:refs/remotes/pull/71756/merge
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
configure: build.submodules     := False
configure: rust.channel         := nightly
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
Hugepagesize:       2048 kB
DirectMap4k:      135104 kB
DirectMap2M:     4059136 kB
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
Build completed successfully in 0:00:27
+ python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
    Finished dev [unoptimized] target(s) in 0.19s
Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/libstd/sys/windows/mod.rs at line 61:
         c::ERROR_FILE_NOT_FOUND => return ErrorKind::NotFound,
         c::ERROR_PATH_NOT_FOUND => return ErrorKind::NotFound,
         c::ERROR_NO_DATA => return ErrorKind::BrokenPipe,
-        c::ERROR_SEM_TIMEOUT |
-        c::WAIT_TIMEOUT |
-        c::ERROR_DRIVER_CANCEL_TIMEOUT |
-        c::ERROR_OPERATION_ABORTED |
-        c::ERROR_SERVICE_REQUEST_TIMEOUT |
-        c::ERROR_COUNTER_TIMEOUT |
-        c::ERROR_TIMEOUT |
-        c::ERROR_RESOURCE_CALL_TIMED_OUT |
-        c::ERROR_CTX_MODEM_RESPONSE_TIMEOUT |
-        c::ERROR_CTX_CLIENT_QUERY_TIMEOUT |
-        c::FRS_ERR_SYSVOL_POPULATE_TIMEOUT |
-        c::ERROR_DS_TIMELIMIT_EXCEEDED |
-        c::DNS_ERROR_RECORD_TIMED_OUT |
-        c::ERROR_IPSEC_IKE_TIMED_OUT |
-        c::ERROR_RUNLEVEL_SWITCH_TIMEOUT |
-        c::ERROR_RUNLEVEL_SWITCH_AGENT_TIMEOUT => return ErrorKind::TimedOut,
+        c::ERROR_SEM_TIMEOUT
+        | c::WAIT_TIMEOUT
+        | c::ERROR_DRIVER_CANCEL_TIMEOUT
+        | c::ERROR_OPERATION_ABORTED
+        | c::ERROR_SERVICE_REQUEST_TIMEOUT
+        | c::ERROR_COUNTER_TIMEOUT
+        | c::ERROR_TIMEOUT
+        | c::ERROR_RESOURCE_CALL_TIMED_OUT
+        | c::ERROR_CTX_MODEM_RESPONSE_TIMEOUT
+        | c::ERROR_CTX_CLIENT_QUERY_TIMEOUT
+        | c::FRS_ERR_SYSVOL_POPULATE_TIMEOUT
+        | c::ERROR_DS_TIMELIMIT_EXCEEDED
+        | c::DNS_ERROR_RECORD_TIMED_OUT
+        | c::ERROR_IPSEC_IKE_TIMED_OUT
+        | c::ERROR_RUNLEVEL_SWITCH_TIMEOUT
+        | c::ERROR_RUNLEVEL_SWITCH_AGENT_TIMEOUT => return ErrorKind::TimedOut,
     }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/libstd/sys/windows/mod.rs"` failed.
If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:40
== clock drift check ==
  local time: Sat May  2 09:34:18 UTC 2020
  network time: Sat, 02 May 2020 09:34:18 GMT
  network time: Sat, 02 May 2020 09:34:18 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71756/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71756/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4948) (python)
##[section]Finishing: Finalize Job

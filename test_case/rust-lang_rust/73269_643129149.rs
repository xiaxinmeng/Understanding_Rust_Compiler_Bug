plain
##[section]Starting: Linux mingw-check
##[section]Starting: Initialize job
Agent name: 'Hosted Agent'
Agent machine name: 'fv-az578'
Current agent version: '2.170.1'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200604.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200604.1/images/linux/Ubuntu1604-README.md
##[endgroup]
Agent running as: 'vsts'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.163.3)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0944d0f8-beca-4d8d-9cff-185469668383.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73269/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73269/merge:refs/remotes/pull/73269/merge
---
 ---> f883e675ad62
Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Using cache
 ---> c0b156eb069c
Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            python3 ../x.py doc --stage 0 src/libstd &&            /scripts/validate-toolstate.sh
 ---> 8541bab6b38c
Successfully built 8541bab6b38c
Successfully tagged rust-ci:latest
Built container sha256:8541bab6b38c07f1b7eb787539b9cbe93daa6ac4458d3d7bd8a8921622a14ba1
---
    Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
    Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
    Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
    Checking rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
    Checking chalk-rust-ir v0.10.0
    Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
    Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
    Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
    Checking chalk-solve v0.10.0
    Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
configure: rust.dist-src        := False
configure: llvm.assertions      := True
configure: rust.codegen-units-std := 1
configure: rust.verify-llvm-ir  := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
Hugepagesize:       2048 kB
DirectMap4k:      120768 kB
DirectMap2M:     5122048 kB
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
Build completed successfully in 0:00:34
+ python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
    Finished dev [unoptimized] target(s) in 0.20s
Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
    Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
    Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
    Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
    Checking rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
    Checking chalk-rust-ir v0.10.0
    Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
    Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
    Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
    Checking chalk-solve v0.10.0
    Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/libstd/sys/sgx/thread.rs at line 80:
         // is no guarantee about how long the thread will sleep on SGX platform.
         let timeout = cmp::min((u64::MAX - 1) as u128, dur.as_nanos()) as u64;
         let wait_error = rtunwrap!(Err, usercalls::wait(0, timeout));
-        rtassert!(wait_error.kind() == io::ErrorKind::TimedOut || wait_error.kind() == io::ErrorKind::WouldBlock);
+        rtassert!(
+            wait_error.kind() == io::ErrorKind::TimedOut
+                || wait_error.kind() == io::ErrorKind::WouldBlock
     }
 
     pub fn join(self) {
     pub fn join(self) {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/libstd/sys/sgx/thread.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:46
Build completed unsuccessfully in 0:00:46
Diff in /checkout/src/libstd/sys/sgx/waitqueue.rs at line 1:
 /// A simple queue implementation for synchronization primitives.
 ///
 /// This queue is used to implement condition variable and mutexes.
 /// This queue is used to implement condition variable and mutexes.
Diff in /checkout/src/libstd/sys/sgx/waitqueue.rs at line 12:
 /// The queue and associated wait state are stored in a `WaitVariable`.
 use crate::cmp;
 use crate::io;
+use crate::num::NonZeroUsize;
 use crate::ops::{Deref, DerefMut};
 
 
Diff in /checkout/src/libstd/sys/sgx/waitqueue.rs at line 164:
     /// Adds the calling thread to the `WaitVariable`'s wait queue, then wait
     /// until a wakeup event or timeout. If event was observed, returns true.
     /// If not, it will remove the calling thread from the wait queue.
-    pub fn wait_timeout<T, F: FnOnce()>(lock: &SpinMutex<WaitVariable<T>>, timeout: Duration, before_wait: F) -> bool {
+    pub fn wait_timeout<T, F: FnOnce()>(
+        lock: &SpinMutex<WaitVariable<T>>,
+        timeout: Duration,
+        before_wait: F,
+    ) -> bool {
         let timeout = cmp::min((u64::MAX - 1) as u128, timeout.as_nanos()) as u64;
         // very unsafe: check requirements of UnsafeList::push
         unsafe {
Diff in /checkout/src/libstd/sys/sgx/waitqueue.rs at line 177:
             // don't panic, this would invalidate `entry` during unwinding
             match usercalls::wait(EV_UNPARK, timeout) {
                 Ok(eventset) => rtassert!(eventset & EV_UNPARK == EV_UNPARK),
-                Err(e) => rtassert!(e.kind() == io::ErrorKind::TimedOut || e.kind() == io::ErrorKind::WouldBlock),
+                Err(e) => rtassert!(
+                    e.kind() == io::ErrorKind::TimedOut || e.kind() == io::ErrorKind::WouldBlock
             }
             }
             // acquire the wait queue's lock first to avoid deadlock.
             let mut guard = lock.lock();
  local time: Fri Jun 12 06:31:42 UTC 2020
  network time: Fri, 12 Jun 2020 06:31:42 GMT
== end clock drift check ==


##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73269/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73269/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4165) (python)
##[section]Finishing: Finalize Job

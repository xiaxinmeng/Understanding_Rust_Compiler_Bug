plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/thread/mod.rs at line 1068:
     // Used only internally to construct a thread object without spawning
     // Panics if the name contains nuls.
     pub(crate) fn new(name: Option<String>) -> Thread {
-        let cname =
-            name.map(|n| CString::new(n).expect("thread name must not contain interior null bytes"));
+        let cname = name
+            .map(|n| CString::new(n).expect("thread name must not contain interior null bytes"));
         Thread {
             inner: Arc::new(Inner { name: cname, id: ThreadId::new(), parker: Parker::new() }),
         }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys_common/os_str_bytes/tests.rs" "/checkout/library/std/src/sys_common/backtrace.rs" "/checkout/library/std/src/path/tests.rs" "/checkout/library/std/src/thread/tests.rs" "/checkout/library/std/src/thread/mod.rs" "/checkout/library/std/src/thread/local.rs" "/checkout/library/std/src/thread/local/dynamic_tests.rs" "/checkout/library/std/src/sync/barrier/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

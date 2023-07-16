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
Diff in /checkout/library/std/src/sys/unix/args.rs at line 98:
         // On Linux-GNU, we rely on `ARGV_INIT_ARRAY` below to initialize
         // `ARGC` and `ARGV`. But in Miri that does not actually happen so we
         // still initialize here.
-        #[cfg(any(
-            miri,
-            not(all(target_os = "linux", any(target_env = "gnu")))
-        ))]
+        #[cfg(any(miri, not(all(target_os = "linux", any(target_env = "gnu")))))]
         really_init(_argc, _argv);
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/unix/weak.rs" "/checkout/library/std/src/sys/unix/mutex.rs" "/checkout/library/std/src/sys/unix/args.rs" "/checkout/library/std/src/sys/unix/l4re.rs" "/checkout/library/std/src/sys/unix/os_str.rs" "/checkout/library/std/src/sys/unix/fd.rs" "/checkout/library/std/src/sys/unix/stdio.rs" "/checkout/library/std/src/sys/unix/net.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_data_structures/src/flock/linux.rs at line 14:
 
 impl Lock {
     pub fn new(p: &Path, wait: bool, create: bool, exclusive: bool) -> io::Result<Lock> {
-            .read(true)
-            .write(true)
-            .create(create)
-            .mode(0o600)
-            .mode(0o600)
-            .open(p)?;
+        let file = OpenOptions::new().read(true).write(true).create(create).mode(0o600).open(p)?;
 
         let mut operation = if exclusive { libc::LOCK_EX } else { libc::LOCK_SH };
         if !wait {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_data_structures/src/flock/unsupported.rs" "/checkout/compiler/rustc_data_structures/src/map_in_place.rs" "/checkout/compiler/rustc_data_structures/src/flock/linux.rs" "/checkout/compiler/rustc_data_structures/src/flock/windows.rs" "/checkout/compiler/rustc_data_structures/src/work_queue.rs" "/checkout/compiler/rustc_data_structures/src/vec_map/tests.rs" "/checkout/compiler/rustc_data_structures/src/stable_hasher/tests.rs" "/checkout/compiler/rustc_data_structures/src/flock/unix.rs"` failed.
Build completed unsuccessfully in 0:00:09
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

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
Diff in /checkout/library/std/src/sys/unix/os.rs at line 392:
         let mut info: mem::MaybeUninit<libc::image_info> = mem::MaybeUninit::uninit();
         let mut cookie: i32 = 0;
         // the executable can be found at team id 0
-        let result =
-            libc::_get_next_image_info(0, &mut cookie, info.as_mut_ptr(), mem::size_of::<libc::image_info>());
+        let result = libc::_get_next_image_info(
+            &mut cookie,
+            info.as_mut_ptr(),
+            info.as_mut_ptr(),
+            mem::size_of::<libc::image_info>(),
+        );
         if result != 0 {
             use crate::io::ErrorKind;
             Err(io::Error::new_const(ErrorKind::Uncategorized, &"Error getting executable path"))
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/unix/os.rs" "/checkout/library/std/src/sys/unix/weak.rs" "/checkout/library/std/src/sys/unix/args.rs" "/checkout/library/std/src/sys/unix/l4re.rs" "/checkout/library/std/src/sys/unix/mutex.rs" "/checkout/library/std/src/sys/unix/fd.rs" "/checkout/library/std/src/sys/unix/stdio.rs" "/checkout/library/std/src/sys/unix/net.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

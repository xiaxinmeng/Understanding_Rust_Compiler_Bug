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
Diff in /checkout/compiler/rustc_mir/src/interpret/intrinsics.rs at line 226:
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/interpret/intrinsics.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
                 let l = self.read_immediate(&args[0])?;
                 let r = self.read_immediate(&args[1])?;
                 let is_add = intrinsic_name == sym::saturating_add;
-                let (val, overflowed, _ty) =
-                    self.overflowing_binary_op(if is_add { BinOp::Add } else { BinOp::Sub }, &l, &r)?;
+                let (val, overflowed, _ty) = self.overflowing_binary_op(
+                    if is_add { BinOp::Add } else { BinOp::Sub },
+                    &l,
+                    &r,
+                )?;
                 let val = if overflowed {
                     let num_bits = l.layout.size.bits();
                     if l.layout.abi.is_signed() {
Build completed unsuccessfully in 0:00:20

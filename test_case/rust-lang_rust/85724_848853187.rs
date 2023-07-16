plain
Successfully built 978f2e0c259b
Successfully tagged rust-ci:latest
Built container sha256:978f2e0c259b7ecab874e6ce1a11cb30fdab689b2471ec8b3ad874a990799d36
Uploading finished image to https://ci-caches.rust-lang.org/docker/5077ec353cd1491e6ee21dfbed017c039592486e2f15c335d9cc920ab986ce86dc748c415ad9e07b56bf3231849d98e46583667527e0138e1dc31d247bb94cde
upload failed: - to s3://rust-lang-ci-sccache2/docker/5077ec353cd1491e6ee21dfbed017c039592486e2f15c335d9cc920ab986ce86dc748c415ad9e07b56bf3231849d98e46583667527e0138e1dc31d247bb94cde Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
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
Diff in /checkout/compiler/rustc_typeck/src/check/upvar.rs at line 1591:
             // We need to restrict Fake Read precision to avoid fake reading unsafe code,
             // such as deref of a raw pointer.
             let place = restrict_capture_precision(place);
-            let place = restrict_repr_packed_field_ref_capture(
-                self.fcx.tcx,
-                self.fcx.param_env,
-                &place,
+            let place =
+            let place =
+                restrict_repr_packed_field_ref_capture(self.fcx.tcx, self.fcx.param_env, &place);
             self.fake_reads.push((place, cause, diag_expr_id));
     }
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/_impl.rs" "/checkout/compiler/rustc_typeck/src/check/upvar.rs" "/checkout/compiler/rustc_typeck/src/check/intrinsic.rs" "/checkout/compiler/rustc_typeck/src/check/dropck.rs" "/checkout/compiler/rustc_session/src/lib.rs" "/checkout/compiler/rustc_typeck/src/check/op.rs" "/checkout/compiler/rustc_session/src/search_paths.rs" "/checkout/compiler/rustc_typeck/src/check/cast.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14

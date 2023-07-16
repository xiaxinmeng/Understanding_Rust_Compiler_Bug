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
Diff in /checkout/compiler/rustc_middle/src/mir/interpret/error.rs at line 393:
                 "scalar size mismatch: expected {} bytes but got {} bytes instead",
             ),
             ),
-            UninhabitedEnumVariantWritten => write!(f, "writing discriminant of an uninhabited enum"),
+            UninhabitedEnumVariantWritten => {
+                write!(f, "writing discriminant of an uninhabited enum")
         }
     }
 }
 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/util/common.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/error.rs" "/checkout/compiler/rustc_middle/src/util/bug.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/value.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/queries.rs" "/checkout/compiler/rustc_middle/src/mir/mono.rs" "/checkout/compiler/rustc_middle/src/util/common/tests.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/pointer.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

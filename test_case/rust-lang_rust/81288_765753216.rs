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
Diff in /checkout/src/librustdoc/clean/mod.rs at line 1096:
                     AssocTypeItem(bounds.clean(cx), default.clean(cx))
             };
             };
-            let what_rustc_thinks = Item::from_def_id_and_parts(local_did, Some(self.ident.name), inner, cx);
+            let what_rustc_thinks =
+                Item::from_def_id_and_parts(local_did, Some(self.ident.name), inner, cx);
             // Trait items always have the trait's visibility -- we don't want to show `pub`.
             Item { visibility: Inherited, ..what_rustc_thinks }
         })
Diff in /checkout/src/librustdoc/clean/mod.rs at line 1133:
                 }
             };
             };
-            let what_rustc_thinks = Item::from_def_id_and_parts(local_did, Some(self.ident.name), inner, cx);
+            let what_rustc_thinks =
+                Item::from_def_id_and_parts(local_did, Some(self.ident.name), inner, cx);
             // Trait impl items always have the trait's/impl's visibility --
             // we don't want to show `pub`.
             Item { visibility: Inherited, ..what_rustc_thinks }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/clean/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:20

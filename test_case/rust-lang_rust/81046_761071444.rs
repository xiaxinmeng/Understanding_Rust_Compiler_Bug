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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_resolve/src/lib.rs at line 2459:
                         }
                     } else {
                         let parent = path[i - 1].ident.name;
-                        let parent =
-                            if parent == kw::PathRoot { "crate root".to_owned() } else { format!("`{}`", parent) };
+                        let parent = if parent == kw::PathRoot {
+                            "crate root".to_owned()
+                        } else {
+                            format!("`{}`", parent)
 
 
                         let mut msg = format!("could not find `{}` in {}", ident, parent);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_resolve/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
                         if ns == TypeNS || ns == ValueNS {
Build completed unsuccessfully in 0:00:15

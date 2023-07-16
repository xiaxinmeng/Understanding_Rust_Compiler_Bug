plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
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
Diff in /checkout/src/librustdoc/clean/cfg.rs at line 72:
         match cfg.kind {
             MetaItemKind::Word => {
                 let cfg = Cfg::Cfg(name, None);
-                if exclude.contains(&cfg) {
-                    Ok(None)
-                } else {
-                    Ok(Some(cfg))
-                }
+                if exclude.contains(&cfg) { Ok(None) } else { Ok(Some(cfg)) }
             }
             MetaItemKind::NameValue(ref lit) => match lit.kind {
                 LitKind::Str(value, _) => {
Diff in /checkout/src/librustdoc/clean/cfg.rs at line 83:
                     let cfg = Cfg::Cfg(name, Some(value));
-                    if exclude.contains(&cfg) {
-                        Ok(None)
-                    } else {
-                        Ok(Some(cfg))
-                    }
+                    if exclude.contains(&cfg) { Ok(None) } else { Ok(Some(cfg)) }
                 }
                 _ => Err(InvalidCfgError {
                     // FIXME: if the main #[cfg] syntax decided to support non-string literals,
Build completed unsuccessfully in 0:00:13
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/proc_macro/src/bridge/handle.rs" "/checkout/library/proc_macro/src/bridge/rpc.rs" "/checkout/library/proc_macro/src/bridge/mod.rs" "/checkout/library/proc_macro/src/bridge/closure.rs" "/checkout/src/librustdoc/clean/cfg.rs" "/checkout/library/std/benches/hash/map.rs" "/checkout/library/std/benches/hash/mod.rs" "/checkout/library/proc_macro/src/diagnostic.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

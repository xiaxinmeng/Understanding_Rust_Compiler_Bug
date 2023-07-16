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
#########                                                                 12.8%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-06-29/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/src/bootstrap/dist.rs at line 40:
         println!("Unable to build {}, skipping dist", tool_name)
     } else {
         let help = "note: not all tools are available on all nightlies\nhelp: see https://forge.rust-lang.org/infra/toolstate.html for more information";
-        panic!("Unable to build submodule tool {} (use `missing-tools = true` to ignore this failure)\n{}", tool_name, help)
+        panic!(
+            "Unable to build submodule tool {} (use `missing-tools = true` to ignore this failure)\n{}",
+            tool_name, help
     }
 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/builder/tests.rs" "/checkout/src/bootstrap/tool.rs" "/checkout/src/bootstrap/bin/sccache-plus-cl.rs" "/checkout/src/bootstrap/dist.rs" "/checkout/src/bootstrap/compile.rs" "/checkout/src/bootstrap/clean.rs" "/checkout/src/bootstrap/dylib_util.rs" "/checkout/src/bootstrap/bin/main.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', format.rs:175:19
Build completed unsuccessfully in 0:00:15

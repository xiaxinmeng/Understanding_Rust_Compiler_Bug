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
Diff in /checkout/src/bootstrap/sanity.rs at line 96:
         if let Some(explicit_name) = build.config.cmake.take() {
             if let Some(cmake_from_env) = env::var_os("CAKE") {
                 if explicit_name != cmake_from_env {
-                    eprintln!("env var CMAKE = {cmake_from_env:?} != {explicit_name:?} from config.toml")
+                    eprintln!(
+                        "env var CMAKE = {cmake_from_env:?} != {explicit_name:?} from config.toml"
                 }
             }
             }
             build.config.cmake = cmd_finder.must_have(explicit_name).into();
Diff in /checkout/src/bootstrap/sanity.rs at line 108:
                 cmd_finder.must_have(cmake_from_env)
             } else {
                 cmd_finder.must_have("cmake")
-            }.into()
+            .into()
         }
     } else {
         None
         None
Diff in /checkout/src/bootstrap/sanity.rs at line 219:
 
 
         if target.contains("msvc") {
-            if let Some(ref cmake_path) = need_default_cmake.as_ref().or(build.config.cmake.as_ref()) {
+            if let Some(ref cmake_path) =
+                need_default_cmake.as_ref().or(build.config.cmake.as_ref())
+            {
                 // There are three builds of cmake on windows: MSVC, MinGW, and
                 // Cygwin. The Cygwin build does not have generators for Visual
                 // Studio, so detect that here and error.
Diff in /checkout/src/bootstrap/sanity.rs at line 226:
-                let out = output( Command::new(cmake_path).arg("--help"));
+                let out = output(Command::new(cmake_path).arg("--help"));
                 if !out.contains("Visual Studio") {
                         "
Diff in /checkout/src/bootstrap/lib.rs at line 646:
Diff in /checkout/src/bootstrap/lib.rs at line 646:
             job::setup(self);
         }
         if let Some(cmake) = self.config.cmake.as_ref() {
-            env::set_var("CMAKE", cmake);  // https://docs.rs/cmake/0.1.48/src/cmake/lib.rs.html#515
+            env::set_var("CMAKE", cmake); // https://docs.rs/cmake/0.1.48/src/cmake/lib.rs.html#515
 
         self.maybe_update_submodules();
         self.maybe_update_submodules();
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/test.rs" "/checkout/src/bootstrap/lib.rs" "/checkout/src/bootstrap/bin/main.rs" "/checkout/src/bootstrap/dist.rs" "/checkout/src/bootstrap/bin/rustc.rs" "/checkout/src/bootstrap/config.rs" "/checkout/src/bootstrap/bin/rustdoc.rs" "/checkout/src/tools/unicode-table-generator/src/unicode_download.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

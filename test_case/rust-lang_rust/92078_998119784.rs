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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/tools/jsondocck/src/main.rs at line 62:
         };
 
         if !count {
-            print_err(&format!("Incorrect number of arguments to `@{}` (got {})", self, args.len()), lineno);
+            print_err(
+                &format!("Incorrect number of arguments to `@{}` (got {})", self, args.len()),
+            );
             return false;
         }
 
 
Diff in /checkout/src/tools/compiletest/src/runtest.rs at line 2413:
             self.fatal_proc_rec("jsondocck failed!", &res)
 
 
-        let crate_name = self.testpaths.file.file_stem().unwrap().to_str().unwrap().replace('-', "_");
+        let crate_name =
+            self.testpaths.file.file_stem().unwrap().to_str().unwrap().replace('-', "_");
         let mut json_out = out_dir.join(crate_name);
         json_out.set_extension("json");
         let res = self.cmd2procres(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/compiletest/src/runtest/debugger.rs" "/checkout/src/tools/compiletest/src/runtest/tests.rs" "/checkout/src/tools/compiletest/src/runtest.rs" "/checkout/src/tools/rustdoc-themes/main.rs" "/checkout/src/tools/jsondocck/src/config.rs" "/checkout/src/tools/unicode-table-generator/src/unicode_download.rs" "/checkout/src/tools/unicode-table-generator/src/case_mapping.rs" "/checkout/src/tools/compiletest/src/common.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

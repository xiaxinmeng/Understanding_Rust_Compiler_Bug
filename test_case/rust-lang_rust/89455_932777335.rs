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
Diff in /checkout/src/bootstrap/flags.rs at line 317:
                     "enable this to generate a Rustfix coverage file, which is saved in \
                         `/<build_base>/rustfix_missing_coverage.txt`",
-                opts.optflag(
-                    "",
-                    "check-diagnostics",
-                    "check-diagnostics",
-                    "runs a lint for diagnostic messages."
-                );
+                opts.optflag("", "check-diagnostics", "runs a lint for diagnostic messages.");
             }
             "check" | "c" => {
                 opts.optflag("", "all-targets", "Check all targets");
Diff in /checkout/src/bootstrap/flags.rs at line 770:
     pub fn check_diagnostics(&self) -> bool {
         match *self {
         match *self {
-            Subcommand::Test {check_diagnostics, ..} => check_diagnostics,
+            Subcommand::Test { check_diagnostics, .. } => check_diagnostics,
         }
     }
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/build.rs" "/checkout/src/bootstrap/config.rs" "/checkout/src/bootstrap/check.rs" "/checkout/src/bootstrap/clean.rs" "/checkout/src/bootstrap/lib.rs" "/checkout/src/bootstrap/setup.rs" "/checkout/src/bootstrap/cc_detect.rs" "/checkout/src/bootstrap/flags.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

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
Diff in /checkout/src/bootstrap/metrics.rs at line 7:
 use crate::builder::Step;
 use crate::Build;
 use build_helper::t;
-use serde::{Serialize, Deserialize};
+use serde::{Deserialize, Serialize};
 use std::io::BufWriter;
 use std::io::BufWriter;
 use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};
Diff in /checkout/src/bootstrap/metrics.rs at line 130:
             childs: steps.into_iter().map(|step| self.prepare_json_step(step)).collect(),
 
 
-        let json = JsonRoot {
-            system_stats,
-        };
-        };
+        let json = JsonRoot { system_stats, invocations };
 
         t!(std::fs::create_dir_all(dest.parent().unwrap()));
         let mut file = BufWriter::new(t!(File::create(&dest)));
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/metrics.rs" "/checkout/src/tools/compiletest/src/runtest/tests.rs" "/checkout/src/bootstrap/run.rs" "/checkout/src/tools/compiletest/src/runtest/debugger.rs" "/checkout/src/bootstrap/util.rs" "/checkout/src/bootstrap/doc.rs" "/checkout/src/tools/compiletest/src/json.rs" "/checkout/src/bootstrap/builder.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

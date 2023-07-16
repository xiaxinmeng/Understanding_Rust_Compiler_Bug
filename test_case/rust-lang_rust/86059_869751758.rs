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
Diff in /checkout/src/bootstrap/test.rs at line 200:
         builder.default_doc(&[]);
         builder.ensure(crate::doc::Rustc { target: self.target, stage: builder.top_stage });
-        try_run(
-            builder,
-            builder
-            builder
-                .tool_cmd(Tool::HtmlChecker)
-                .arg(builder.doc_out(self.target)),
-        );
+        try_run(builder, builder.tool_cmd(Tool::HtmlChecker).arg(builder.doc_out(self.target)));
 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/compile.rs" "/checkout/src/bootstrap/test.rs" "/checkout/src/bootstrap/builder.rs" "/checkout/src/bootstrap/clean.rs" "/checkout/src/bootstrap/builder/tests.rs" "/checkout/src/bootstrap/tool.rs" "/checkout/src/bootstrap/lib.rs" "/checkout/src/bootstrap/check.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14

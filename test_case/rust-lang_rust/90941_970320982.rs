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
Diff in /checkout/compiler/rustc_errors/src/lib.rs at line 222:
     fn transcribe<'a>(&self, text: Option<Cow<'a, str>>) -> Option<Cow<'a, str>> {
         match self.transcription {
             Transcription::Copy => text,
-            Transcription::Blank => text.map(|s| {
-                s.chars().map(|c| if c.is_whitespace() { c } else { ' ' }).collect()
-            }),
+            Transcription::Blank => {
+                text.map(|s| s.chars().map(|c| if c.is_whitespace() { c } else { ' ' }).collect())
         }
     }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/spec/mipsel_sony_psp.rs" "/checkout/compiler/rustc_target/src/spec/thumbv7neon_unknown_linux_musleabihf.rs" "/checkout/compiler/rustc_target/src/spec/i686_unknown_netbsd.rs" "/checkout/compiler/rustc_target/src/spec/l4re_base.rs" "/checkout/compiler/rustc_errors/src/styled_buffer.rs" "/checkout/compiler/rustc_target/src/spec/armv7_unknown_linux_gnueabi.rs" "/checkout/compiler/rustc_errors/src/lib.rs" "/checkout/compiler/rustc_target/src/spec/wasm32_unknown_unknown.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

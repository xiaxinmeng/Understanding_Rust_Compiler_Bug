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
Diff in /checkout/compiler/rustc_span/src/tests.rs at line 3:
 #[test]
 fn test_lookup_line() {
     let source = "abcdefghijklm\nabcdefghij\n...".to_owned();
-    let sf = SourceFile::new(FileName::Anon(0), source, BytePos(3), SourceFileHashAlgorithm::Sha256);
+    let sf =
+        SourceFile::new(FileName::Anon(0), source, BytePos(3), SourceFileHashAlgorithm::Sha256);
     assert_eq!(sf.lines.as_slice(), &[BytePos(3), BytePos(17), BytePos(28)]);
 
     assert_eq!(sf.lookup_line(BytePos(0)), None);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_span/src/source_map/tests.rs" "/checkout/compiler/rustc_span/src/symbol.rs" "/checkout/compiler/rustc_span/src/symbol/tests.rs" "/checkout/compiler/rustc_span/src/lib.rs" "/checkout/compiler/rustc_span/src/crate_disambiguator.rs" "/checkout/compiler/rustc_span/src/edition.rs" "/checkout/compiler/rustc_span/src/tests.rs" "/checkout/compiler/rustc_ty_utils/src/representability.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

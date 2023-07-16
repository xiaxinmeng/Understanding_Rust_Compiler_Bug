plain
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 218:
                     }
                     Res::Def(DefKind::Mod, _) => {
                         // This resolved to a module, but we want primitive types to take precedence instead.
-                        if matches!(disambiguator, None | Some(Disambiguator::Namespace(Namespace::TypeNS))) {
+                        if matches!(
+                            disambiguator,
+                            None | Some(Disambiguator::Namespace(Namespace::TypeNS))
+                        ) {
                             if let Some(prim) = is_primitive(path_str, ns) {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/passes/collect_intra_doc_links.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
                                 if extra_fragment.is_some() {
                                     return Err(ErrorKind::AnchorFailure(AnchorFailure::Primitive));
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
== clock drift check ==
  local time: Sun Aug  9 05:24:22 UTC 2020
  network time: Sun, 09 Aug 2020 05:24:22 GMT
== end clock drift check ==
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (2871) (node)
Terminate orphan process: pid (2899) (python)

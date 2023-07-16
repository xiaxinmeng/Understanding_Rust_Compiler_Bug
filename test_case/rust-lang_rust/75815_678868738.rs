plain
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 741:
 
                 match disambiguator.map(Disambiguator::ns) {
                     Some(ns @ (ValueNS | TypeNS)) => {
-                        match self.resolve(
-                            ns,
-                            ns,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/passes/collect_intra_doc_links.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
-                            &current_item,
-                            base_node,
-                            &extra_fragment,
-                        ) {
+                        match self.resolve(path_str, ns, &current_item, base_node, &extra_fragment)
                             Ok(res) => res,
                             Ok(res) => res,
                             Err(ErrorKind::ResolutionFailure) => {
                                 resolution_failure(cx, &item, path_str, &dox, link_range);
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
== clock drift check ==
  local time: Mon Aug 24 02:16:35 UTC 2020
  network time: Mon, 24 Aug 2020 02:16:36 GMT
== end clock drift check ==
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (2910) (node)
Terminate orphan process: pid (2938) (python)

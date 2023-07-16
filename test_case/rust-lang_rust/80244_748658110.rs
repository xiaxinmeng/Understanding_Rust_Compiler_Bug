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
Diff in /checkout/src/librustdoc/html/markdown.rs at line 460:
     }
 }
 
-impl<'a, 'b, 'ids, I: Iterator<Item = (Event<'a>, Range<usize>)>> Iterator for HeadingLinks<'a, 'b, 'ids, I> {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/markdown.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
+impl<'a, 'b, 'ids, I: Iterator<Item = (Event<'a>, Range<usize>)>> Iterator
+    for HeadingLinks<'a, 'b, 'ids, I>
+{
     type Item = (Event<'a>, Range<usize>);
 
     fn next(&mut self) -> Option<Self::Item> {
Diff in /checkout/src/librustdoc/html/markdown.rs at line 1168:
         match md[span.clone()].rfind(link) {
             Some(start) => {
                 let start = span.start + start;
-                start .. start + link.len()
+                start..start + link.len()
             }
             // This can happen for things other than intra-doc links, like `#1` expanded to `https://github.com/rust-lang/rust/issues/1`.
             None => span,
Build completed unsuccessfully in 0:00:21

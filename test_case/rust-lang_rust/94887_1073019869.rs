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
     }
 }
 
-
 struct PrefixParserSlice<'a, 'b> {
     parser: &'b PrefixParser<'a>,
Diff in /checkout/library/std/src/sys/windows/path.rs at line 84:
 
 
 impl<'a> PrefixParserSlice<'a, '_> {
     fn strip_prefix(&self, prefix: &str) -> Option<Self> {
-        self.parser
-            .prefix[self.index..]
+        self.parser.prefix[self.index..]
             .starts_with(prefix.as_bytes())
             .then(|| Self { parser: self.parser, index: self.index + prefix.len() })
Diff in /checkout/library/std/src/sys/windows/path.rs at line 272:
Diff in /checkout/library/std/src/sys/windows/path.rs at line 272:
         // SAFETY: `fill_utf16_buf` ensures the `buffer` and `size` are valid.
         // `lpfilename` is a pointer to a null terminated string that is not
         // invalidated until after `GetFullPathNameW` returns successfully.
-        |buffer, size| unsafe {
-            c::GetFullPathNameW(lpfilename, size, buffer, ptr::null_mut())
-        },
+        |buffer, size| unsafe { c::GetFullPathNameW(lpfilename, size, buffer, ptr::null_mut()) },
         |mut absolute| {
             path.clear();
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys_common/io.rs" "/checkout/library/std/src/sys_common/tests.rs" "/checkout/library/std/src/sys/windows/io.rs" "/checkout/library/std/src/sys_common/memchr/tests.rs" "/checkout/library/std/src/sys_common/net/tests.rs" "/checkout/library/std/src/sys/windows/args.rs" "/checkout/library/std/src/sys/windows/path.rs" "/checkout/library/std/src/sys/windows/net.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

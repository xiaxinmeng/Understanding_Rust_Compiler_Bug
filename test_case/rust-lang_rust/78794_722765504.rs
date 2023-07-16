
Diff in /checkout/compiler/rustc_expand/src/expand.rs at line 1436:
                 item.attrs = attrs;
                 self.check_attributes(&item.attrs);
                 item.and_then(|item| match item.kind {
-                    ItemKind::MacCall(mac) => self
-                        .collect_bang(mac, span, AstFragmentKind::Items)
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_expand/src/expand.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
-                        .make_items(),
+                    ItemKind::MacCall(mac) => {
+                        self.collect_bang(mac, span, AstFragmentKind::Items).make_items()
+                    }

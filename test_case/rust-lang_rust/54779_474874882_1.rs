
error: lifetime may not live long enough
  --> src/main.rs:28:24
   |
26 |     fn fmt_with(&self, cx: &dyn DebugContext, _fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   |                            - let's call the lifetime of this reference `'1`
27 |         let Foo { bar } = self;
28 |         bar.debug_with(cx);
   |                        ^^ cast requires that `'1` must outlive `'static`

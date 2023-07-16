
error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
  --> src/main.rs:26:24
   |
26 |         bar.debug_with(cx);
   |                        ^^
   |
note: first, the lifetime cannot outlive the anonymous lifetime #2 defined on the method body at 24:5...
  --> src/main.rs:24:5
   |
24 | /     fn fmt_with(&self, cx: &dyn DebugContext, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
25 | |         let Foo { bar } = self;
26 | |         bar.debug_with(cx);
27 | |         Ok(())
28 | |     }
   | |_____^
note: ...so that the declared lifetime parameter bounds are satisfied
  --> src/main.rs:26:24
   |
26 |         bar.debug_with(cx);
   |                        ^^
   = note: but, the lifetime must be valid for the static lifetime...
   = note: ...so that the expression is assignable:
           expected &(dyn DebugContext + 'static)
              found &dyn DebugContext

error: aborting due to previous error

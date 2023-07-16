diff
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{0} {0}", local_i32);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{0} {0}", local_i32);
-LL +     println!("{local_i32} {local_i32}");
+thread 'rustc' panicked at 'assertion failed: `(left == right)`
+  left: `None`,
+  left: `None`,
+ right: `Some([SubstitutionPart { span: $DIR/uninlined_format_args.rs:78:23: 78:34 (#0), snippet: "" }, SubstitutionPart { span: $DIR/uninlined_format_args.rs:78:23: 78:34 (#0), snippet: "" }])`: suggestion must not have overlapping parts', /checkout/compiler/rustc_errors/src/diagnostic.rs:646:9

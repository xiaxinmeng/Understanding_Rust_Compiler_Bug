plain

---- compile_test stdout ----
diff of stderr:

 error: variables can be used directly in the `format!` string
    |
    |
 LL |     println!("val='{}'", local_i32);
    |
    |
    = note: `-D clippy::uninlined-format-args` implied by `-D warnings`
    |
    |
 LL -     println!("val='{}'", local_i32);
 LL +     println!("val='{local_i32}'");
 
 
 error: variables can be used directly in the `format!` string
    |
    |
 LL |     println!("val='{   }'", local_i32); // 3 spaces
    |
 help: change this to
    |
    |
 LL -     println!("val='{   }'", local_i32); // 3 spaces
 LL +     println!("val='{local_i32}'"); // 3 spaces
    |
 
 
 error: variables can be used directly in the `format!` string
    |
    |
 LL |     println!("val='{    }'", local_i32); // tab
    |
 help: change this to
    |
    |
 LL -     println!("val='{    }'", local_i32); // tab
 LL +     println!("val='{local_i32}'"); // tab
 
 
 error: variables can be used directly in the `format!` string
    |
    |
 LL |     println!("val='{     }'", local_i32); // space+tab
    |
 help: change this to
    |
    |
 LL -     println!("val='{     }'", local_i32); // space+tab
 LL +     println!("val='{local_i32}'"); // space+tab
 
 
 error: variables can be used directly in the `format!` string
    |
    |
 LL |     println!("val='{     }'", local_i32); // tab+space
    |
 help: change this to
    |
    |
 LL -     println!("val='{     }'", local_i32); // tab+space
 LL +     println!("val='{local_i32}'"); // tab+space
 
 
 error: variables can be used directly in the `format!` string
    |
 LL | /     println!(
 LL | /     println!(
 LL | |         "val='{
 LL | |     }'",
 LL | |         local_i32
 LL | |     );
 
 
 error: variables can be used directly in the `format!` string
    |
 LL |     println!("{}", local_i32);
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
 help: change this to
    |
 LL -     println!("{}", local_i32);
 LL +     println!("{local_i32}");
 
 
 error: variables can be used directly in the `format!` string
    |
 LL |     println!("{}", fn_arg);
    |     ^^^^^^^^^^^^^^^^^^^^^^
    |
    |
 help: change this to
    |
 LL -     println!("{}", fn_arg);
 LL +     println!("{fn_arg}");
 
 
 error: variables can be used directly in the `format!` string
    |
 LL |     println!("{:?}", local_i32);
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
 help: change this to
    |
 LL -     println!("{:?}", local_i32);
 LL +     println!("{local_i32:?}");
 
 
 error: variables can be used directly in the `format!` string
    |
 LL |     println!("{:#?}", local_i32);
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
 help: change this to
    |
 LL -     println!("{:#?}", local_i32);
 LL +     println!("{local_i32:#?}");
 
 
 error: variables can be used directly in the `format!` string
    |
    |
 LL |     println!("{:4}", local_i32);
    |
 help: change this to
    |
    |
 LL -     println!("{:4}", local_i32);
 LL +     println!("{local_i32:4}");
 
 
 error: variables can be used directly in the `format!` string
    |
    |
 LL |     println!("{:04}", local_i32);
    |
 help: change this to
    |
    |
 LL -     println!("{:04}", local_i32);
 LL +     println!("{local_i32:04}");
 
 
 error: variables can be used directly in the `format!` string
    |
    |
 LL |     println!("{:<3}", local_i32);
    |
 help: change this to
    |
    |
 LL -     println!("{:<3}", local_i32);
 LL +     println!("{local_i32:<3}");
 
 
 error: variables can be used directly in the `format!` string
    |
    |
 LL |     println!("{:#010x}", local_i32);
    |
 help: change this to
    |
    |
 LL -     println!("{:#010x}", local_i32);
 LL +     println!("{local_i32:#010x}");
 
 
 error: variables can be used directly in the `format!` string
    |
    |
 LL |     println!("{:.1}", local_f64);
    |
 help: change this to
    |
    |
 LL -     println!("{:.1}", local_f64);
 LL +     println!("{local_f64:.1}");
 
 
 error: variables can be used directly in the `format!` string
    |
    |
 LL |     println!("{} {}", local_i32, local_f64);
    |
 help: change this to
    |
    |
 LL -     println!("{} {}", local_i32, local_f64);
 LL +     println!("{local_i32} {local_f64}");
 
 
 error: variables can be used directly in the `format!` string
    |
 LL |     println!("{}", val);
    |     ^^^^^^^^^^^^^^^^^^^
    |
    |
 help: change this to
    |
 LL -     println!("{}", val);
 LL +     println!("{val}");
 
 
 error: variables can be used directly in the `format!` string
    |
 LL |     println!("{}", v = val);
    |     ^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
 help: change this to
    |
 LL -     println!("{}", v = val);
 LL +     println!("{val}");
 
 
 error: variables can be used directly in the `format!` string
    |
    |
 LL |     println!("val='{/t }'", local_i32);
    |
 help: change this to
    |
    |
 LL -     println!("val='{/t }'", local_i32);
 LL +     println!("val='{local_i32}'");
 
 
 error: variables can be used directly in the `format!` string
    |
    |
 LL |     println!("val='{/n }'", local_i32);
    |
 help: change this to
    |
    |
 LL -     println!("val='{/n }'", local_i32);
 LL +     println!("val='{local_i32}'");
 
 
 error: variables can be used directly in the `format!` string
    |
    |
 LL |     println!("val='{local_i32}'", local_i32 = local_i32);
    |
 help: change this to
    |
    |
 LL -     println!("val='{local_i32}'", local_i32 = local_i32);
 LL +     println!("val='{local_i32}'");
 
 
 error: variables can be used directly in the `format!` string
    |
    |
 LL |     println!("val='{local_i32}'", local_i32 = fn_arg);
    |
 help: change this to
    |
    |
 LL -     println!("val='{local_i32}'", local_i32 = fn_arg);
 LL +     println!("val='{fn_arg}'");
 
 
 error: variables can be used directly in the `format!` string
    |
 LL |     println!("{0}", local_i32);
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
 help: change this to
    |
 LL -     println!("{0}", local_i32);
 LL +     println!("{local_i32}");
 
 
 error: variables can be used directly in the `format!` string
    |
 LL |     println!("{0:?}", local_i32);
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
 help: change this to
    |
 LL -     println!("{0:?}", local_i32);
 LL +     println!("{local_i32:?}");
 
 
 error: variables can be used directly in the `format!` string
    |
 LL |     println!("{0:#?}", local_i32);
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
 help: change this to
    |
 LL -     println!("{0:#?}", local_i32);
 LL +     println!("{local_i32:#?}");
 
 
 error: variables can be used directly in the `format!` string
    |
    |
 LL |     println!("{0:04}", local_i32);
    |
 help: change this to
    |
    |
 LL -     println!("{0:04}", local_i32);
 LL +     println!("{local_i32:04}");
 
 
 error: variables can be used directly in the `format!` string
    |
    |
 LL |     println!("{0:<3}", local_i32);
    |
 help: change this to
    |
    |
 LL -     println!("{0:<3}", local_i32);
 LL +     println!("{local_i32:<3}");
 
 
 error: variables can be used directly in the `format!` string
    |
    |
 LL |     println!("{0:#010x}", local_i32);
    |
 help: change this to
    |
    |
 LL -     println!("{0:#010x}", local_i32);
 LL +     println!("{local_i32:#010x}");
 
 
 error: variables can be used directly in the `format!` string
    |
    |
 LL |     println!("{0:.1}", local_f64);
    |
 help: change this to
    |
    |
 LL -     println!("{0:.1}", local_f64);
 LL +     println!("{local_f64:.1}");
 
 
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
 
 
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{1} {} {0} {}", local_i32, local_f64);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{1} {} {0} {}", local_i32, local_f64);
-LL +     println!("{local_f64} {local_i32} {local_i32} {local_f64}");
+error: internal compiler error: unexpected panic
 
 
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{0} {1}", local_i32, local_f64);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{0} {1}", local_i32, local_f64);
-LL +     println!("{local_i32} {local_f64}");
+note: the compiler unexpectedly panicked. this is a bug.
 
 
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{1} {0}", local_i32, local_f64);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{1} {0}", local_i32, local_f64);
-LL +     println!("{local_f64} {local_i32}");
+note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new
 
 
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{1} {0} {1} {0}", local_i32, local_f64);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{1} {0} {1} {0}", local_i32, local_f64);
-LL +     println!("{local_f64} {local_i32} {local_f64} {local_i32}");
-   |
+note: Clippy version: clippy 0.1.68 (842d70d5 2023-01-18)
 
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{v}", v = local_i32);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{v}", v = local_i32);
-LL +     println!("{local_i32}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{local_i32:0$}", width);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{local_i32:0$}", width);
-LL +     println!("{local_i32:width$}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{local_i32:w$}", w = width);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{local_i32:w$}", w = width);
-LL +     println!("{local_i32:width$}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{local_i32:.0$}", prec);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{local_i32:.0$}", prec);
-LL +     println!("{local_i32:.prec$}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{local_i32:.p$}", p = prec);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{local_i32:.p$}", p = prec);
-LL +     println!("{local_i32:.prec$}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{:0$}", v = val);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{:0$}", v = val);
-LL +     println!("{val:val$}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{0:0$}", v = val);
-   |
-help: change this to
-   |
-   |

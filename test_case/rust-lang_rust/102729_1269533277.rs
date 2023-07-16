plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 0152393048c4eb6c6d2aec63e9899cc86a269b44 and 6e89de8c19e545c121fd0cd689c3816e743ba883
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

---- compile_test stdout ----
diff of stderr:

 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:41:5
    |
 LL |     println!("val='{}'", local_i32);
    |
    |
    = note: `-D clippy::uninlined-format-args` implied by `-D warnings`
    |
    |
 LL -     println!("val='{}'", local_i32);
error: test failed, to rerun pass `--test compile-test`
 LL +     println!("val='{local_i32}'");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:42:5
    |
 LL |     println!("val='{   }'", local_i32); // 3 spaces
    |
 help: change this to
    |
    |
 LL -     println!("val='{   }'", local_i32); // 3 spaces
 LL +     println!("val='{local_i32}'"); // 3 spaces
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:43:5
    |
 LL |     println!("val='{    }'", local_i32); // tab
    |
 help: change this to
    |
    |
 LL -     println!("val='{    }'", local_i32); // tab
 LL +     println!("val='{local_i32}'"); // tab
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:44:5
    |
 LL |     println!("val='{     }'", local_i32); // space+tab
    |
 help: change this to
    |
    |
 LL -     println!("val='{     }'", local_i32); // space+tab
 LL +     println!("val='{local_i32}'"); // space+tab
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:45:5
    |
 LL |     println!("val='{     }'", local_i32); // tab+space
    |
 help: change this to
    |
    |
 LL -     println!("val='{     }'", local_i32); // tab+space
 LL +     println!("val='{local_i32}'"); // tab+space
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:46:5
 LL | /     println!(
 LL | /     println!(
 LL | |         "val='{
 LL | |     }'",
 LL | |         local_i32
 LL | |     );
    |
 help: change this to
    |
    |
 LL -         "val='{
 LL +         "val='{local_i32}'"
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:51:5
 LL |     println!("{}", local_i32);
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
    |
 help: change this to
 help: change this to
    |
 LL -     println!("{}", local_i32);
 LL +     println!("{local_i32}");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:52:5
 LL |     println!("{}", fn_arg);
    |     ^^^^^^^^^^^^^^^^^^^^^^
    |
 help: change this to
 help: change this to
    |
 LL -     println!("{}", fn_arg);
 LL +     println!("{fn_arg}");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:53:5
 LL |     println!("{:?}", local_i32);
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
 help: change this to
 help: change this to
    |
 LL -     println!("{:?}", local_i32);
 LL +     println!("{local_i32:?}");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:54:5
 LL |     println!("{:#?}", local_i32);
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
 help: change this to
 help: change this to
    |
 LL -     println!("{:#?}", local_i32);
 LL +     println!("{local_i32:#?}");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:55:5
    |
 LL |     println!("{:4}", local_i32);
    |
 help: change this to
    |
    |
 LL -     println!("{:4}", local_i32);
 LL +     println!("{local_i32:4}");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:56:5
    |
 LL |     println!("{:04}", local_i32);
    |
 help: change this to
    |
    |
 LL -     println!("{:04}", local_i32);
 LL +     println!("{local_i32:04}");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:57:5
    |
 LL |     println!("{:<3}", local_i32);
    |
 help: change this to
    |
    |
 LL -     println!("{:<3}", local_i32);
 LL +     println!("{local_i32:<3}");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:58:5
    |
 LL |     println!("{:#010x}", local_i32);
    |
 help: change this to
    |
    |
 LL -     println!("{:#010x}", local_i32);
 LL +     println!("{local_i32:#010x}");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:59:5
    |
 LL |     println!("{:.1}", local_f64);
    |
 help: change this to
    |
    |
 LL -     println!("{:.1}", local_f64);
 LL +     println!("{local_f64:.1}");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:60:5
    |
 LL |     println!("Hello {} is {:.*}", "x", local_i32, local_f64);
    |
 help: change this to
    |
    |
 LL -     println!("Hello {} is {:.*}", "x", local_i32, local_f64);
 LL +     println!("Hello {} is {local_f64:.local_i32$}", "x");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:61:5
    |
 LL |     println!("Hello {} is {:.*}", local_i32, 5, local_f64);
    |
 help: change this to
    |
    |
 LL -     println!("Hello {} is {:.*}", local_i32, 5, local_f64);
 LL +     println!("Hello {local_i32} is {local_f64:.*}", 5);
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:62:5
    |
 LL |     println!("Hello {} is {2:.*}", local_i32, 5, local_f64);
    |
 help: change this to
    |
    |
 LL -     println!("Hello {} is {2:.*}", local_i32, 5, local_f64);
 LL +     println!("Hello {local_i32} is {local_f64:.*}", 5);
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:63:5
    |
 LL |     println!("{} {}", local_i32, local_f64);
    |
 help: change this to
    |
    |
 LL -     println!("{} {}", local_i32, local_f64);
 LL +     println!("{local_i32} {local_f64}");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:64:5
    |
 LL |     println!("{}, {}", local_i32, local_opt.unwrap());
    |
 help: change this to
    |
    |
 LL -     println!("{}, {}", local_i32, local_opt.unwrap());
 LL +     println!("{local_i32}, {}", local_opt.unwrap());
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:65:5
 LL |     println!("{}", val);
    |     ^^^^^^^^^^^^^^^^^^^
    |
 help: change this to
 help: change this to
    |
 LL -     println!("{}", val);
 LL +     println!("{val}");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:66:5
 LL |     println!("{}", v = val);
    |     ^^^^^^^^^^^^^^^^^^^^^^^
    |
 help: change this to
 help: change this to
    |
 LL -     println!("{}", v = val);
 LL +     println!("{val}");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:68:5
    |
 LL |     println!("val='{/t }'", local_i32);
    |
 help: change this to
    |
    |
 LL -     println!("val='{/t }'", local_i32);
 LL +     println!("val='{local_i32}'");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:69:5
    |
 LL |     println!("val='{/n }'", local_i32);
    |
 help: change this to
    |
    |
 LL -     println!("val='{/n }'", local_i32);
 LL +     println!("val='{local_i32}'");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:70:5
    |
 LL |     println!("val='{local_i32}'", local_i32 = local_i32);
    |
 help: change this to
    |
    |
 LL -     println!("val='{local_i32}'", local_i32 = local_i32);
 LL +     println!("val='{local_i32}'");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:71:5
    |
 LL |     println!("val='{local_i32}'", local_i32 = fn_arg);
    |
 help: change this to
    |
    |
 LL -     println!("val='{local_i32}'", local_i32 = fn_arg);
 LL +     println!("val='{fn_arg}'");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:72:5
 LL |     println!("{0}", local_i32);
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
 help: change this to
 help: change this to
    |
 LL -     println!("{0}", local_i32);
 LL +     println!("{local_i32}");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:73:5
 LL |     println!("{0:?}", local_i32);
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
 help: change this to
 help: change this to
    |
 LL -     println!("{0:?}", local_i32);
 LL +     println!("{local_i32:?}");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:74:5
 LL |     println!("{0:#?}", local_i32);
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
 help: change this to
 help: change this to
    |
 LL -     println!("{0:#?}", local_i32);
 LL +     println!("{local_i32:#?}");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:75:5
    |
 LL |     println!("{0:04}", local_i32);
    |
 help: change this to
    |
    |
 LL -     println!("{0:04}", local_i32);
 LL +     println!("{local_i32:04}");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:76:5
    |
 LL |     println!("{0:<3}", local_i32);
    |
 help: change this to
    |
    |
 LL -     println!("{0:<3}", local_i32);
 LL +     println!("{local_i32:<3}");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:77:5
    |
 LL |     println!("{0:#010x}", local_i32);
    |
 help: change this to
    |
    |
 LL -     println!("{0:#010x}", local_i32);
 LL +     println!("{local_i32:#010x}");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:78:5
    |
 LL |     println!("{0:.1}", local_f64);
    |
 help: change this to
    |
    |
 LL -     println!("{0:.1}", local_f64);
 LL +     println!("{local_f64:.1}");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:79:5
    |
 LL |     println!("{0} {0}", local_i32);
    |
 help: change this to
    |
    |
 LL -     println!("{0} {0}", local_i32);
 LL +     println!("{local_i32} {local_i32}");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:80:5
    |
 LL |     println!("{1} {} {0} {}", local_i32, local_f64);
    |
 help: change this to
    |
    |
 LL -     println!("{1} {} {0} {}", local_i32, local_f64);
 LL +     println!("{local_f64} {local_i32} {local_i32} {local_f64}");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:81:5
    |
 LL |     println!("{0} {1}", local_i32, local_f64);
    |
 help: change this to
    |
    |
 LL -     println!("{0} {1}", local_i32, local_f64);
 LL +     println!("{local_i32} {local_f64}");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:82:5
    |
 LL |     println!("{1} {0}", local_i32, local_f64);
    |
 help: change this to
    |
    |
 LL -     println!("{1} {0}", local_i32, local_f64);
 LL +     println!("{local_f64} {local_i32}");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:83:5
    |
 LL |     println!("{1} {0} {1} {0}", local_i32, local_f64);
    |
 help: change this to
    |
    |
 LL -     println!("{1} {0} {1} {0}", local_i32, local_f64);
 LL +     println!("{local_f64} {local_i32} {local_f64} {local_i32}");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:85:5
    |
 LL |     println!("{v}", v = local_i32);
    |
 help: change this to
    |
    |
 LL -     println!("{v}", v = local_i32);
 LL +     println!("{local_i32}");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:86:5
    |
 LL |     println!("{local_i32:0$}", width);
    |
 help: change this to
    |
    |
 LL -     println!("{local_i32:0$}", width);
 LL +     println!("{local_i32:width$}");
 
 
 error: variables can be used directly in the `format!` string
   --> $DIR/uninlined_format_args.rs:87:5
    |
 LL |     println!("{local_i32:w$}", w = width);
    |
 help: change this to
    |
    |
 LL -     println!("{local_i32:w$}", w = width);
 LL +     println!("{local_i32:width$}");

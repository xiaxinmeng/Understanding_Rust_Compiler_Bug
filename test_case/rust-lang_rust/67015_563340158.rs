
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-11]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
  left: `[["z\u{82}$", ""], ["w\'", ">R"], ["쭄爾", "ŉ\u{85}\u{8}*"], ["=", ""]]`,
 right: `[["\u{feff}z\u{82}$", ""], ["w\'", ">R"], ["쭄爾", "ŉ\u{85}\u{8}*"], ["=", ""]]`', tests/test_cat.rs:86:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

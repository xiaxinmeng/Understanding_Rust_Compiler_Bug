
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-499]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
thread 'test_cat::prop_cat_cols' panicked at '[quickcheck] TEST FAILED (runtime error). Arguments: (CsvData { data: [[[]]] }, CsvData { data: [[[239, 187, 191]]] })
Error: "assertion failed: `(left == right)`\n  left: `[]`,\n right: `[[\"\", \"\\u{feff}\"]]`"', /checkout/src/libstd/macros.rs:13:23


failures:
    test_cat::prop_cat_cols

test result: FAILED. 421 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargotest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/cargo" "/checkout/obj/build/ct"
expected success, got: exit code: 101


thread 'test_cat::prop_cat_cols' panicked at '[quickcheck] TEST FAILED (runtime error). Arguments: (CsvData { data: [[[]]] }, CsvData { data: [[[239, 187, 191]]] })
Error: "assertion failed: `(left == right)`\n  left: `[]`,\n right: `[[\"\", \"\\u{feff}\"]]`"', C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\quickcheck-0.4.1\src\tester.rs:147:28

failures:
    test_cat::prop_cat_cols

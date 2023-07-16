
[01:54:53] thread 'test_cat::prop_cat_cols' panicked at '[quickcheck] TEST FAILED (runtime error). Arguments: (CsvData { data: [[[]]] }, CsvData { data: [[[239, 187, 191]]] })
[01:54:53] Error: "assertion failed: `(left == right)`\n  left: `[]`,\n right: `[[\"\", \"\\u{feff}\"]]`"', /cargo/registry/src/github.com-1ecc6299db9ec823/quickcheck-0.4.1/src/tester.rs:147:28
[01:54:53]
[01:54:53]
[01:54:53] failures:
[01:54:53]     test_cat::prop_cat_cols
[01:54:53]
[01:54:53] test result: FAILED. 421 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:54:53]
[01:54:53]
[01:54:53] error: test failed, to rerun pass '--test tests'
[01:54:53] thread 'main' panicked at 'tests failed for https://github.com/BurntSushi/xsv', tools/cargotest/main.rs:100:9

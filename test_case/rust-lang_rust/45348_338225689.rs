
[01:08:09] failures:
[01:08:09] 
[01:08:09] ---- test_frequency::prop_frequency stdout ----
[01:08:09] 	[C:\projects\rust\build\ct\xsv\target\debug\xit\prop_frequency\test-821]: "C:\\projects\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "frequency" "in.csv" "-j" "4" "--limit" "0"
[01:08:09] [C:\projects\rust\build\ct\xsv\target\debug\xit\prop_frequency\test-823]: "C:\\projects\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "frequency" "in.csv" "-j" "4" "--limit" "0"
...
[01:08:09] [C:\projects\rust\build\ct\xsv\target\debug\xit\prop_frequency\test-957]: "C:\\projects\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "frequency" "in.csv" "-j" "4" "--limit" "0"
[01:08:09] thread 'test_frequency::prop_frequency' panicked at '[quickcheck] TEST FAILED (runtime error). Arguments: (CsvData { data: [[[239, 187, 191]], [[]]] })
[01:08:09] Error: "called `Option::unwrap()` on a `None` value"', C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\quickcheck-0.4.1\src\tester.rs:147:27
[01:08:09] 
[01:08:09] 
[01:08:09] failures:
[01:08:09]     test_frequency::prop_frequency
[01:08:09] 
[01:08:09] test result: FAILED. 420 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:08:09] 
[01:08:09] error: test failed, to rerun pass '--test tests'

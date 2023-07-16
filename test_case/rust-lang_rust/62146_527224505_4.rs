bash
$ cargo test
...
  = note: /usr/bin/ld: /home/ahuszagh/git/rust-proc-macro-test-linking-error/target/debug/deps/le_proc_macro-8dbd2ca2e9b83841.ltp2306el5q7003.rcgu.o: in function `test::assert_test_result':
          /home/ahuszagh/git/rust-proc-macro-test-linking-error/<::core::macros::assert_eq macros>:(.text._ZN4test18assert_test_result17had0255f09aef5585E+0x113): undefined reference to `<&T as core::fmt::Debug>::fmt'
          /usr/bin/ld: /home/ahuszagh/git/rust-proc-macro-test-linking-error/target/debug/deps/le_proc_macro-8dbd2ca2e9b83841.ltp2306el5q7003.rcgu.o: in function `test::assert_test_result':
          /home/ahuszagh/git/rust-proc-macro-test-linking-error/<::std::macros::panic macros>:(.text._ZN4test18assert_test_result17had0255f09aef5585E+0x18e): undefined reference to `<&T as core::fmt::Debug>::fmt'
          collect2: error: ld returned 1 exit status

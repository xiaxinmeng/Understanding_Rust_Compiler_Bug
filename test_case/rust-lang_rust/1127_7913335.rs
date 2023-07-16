
$ cat ./test_compile_error_example.rs
fn inc_int(x: int) -> int { x + 1 }


#[test]
fn test_inc_int() {
  assert (inc_int(3) == 4);
}

$ rustc --test ./test_compile_error_example.rs
./test_compile_error_example.rs:1:0: 1:0 error: unresolved name
./test_compile_error_example.rs:1 fn inc_int(x: int) -> int { x + 1 }
                                  ^
./test_compile_error_example.rs:1:0: 1:0 error: use of undeclared module `std::test`
./test_compile_error_example.rs:1 fn inc_int(x: int) -> int { x + 1 }
                                  ^
./test_compile_error_example.rs:1:0: 1:0 error: unresolved name: std::test::test_main
./test_compile_error_example.rs:1 fn inc_int(x: int) -> int { x + 1 }
                                  ^
./test_compile_error_example.rs:1:0: 1:0 error: unresolved name
./test_compile_error_example.rs:1 fn inc_int(x: int) -> int { x + 1 }
                                  ^
./test_compile_error_example.rs:1:0: 1:0 error: use of undeclared module `std::test`
./test_compile_error_example.rs:1 fn inc_int(x: int) -> int { x + 1 }
                                  ^
./test_compile_error_example.rs:1:0: 1:0 error: use of undeclared type name `std::test::test_desc`
./test_compile_error_example.rs:1 fn inc_int(x: int) -> int { x + 1 }
                                  ^
error: aborting due to 6 previous errors

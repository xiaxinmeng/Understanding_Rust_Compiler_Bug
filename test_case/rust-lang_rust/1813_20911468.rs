
#[cfg(test)] fn actual_test_1() { ... }
#[cfg(test)] fn actual_test_2() { ... }
...
#[cfg(test)] fn actual_test_N() { ... }

#[test]
fn run_all_tests() {
    actual_test_1();
    actual_test_2();
    ...
    actual_test_N();
}

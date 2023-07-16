plain

running 658 tests
.................................................................................................... 100/658
.................................................................................................... 200/658
...........................................F..FFFF.FF.FFFF.F.F.....F................................ 300/658
.F.............F.................................................................................... 400/658
.................................................................................................... 600/658
..........................................................
failures:


---- str::slice_index::boundary::range_1::index_mut_fail stdout ----
thread 'str::slice_index::boundary::range_1::index_mut_fail' panicked at 'byte index 4 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`', library/alloc/tests/str.rs:653:9
note: panic did not contain expected string
      panic message: `"byte index 4 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`"`,
 expected substring: `"byte index 4 is not a char boundary; it is inside 'α' (bytes 3..5) of"`
---- str::slice_index::boundary::range_2::index_fail stdout ----
thread 'str::slice_index::boundary::range_2::index_fail' panicked at 'byte index 6 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`', library/alloc/tests/str.rs:653:9
note: panic did not contain expected string
      panic message: `"byte index 6 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`"`,
 expected substring: `"byte index 6 is not a char boundary; it is inside 'β' (bytes 5..7) of"`
---- str::slice_index::boundary::rangefrom::index_fail stdout ----
thread 'str::slice_index::boundary::rangefrom::index_fail' panicked at 'byte index 4 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`', library/alloc/tests/str.rs:653:9
error: test failed, to rerun pass '-p alloc --test collectionstests'
note: panic did not contain expected string
      panic message: `"byte index 4 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`"`,
 expected substring: `"byte index 4 is not a char boundary; it is inside 'α' (bytes 3..5) of"`
---- str::slice_index::boundary::range_1::index_fail stdout ----
thread 'str::slice_index::boundary::range_1::index_fail' panicked at 'byte index 4 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`', library/alloc/tests/str.rs:653:9
note: panic did not contain expected string
      panic message: `"byte index 4 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`"`,
 expected substring: `"byte index 4 is not a char boundary; it is inside 'α' (bytes 3..5) of"`
---- str::slice_index::boundary::range_2::index_mut_fail stdout ----
thread 'str::slice_index::boundary::range_2::index_mut_fail' panicked at 'byte index 6 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`', library/alloc/tests/str.rs:653:9
note: panic did not contain expected string
      panic message: `"byte index 6 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`"`,
 expected substring: `"byte index 6 is not a char boundary; it is inside 'β' (bytes 5..7) of"`
---- str::slice_index::boundary::rangefrom::index_mut_fail stdout ----
thread 'str::slice_index::boundary::rangefrom::index_mut_fail' panicked at 'byte index 4 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`', library/alloc/tests/str.rs:653:9
note: panic did not contain expected string
      panic message: `"byte index 4 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`"`,
 expected substring: `"byte index 4 is not a char boundary; it is inside 'α' (bytes 3..5) of"`
---- str::slice_index::boundary::rangeinclusive_1::index_mut_fail stdout ----
thread 'str::slice_index::boundary::rangeinclusive_1::index_mut_fail' panicked at 'byte index 4 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`', library/alloc/tests/str.rs:653:9
note: panic did not contain expected string
      panic message: `"byte index 4 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`"`,
 expected substring: `"byte index 4 is not a char boundary; it is inside 'α' (bytes 3..5) of"`
---- str::slice_index::boundary::rangeinclusive_1::index_fail stdout ----
thread 'str::slice_index::boundary::rangeinclusive_1::index_fail' panicked at 'byte index 4 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`', library/alloc/tests/str.rs:653:9
note: panic did not contain expected string
      panic message: `"byte index 4 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`"`,
 expected substring: `"byte index 4 is not a char boundary; it is inside 'α' (bytes 3..5) of"`
---- str::slice_index::boundary::rangeinclusive_2::index_fail stdout ----
thread 'str::slice_index::boundary::rangeinclusive_2::index_fail' panicked at 'byte index 6 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`', library/alloc/tests/str.rs:653:9
note: panic did not contain expected string
      panic message: `"byte index 6 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`"`,
 expected substring: `"byte index 6 is not a char boundary; it is inside 'β' (bytes 5..7) of"`
---- str::slice_index::boundary::rangeinclusive_2::index_mut_fail stdout ----
thread 'str::slice_index::boundary::rangeinclusive_2::index_mut_fail' panicked at 'byte index 6 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`', library/alloc/tests/str.rs:653:9
note: panic did not contain expected string
      panic message: `"byte index 6 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`"`,
 expected substring: `"byte index 6 is not a char boundary; it is inside 'β' (bytes 5..7) of"`
---- str::slice_index::boundary::rangeto::index_fail stdout ----
thread 'str::slice_index::boundary::rangeto::index_fail' panicked at 'byte index 6 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`', library/alloc/tests/str.rs:653:9
note: panic did not contain expected string
      panic message: `"byte index 6 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`"`,
 expected substring: `"byte index 6 is not a char boundary; it is inside 'β' (bytes 5..7) of"`
---- str::slice_index::boundary::rangeto::index_mut_fail stdout ----
thread 'str::slice_index::boundary::rangeto::index_mut_fail' panicked at 'byte index 6 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`', library/alloc/tests/str.rs:653:9
note: panic did not contain expected string
      panic message: `"byte index 6 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`"`,
 expected substring: `"byte index 6 is not a char boundary; it is inside 'β' (bytes 5..7) of"`
---- str::slice_index::boundary::rangetoinclusive::index_fail stdout ----
thread 'str::slice_index::boundary::rangetoinclusive::index_fail' panicked at 'byte index 6 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`', library/alloc/tests/str.rs:653:9
note: panic did not contain expected string
      panic message: `"byte index 6 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`"`,
 expected substring: `"byte index 6 is not a char boundary; it is inside 'β' (bytes 5..7) of"`
---- str::slice_index::boundary::rangetoinclusive::index_mut_fail stdout ----
thread 'str::slice_index::boundary::rangetoinclusive::index_mut_fail' panicked at 'byte index 6 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`', library/alloc/tests/str.rs:653:9
note: panic did not contain expected string
      panic message: `"byte index 6 is not a char boundary; it is inside 'c' (bytes 2..3) of `abcαβγ`"`,
 expected substring: `"byte index 6 is not a char boundary; it is inside 'β' (bytes 5..7) of"`
---- str::slice_index::test_slice_fail_truncated_2 stdout ----
thread 'str::slice_index::test_slice_fail_truncated_2' panicked at 'byte index 1024 is out of bounds of `Lor`[...]', library/alloc/tests/str.rs:723:18
note: panic did not contain expected string
      panic message: `"byte index 1024 is out of bounds of `Lor`[...]"`,
 expected substring: `"luctus, im`[...]"`
---- str::slice_index::test_slice_fail_truncated_1 stdout ----
thread 'str::slice_index::test_slice_fail_truncated_1' panicked at 'byte index 1024 is out of bounds of `Lor`[...]', library/alloc/tests/str.rs:717:18
note: panic did not contain expected string
      panic message: `"byte index 1024 is out of bounds of `Lor`[...]"`,
 expected substring: `"byte index 1024 is out of bounds of `Lorem ipsum dolor sit amet"`
failures:
    str::slice_index::boundary::range_1::index_fail
    str::slice_index::boundary::range_1::index_mut_fail
    str::slice_index::boundary::range_2::index_fail
---
test result: FAILED. 642 passed; 16 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.51s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:03

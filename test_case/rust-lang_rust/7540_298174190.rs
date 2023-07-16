
touch src/librustdoc/lib.rs
./x.py test --stage 1 src/test/compile-fail --test-args xcrate-unit-struct
   => runs xcrate-unit-struct test
./x.py test --stage 1 src/test/compile-fail --test-args xcrate-private-by-
   => runs xcrate-private-by-default test
./x.py test --stage 1 src/test/compile-fail
   => ignores the two already-run tests (private-by-default and unit-struct).

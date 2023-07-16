
warning: constant `foo` should have an upper case name
  --> $DIR/test-inner-fn.rs:14:4
   |
LL | fn foo() {
   |    ^^^ help: convert the identifier to upper case: `FOO`
   |
   = note: #[warn(non_upper_case_globals)] on by default

warning: constant `bar` should have an upper case name
  --> $DIR/test-inner-fn.rs:16:8
   |
LL |     fn bar() {}
   |        ^^^ help: convert the identifier to upper case: `BAR`

error: cannot test inner items
  --> $DIR/test-inner-fn.rs:15:5
   |
LL |     #[test] //~ ERROR cannot test inner items [unnameable_test_items]
   |     ^^^^^^^
   |
   = note: requested on the command line with `-D unnameable-test-items`

warning: constant `foo` should have an upper case name
  --> $DIR/test-inner-fn.rs:22:8
   |
LL |     fn foo() {
   |        ^^^ help: convert the identifier to upper case: `FOO`

warning: constant `bar` should have an upper case name
  --> $DIR/test-inner-fn.rs:24:12
   |
LL |         fn bar() {}
   |            ^^^ help: convert the identifier to upper case: `BAR`

error: cannot test inner items
  --> $DIR/test-inner-fn.rs:23:9
   |
LL |         #[test] //~ ERROR cannot test inner items [unnameable_test_items]
   |         ^^^^^^^

error: aborting due to 2 previous errors


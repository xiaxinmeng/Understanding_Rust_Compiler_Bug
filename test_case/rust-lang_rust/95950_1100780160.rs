plain
15 note: function defined here
-   --> $DIR/issue-54505-no-std.rs:23:4
+   --> $DIR/issue-54505-no-std.rs:20:4
17    |
18 LL | fn take_range(_r: &impl RangeBounds<i8>) {}

31    = note: expected reference `&_`
32                  found struct `RangeFrom<{integer}>`
33 note: function defined here
33 note: function defined here
-   --> $DIR/issue-54505-no-std.rs:23:4
+   --> $DIR/issue-54505-no-std.rs:20:4
35    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
36 LL | fn take_range(_r: &impl RangeBounds<i8>) {}

49    = note: expected reference `&_`
50                  found struct `RangeFull`
51 note: function defined here
51 note: function defined here
-   --> $DIR/issue-54505-no-std.rs:23:4
+   --> $DIR/issue-54505-no-std.rs:20:4
53    |
54 LL | fn take_range(_r: &impl RangeBounds<i8>) {}

67    = note: expected reference `&_`
68                  found struct `RangeInclusive<{integer}>`
69 note: function defined here
69 note: function defined here
-   --> $DIR/issue-54505-no-std.rs:23:4
+   --> $DIR/issue-54505-no-std.rs:20:4
71    |
72 LL | fn take_range(_r: &impl RangeBounds<i8>) {}

85    = note: expected reference `&_`
86                  found struct `RangeTo<{integer}>`
87 note: function defined here
87 note: function defined here
-   --> $DIR/issue-54505-no-std.rs:23:4
+   --> $DIR/issue-54505-no-std.rs:20:4
89    |
90 LL | fn take_range(_r: &impl RangeBounds<i8>) {}

103    = note: expected reference `&_`
104                  found struct `RangeToInclusive<{integer}>`
105 note: function defined here
105 note: function defined here
-   --> $DIR/issue-54505-no-std.rs:23:4
+   --> $DIR/issue-54505-no-std.rs:20:4
107    |
108 LL | fn take_range(_r: &impl RangeBounds<i8>) {}


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/issue-54505-no-std/issue-54505-no-std.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/issue-54505-no-std/issue-54505-no-std.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args range/issue-54505-no-std.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/range/issue-54505-no-std.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/issue-54505-no-std" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/issue-54505-no-std/auxiliary"
stdout: none
--- stderr -------------------------------
error: `#[panic_handler]` function required, but not found
error[E0308]: mismatched types
  --> /checkout/src/test/ui/range/issue-54505-no-std.rs:24:16
   |
   |
LL |     take_range(0..1);
   |     ---------- ^^^^
   |     |          expected reference, found struct `Range`
   |     |          help: consider borrowing here: `&(0..1)`
   |     arguments to this function are incorrect
   |
   |
   = note: expected reference `&_`
                 found struct `Range<{integer}>`
note: function defined here
  --> /checkout/src/test/ui/range/issue-54505-no-std.rs:20:4
   |
LL | fn take_range(_r: &impl RangeBounds<i8>) {}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/range/issue-54505-no-std.rs:29:16
   |
   |
LL |     take_range(1..);
   |     ---------- ^^^
   |     |          expected reference, found struct `RangeFrom`
   |     |          help: consider borrowing here: `&(1..)`
   |     arguments to this function are incorrect
   |
   |
   = note: expected reference `&_`
                 found struct `RangeFrom<{integer}>`
note: function defined here
  --> /checkout/src/test/ui/range/issue-54505-no-std.rs:20:4
   |
LL | fn take_range(_r: &impl RangeBounds<i8>) {}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/range/issue-54505-no-std.rs:34:16
   |
   |
LL |     take_range(..);
   |     ---------- ^^
   |     |          expected reference, found struct `RangeFull`
   |     |          help: consider borrowing here: `&(..)`
   |     arguments to this function are incorrect
   |
   |
   = note: expected reference `&_`
                 found struct `RangeFull`
note: function defined here
  --> /checkout/src/test/ui/range/issue-54505-no-std.rs:20:4
   |
LL | fn take_range(_r: &impl RangeBounds<i8>) {}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/range/issue-54505-no-std.rs:39:16
   |
   |
LL |     take_range(0..=1);
   |     ---------- ^^^^^
   |     |          expected reference, found struct `RangeInclusive`
   |     |          expected reference, found struct `RangeInclusive`
   |     |          help: consider borrowing here: `&(0..=1)`
   |     arguments to this function are incorrect
   = note: expected reference `&_`
                 found struct `RangeInclusive<{integer}>`
note: function defined here
  --> /checkout/src/test/ui/range/issue-54505-no-std.rs:20:4
  --> /checkout/src/test/ui/range/issue-54505-no-std.rs:20:4
   |
LL | fn take_range(_r: &impl RangeBounds<i8>) {}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/range/issue-54505-no-std.rs:44:16
   |
   |
LL |     take_range(..5);
   |     ---------- ^^^
   |     |          expected reference, found struct `RangeTo`
   |     |          help: consider borrowing here: `&(..5)`
   |     arguments to this function are incorrect
   |
   |
   = note: expected reference `&_`
                 found struct `RangeTo<{integer}>`
note: function defined here
  --> /checkout/src/test/ui/range/issue-54505-no-std.rs:20:4
   |
LL | fn take_range(_r: &impl RangeBounds<i8>) {}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/range/issue-54505-no-std.rs:49:16
   |
   |
LL |     take_range(..=42);
   |     ---------- ^^^^^
   |     |          expected reference, found struct `RangeToInclusive`
   |     |          expected reference, found struct `RangeToInclusive`
   |     |          help: consider borrowing here: `&(..=42)`
   |     arguments to this function are incorrect
   = note: expected reference `&_`
                 found struct `RangeToInclusive<{integer}>`
note: function defined here
  --> /checkout/src/test/ui/range/issue-54505-no-std.rs:20:4
  --> /checkout/src/test/ui/range/issue-54505-no-std.rs:20:4
   |
LL | fn take_range(_r: &impl RangeBounds<i8>) {}

error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0308`.

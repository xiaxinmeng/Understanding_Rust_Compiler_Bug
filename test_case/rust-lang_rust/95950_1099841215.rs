plain

---- [ui] src/test/ui/macros/macro-comma-behavior.rs#core stdout ----
diff of stderr:

1 error: 1 positional argument in format string, but no arguments were given
-   --> $DIR/macro-comma-behavior.rs:21:23
3    |
3    |
4 LL |     assert_eq!(1, 1, "{}",);

6 
6 
7 error: 1 positional argument in format string, but no arguments were given
-   --> $DIR/macro-comma-behavior.rs:24:23
9    |
9    |
10 LL |     assert_ne!(1, 2, "{}",);

12 
12 
13 error: 1 positional argument in format string, but no arguments were given
-   --> $DIR/macro-comma-behavior.rs:30:29
15    |
15    |
16 LL |     debug_assert_eq!(1, 1, "{}",);

18 
18 
19 error: 1 positional argument in format string, but no arguments were given
-   --> $DIR/macro-comma-behavior.rs:33:29
21    |
21    |
22 LL |     debug_assert_ne!(1, 2, "{}",);

24 
24 
25 error: 1 positional argument in format string, but no arguments were given
-   --> $DIR/macro-comma-behavior.rs:52:19
27    |
28 LL |     format_args!("{}",);
29    |                   ^^


30 
31 error: 1 positional argument in format string, but no arguments were given
-   --> $DIR/macro-comma-behavior.rs:68:21
33    |
33    |
34 LL |     unimplemented!("{}",);
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

36 
36 
37 error: 1 positional argument in format string, but no arguments were given
-   --> $DIR/macro-comma-behavior.rs:77:24
39    |
40 LL |             write!(f, "{}",)?;
41    |                        ^^


42 
43 error: 1 positional argument in format string, but no arguments were given
-   --> $DIR/macro-comma-behavior.rs:81:26
45    |
46 LL |             writeln!(f, "{}",)?;
47    |                          ^^



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior.core/macro-comma-behavior.core.stderr
To only update this specific test, also pass `--test-args macros/macro-comma-behavior.rs`


error in revision `core`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-comma-behavior.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "core" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior.core" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior.core/auxiliary"
stdout: none
--- stderr -------------------------------
error: 1 positional argument in format string, but no arguments were given
   |
   |
LL |     assert_eq!(1, 1, "{}",);


error: 1 positional argument in format string, but no arguments were given
   |
   |
LL |     assert_ne!(1, 2, "{}",);


error: 1 positional argument in format string, but no arguments were given
   |
   |
LL |     debug_assert_eq!(1, 1, "{}",);


error: 1 positional argument in format string, but no arguments were given
   |
   |
LL |     debug_assert_ne!(1, 2, "{}",);


error: 1 positional argument in format string, but no arguments were given
   |
LL |     format_args!("{}",);
   |                   ^^


error: 1 positional argument in format string, but no arguments were given
   |
   |
LL |     unimplemented!("{}",);


error: 1 positional argument in format string, but no arguments were given
   |
LL |             write!(f, "{}",)?;
   |                        ^^


error: 1 positional argument in format string, but no arguments were given
   |
LL |             writeln!(f, "{}",)?;
   |                          ^^


error: aborting due to 8 previous errors
------------------------------------------


---- [ui] src/test/ui/macros/macro-comma-behavior.rs#std stdout ----
diff of stderr:

1 error: 1 positional argument in format string, but no arguments were given
-   --> $DIR/macro-comma-behavior.rs:21:23
3    |
3    |
4 LL |     assert_eq!(1, 1, "{}",);

6 
6 
7 error: 1 positional argument in format string, but no arguments were given
-   --> $DIR/macro-comma-behavior.rs:24:23
9    |
9    |
10 LL |     assert_ne!(1, 2, "{}",);

12 
12 
13 error: 1 positional argument in format string, but no arguments were given
-   --> $DIR/macro-comma-behavior.rs:30:29
15    |
15    |
16 LL |     debug_assert_eq!(1, 1, "{}",);

18 
18 
19 error: 1 positional argument in format string, but no arguments were given
-   --> $DIR/macro-comma-behavior.rs:33:29
21    |
21    |
22 LL |     debug_assert_ne!(1, 2, "{}",);

24 
24 
25 error: 1 positional argument in format string, but no arguments were given
-   --> $DIR/macro-comma-behavior.rs:38:18
27    |
27    |
28 LL |         eprint!("{}",);

30 
30 
31 error: 1 positional argument in format string, but no arguments were given
-   --> $DIR/macro-comma-behavior.rs:43:20
33    |
34 LL |         eprintln!("{}",);
35    |                    ^^


36 
37 error: 1 positional argument in format string, but no arguments were given
-   --> $DIR/macro-comma-behavior.rs:48:18
39    |
40 LL |         format!("{}",);
41    |                  ^^


42 
43 error: 1 positional argument in format string, but no arguments were given
-   --> $DIR/macro-comma-behavior.rs:52:19
45    |
46 LL |     format_args!("{}",);
47    |                   ^^


48 
49 error: 1 positional argument in format string, but no arguments were given
-   --> $DIR/macro-comma-behavior.rs:59:17
51    |
52 LL |         print!("{}",);
53    |                 ^^


54 
55 error: 1 positional argument in format string, but no arguments were given
-   --> $DIR/macro-comma-behavior.rs:64:19
57    |
58 LL |         println!("{}",);
59    |                   ^^


60 
61 error: 1 positional argument in format string, but no arguments were given
-   --> $DIR/macro-comma-behavior.rs:68:21
63    |
63    |
64 LL |     unimplemented!("{}",);

66 
66 
67 error: 1 positional argument in format string, but no arguments were given
-   --> $DIR/macro-comma-behavior.rs:77:24
69    |
70 LL |             write!(f, "{}",)?;
71    |                        ^^


72 
73 error: 1 positional argument in format string, but no arguments were given
-   --> $DIR/macro-comma-behavior.rs:81:26
75    |
76 LL |             writeln!(f, "{}",)?;
77    |                          ^^



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior.std/macro-comma-behavior.std.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/macro-comma-behavior.rs`

error in revision `std`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-comma-behavior.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "std" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior.std" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior.std/auxiliary"
stdout: none
--- stderr -------------------------------
error: 1 positional argument in format string, but no arguments were given
   |
   |
LL |     assert_eq!(1, 1, "{}",);


error: 1 positional argument in format string, but no arguments were given
   |
   |
LL |     assert_ne!(1, 2, "{}",);


error: 1 positional argument in format string, but no arguments were given
   |
   |
LL |     debug_assert_eq!(1, 1, "{}",);


error: 1 positional argument in format string, but no arguments were given
   |
   |
LL |     debug_assert_ne!(1, 2, "{}",);


error: 1 positional argument in format string, but no arguments were given
   |
   |
LL |         eprint!("{}",);


error: 1 positional argument in format string, but no arguments were given
   |
LL |         eprintln!("{}",);
   |                    ^^


error: 1 positional argument in format string, but no arguments were given
   |
LL |         format!("{}",);
   |                  ^^


error: 1 positional argument in format string, but no arguments were given
   |
LL |     format_args!("{}",);
   |                   ^^


error: 1 positional argument in format string, but no arguments were given
   |
LL |         print!("{}",);
   |                 ^^


error: 1 positional argument in format string, but no arguments were given
   |
LL |         println!("{}",);
   |                   ^^


error: 1 positional argument in format string, but no arguments were given
   |
   |
LL |     unimplemented!("{}",);


error: 1 positional argument in format string, but no arguments were given
   |
LL |             write!(f, "{}",)?;
   |                        ^^


error: 1 positional argument in format string, but no arguments were given
   |
LL |             writeln!(f, "{}",)?;
   |                          ^^


error: aborting due to 13 previous errors
------------------------------------------


---- [ui] src/test/ui/range/issue-54505-no-std.rs stdout ----
diff of stderr:

1 error: `#[panic_handler]` function required, but not found
3 error[E0308]: mismatched types
-   --> $DIR/issue-54505-no-std.rs:27:16
+   --> $DIR/issue-54505-no-std.rs:24:16
5    |
5    |
6 LL |     take_range(0..1);

13                  found struct `Range<{integer}>`
14 
15 error[E0308]: mismatched types
15 error[E0308]: mismatched types
-   --> $DIR/issue-54505-no-std.rs:32:16
+   --> $DIR/issue-54505-no-std.rs:29:16
17    |
18 LL |     take_range(1..);

25                  found struct `RangeFrom<{integer}>`
26 
27 error[E0308]: mismatched types
27 error[E0308]: mismatched types
-   --> $DIR/issue-54505-no-std.rs:37:16
+   --> $DIR/issue-54505-no-std.rs:34:16
29    |
30 LL |     take_range(..);

37                  found struct `RangeFull`
38 
39 error[E0308]: mismatched types
39 error[E0308]: mismatched types
-   --> $DIR/issue-54505-no-std.rs:42:16
+   --> $DIR/issue-54505-no-std.rs:39:16
41    |
42 LL |     take_range(0..=1);

49                  found struct `RangeInclusive<{integer}>`
50 
51 error[E0308]: mismatched types
51 error[E0308]: mismatched types
-   --> $DIR/issue-54505-no-std.rs:47:16
+   --> $DIR/issue-54505-no-std.rs:44:16
53    |
54 LL |     take_range(..5);

61                  found struct `RangeTo<{integer}>`
62 
63 error[E0308]: mismatched types
63 error[E0308]: mismatched types
-   --> $DIR/issue-54505-no-std.rs:52:16
+   --> $DIR/issue-54505-no-std.rs:49:16
65    |
66 LL |     take_range(..=42);


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
   |                |
   |                expected reference, found struct `Range`
   |                help: consider borrowing here: `&(0..1)`
   |
   |
   = note: expected reference `&_`
                 found struct `Range<{integer}>`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/range/issue-54505-no-std.rs:29:16
   |
LL |     take_range(1..);
   |                |
   |                expected reference, found struct `RangeFrom`
   |                help: consider borrowing here: `&(1..)`
   |
   |
   = note: expected reference `&_`
                 found struct `RangeFrom<{integer}>`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/range/issue-54505-no-std.rs:34:16
   |
LL |     take_range(..);
   |                |
   |                expected reference, found struct `RangeFull`
   |                help: consider borrowing here: `&(..)`
   |
   |
   = note: expected reference `&_`
                 found struct `RangeFull`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/range/issue-54505-no-std.rs:39:16
   |
LL |     take_range(0..=1);
   |                |
   |                expected reference, found struct `RangeInclusive`
   |                expected reference, found struct `RangeInclusive`
   |                help: consider borrowing here: `&(0..=1)`
   = note: expected reference `&_`
                 found struct `RangeInclusive<{integer}>`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/range/issue-54505-no-std.rs:44:16
   |
LL |     take_range(..5);
   |                |
   |                expected reference, found struct `RangeTo`
   |                help: consider borrowing here: `&(..5)`
   |
   |
   = note: expected reference `&_`
                 found struct `RangeTo<{integer}>`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/range/issue-54505-no-std.rs:49:16
   |
LL |     take_range(..=42);
   |                |
   |                expected reference, found struct `RangeToInclusive`
   |                expected reference, found struct `RangeToInclusive`
   |                help: consider borrowing here: `&(..=42)`
   = note: expected reference `&_`
                 found struct `RangeToInclusive<{integer}>`

error: aborting due to 7 previous errors

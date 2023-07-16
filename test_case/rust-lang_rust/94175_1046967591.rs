plain
.......................................................i.ii......................................... 12600/12660
............................................................
failures:

---- [ui] ui/check-cfg/mix.rs stdout ----


1 warning: unexpected `cfg` condition name
-   --> $DIR/mix.rs:10:7
+   --> $DIR/mix.rs:11:7
3    |
4 LL | #[cfg(widnows)]
5    |       ^^^^^^^ help: did you mean: `windows`
7    = note: `#[warn(unexpected_cfgs)]` on by default
8 
9 warning: unexpected `cfg` condition value
-   --> $DIR/mix.rs:20:7
-   --> $DIR/mix.rs:20:7
+   --> $DIR/mix.rs:21:7
11    |
12 LL | #[cfg(feature = "zebra")]


15    = note: possible values for `feature` are: bar, foo
16 
17 warning: unexpected `cfg` condition name
-   --> $DIR/mix.rs:24:12
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+   --> $DIR/mix.rs:25:12
19    |
20 LL | #[cfg_attr(uu, test)]

22 
22 
23 warning: unexpected `cfg` condition name
-   --> $DIR/mix.rs:33:10
+   --> $DIR/mix.rs:34:10
25    |
26 LL |     cfg!(widnows);
27    |          ^^^^^^^ help: did you mean: `windows`
28 
29 warning: unexpected `cfg` condition value
-   --> $DIR/mix.rs:37:10
+   --> $DIR/mix.rs:38:10
+   --> $DIR/mix.rs:38:10
31    |
32 LL |     cfg!(feature = "zebra");


35    = note: possible values for `feature` are: bar, foo
36 
37 warning: unexpected `cfg` condition name
-   --> $DIR/mix.rs:39:10
+   --> $DIR/mix.rs:40:10
39    |
40 LL |     cfg!(xxx = "foo");

42 
42 
43 warning: unexpected `cfg` condition name
-   --> $DIR/mix.rs:41:10
+   --> $DIR/mix.rs:42:10
45    |
46 LL |     cfg!(xxx);

48 
48 
49 warning: unexpected `cfg` condition name
-   --> $DIR/mix.rs:43:23
+   --> $DIR/mix.rs:44:23
51    |
52 LL |     cfg!(any(windows, xxx));

54 
54 
55 warning: unexpected `cfg` condition name
-   --> $DIR/mix.rs:45:14
+   --> $DIR/mix.rs:46:14
57    |
58 LL |     cfg!(any(xxx, windows));

60 
61 warning: unexpected `cfg` condition value
-   --> $DIR/mix.rs:47:14
-   --> $DIR/mix.rs:47:14
+   --> $DIR/mix.rs:48:14
63    |
64 LL |     cfg!(any(feature = "bad", windows));


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/mix/mix.stderr
To only update this specific test, also pass `--test-args check-cfg/mix.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/check-cfg/mix.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/mix" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--check-cfg=names()" "--check-cfg=values(feature,\"foo\")" "--cfg" "feature=\"bar\"" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/mix/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:11:7
   |
LL | #[cfg(widnows)]
   |       ^^^^^^^ help: did you mean: `windows`
   = note: `#[warn(unexpected_cfgs)]` on by default

warning: unexpected `cfg` condition value
  --> /checkout/src/test/ui/check-cfg/mix.rs:21:7
  --> /checkout/src/test/ui/check-cfg/mix.rs:21:7
   |
LL | #[cfg(feature = "zebra")]
   |
   |
   = note: possible values for `feature` are: bar, foo

warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:25:12
   |
LL | #[cfg_attr(uu, test)]


warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:34:10
   |
LL |     cfg!(widnows);
   |          ^^^^^^^ help: did you mean: `windows`
warning: unexpected `cfg` condition value
  --> /checkout/src/test/ui/check-cfg/mix.rs:38:10
   |
   |
LL |     cfg!(feature = "zebra");
   |
   |
   = note: possible values for `feature` are: bar, foo

warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:40:10
   |
LL |     cfg!(xxx = "foo");


warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:42:10
   |
LL |     cfg!(xxx);


warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:44:23
   |
LL |     cfg!(any(windows, xxx));


warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:46:14
   |
LL |     cfg!(any(xxx, windows));

warning: unexpected `cfg` condition value
  --> /checkout/src/test/ui/check-cfg/mix.rs:48:14
   |
   |
LL |     cfg!(any(feature = "bad", windows));
   |                        |
   |                        |
   |                        help: did you mean: `"bar"`
   |
   = note: possible values for `feature` are: bar, foo
warning: 10 warnings emitted


------------------------------------------

plain
.................................................................................................... 1100/12767
.................................................................................................... 1200/12767
.....................................i.............................................................. 1300/12767
.........................................................................i.......................... 1400/12767
.........................FFF..F.F................................................................... 1500/12767
......................................................i............................................. 1700/12767
.................................................................................................... 1800/12767
............................................................................i....................... 1900/12767
.................................................................................................... 2000/12767
---

- warning: unexpected `cfg` condition value
-   --> $DIR/empty-values.rs:6:7
-    |
- LL | #[cfg(test = "value")]
-    |           |
-    |           help: remove the value
-    |
-    = note: `#[warn(unexpected_cfgs)]` on by default
---
To only update this specific test, also pass `--test-args check-cfg/empty-values.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/check-cfg/empty-values.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/empty-values" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--check-cfg=values()" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/empty-values/auxiliary"
stdout: none
stderr: none

---- [ui] ui/check-cfg/invalid-cfg-value.rs stdout ----
diff of stderr:


- warning: unexpected `cfg` condition value
-   --> $DIR/invalid-cfg-value.rs:7:7
-    |
- LL | #[cfg(feature = "sedre")]
-    |
-    = note: `#[warn(unexpected_cfgs)]` on by default
-    = note: `#[warn(unexpected_cfgs)]` on by default
-    = note: expected values for `feature` are: full, rand, serde
- warning: 1 warning emitted
- 
- 

---
To only update this specific test, also pass `--test-args check-cfg/invalid-cfg-value.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/check-cfg/invalid-cfg-value.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/invalid-cfg-value" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--check-cfg=values(feature,\"serde\",\"full\")" "--cfg=feature=\"rand\"" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/invalid-cfg-value/auxiliary"
stdout: none
stderr: none

---- [ui] ui/check-cfg/no-values.rs stdout ----
diff of stderr:


- warning: unexpected `cfg` condition value
-   --> $DIR/no-values.rs:6:7
-    |
- LL | #[cfg(feature = "foo")]
-    |
-    = note: `#[warn(unexpected_cfgs)]` on by default
-    = note: no expected value for `feature`
- 
- 
- warning: unexpected `cfg` condition value
-   --> $DIR/no-values.rs:10:7
-    |
- LL | #[cfg(test = "foo")]
-    |           |
-    |           help: remove the value
-    |
-    = note: no expected value for `test`
---
To only update this specific test, also pass `--test-args check-cfg/no-values.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/check-cfg/no-values.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/no-values" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--check-cfg=values(test)" "--check-cfg=values(feature)" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/no-values/auxiliary"
stdout: none
stderr: none

---- [ui] ui/check-cfg/well-known-values.rs stdout ----
diff of stderr:


- warning: unexpected `cfg` condition value
-   --> $DIR/well-known-values.rs:7:7
-    |
- LL | #[cfg(target_os = "linuz")]
-    |                   |
-    |                   |
-    |                   help: did you mean: `"linux"`
-    = note: `#[warn(unexpected_cfgs)]` on by default
-    = note: `#[warn(unexpected_cfgs)]` on by default
-    = note: expected values for `target_os` are: android, cuda, dragonfly, emscripten, espidf, freebsd, fuchsia, haiku, hermit, horizon, illumos, ios, l4re, linux, macos, netbsd, none, openbsd, psp, redox, solaris, solid_asp3, tvos, uefi, unknown, vxworks, wasi, windows
- warning: unexpected `cfg` condition value
-   --> $DIR/well-known-values.rs:14:7
-    |
-    |
- LL | #[cfg(target_has_atomic = "0")]
-    |                           |
-    |                           |
-    |                           help: did you mean: `"8"`
-    |
-    = note: expected values for `target_has_atomic` are: 128, 16, 32, 64, 8, ptr
- warning: unexpected `cfg` condition value
-   --> $DIR/well-known-values.rs:21:7
-    |
-    |
- LL | #[cfg(unix = "aa")]
-    |           |
-    |           help: remove the value
-    |
-    = note: no expected value for `unix`
-    = note: no expected value for `unix`
- 
- warning: unexpected `cfg` condition value
-   --> $DIR/well-known-values.rs:28:7
-    |
- LL | #[cfg(miri = "miri")]
-    |           |
-    |           help: remove the value
-    |
-    = note: no expected value for `miri`
-    = note: no expected value for `miri`
- 
- warning: unexpected `cfg` condition value
-   --> $DIR/well-known-values.rs:35:7
-    |
- LL | #[cfg(doc = "linux")]
-    |       ^^^----------
-    |          help: remove the value
-    |
-    = note: no expected value for `doc`
- 
---
To only update this specific test, also pass `--test-args check-cfg/well-known-values.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/check-cfg/well-known-values.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/well-known-values" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--check-cfg=values()" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/well-known-values/auxiliary"
stdout: none
stderr: none

---- [ui] ui/check-cfg/mix.rs stdout ----
diff of stderr:


6    |
7    = note: `#[warn(unexpected_cfgs)]` on by default
8 
- warning: unexpected `cfg` condition value
-   --> $DIR/mix.rs:21:7
-    |
- LL | #[cfg(feature = "zebra")]
-    |
-    |
-    = note: expected values for `feature` are: bar, foo
17 warning: unexpected `cfg` condition name
18   --> $DIR/mix.rs:25:12
19    |


26 LL |     cfg!(widnows);
27    |          ^^^^^^^ help: did you mean: `windows`
- warning: unexpected `cfg` condition value
-   --> $DIR/mix.rs:38:10
-    |
-    |
- LL |     cfg!(feature = "zebra");
-    |
-    |
-    = note: expected values for `feature` are: bar, foo
37 warning: unexpected `cfg` condition name
38   --> $DIR/mix.rs:40:10
39    |


52 LL |     cfg!(any(xxx, windows));
54 
- warning: unexpected `cfg` condition value
-   --> $DIR/mix.rs:46:14
-    |
-    |
- LL |     cfg!(any(feature = "bad", windows));
-    |                        |
-    |                        |
-    |                        help: did you mean: `"bar"`
-    |
-    = note: expected values for `feature` are: bar, foo
65 warning: unexpected `cfg` condition name
66   --> $DIR/mix.rs:48:23
67    |


98 LL |     cfg!(any(aa, bb));
100 
- warning: unexpected `cfg` condition value
-   --> $DIR/mix.rs:58:20
-    |
-    |
- LL |     cfg!(any(unix, feature = "zebra"));
-    |
-    |
-    = note: expected values for `feature` are: bar, foo
109 warning: unexpected `cfg` condition name
110   --> $DIR/mix.rs:60:14
111    |


112 LL |     cfg!(any(xxx, feature = "zebra"));
114 
- warning: unexpected `cfg` condition value
-   --> $DIR/mix.rs:60:19
-    |
-    |
- LL |     cfg!(any(xxx, feature = "zebra"));
-    |
-    |
-    = note: expected values for `feature` are: bar, foo
123 warning: unexpected `cfg` condition name
124   --> $DIR/mix.rs:63:14
125    |


132 LL |     cfg!(any(xxx, unix, xxx));
134 
- warning: unexpected `cfg` condition value
-   --> $DIR/mix.rs:66:14
-    |
-    |
- LL |     cfg!(all(feature = "zebra", feature = "zebra", feature = "zebra"));
-    |
-    |
-    = note: expected values for `feature` are: bar, foo
- warning: unexpected `cfg` condition value
-   --> $DIR/mix.rs:66:33
-    |
-    |
- LL |     cfg!(all(feature = "zebra", feature = "zebra", feature = "zebra"));
-    |
-    |
-    = note: expected values for `feature` are: bar, foo
- warning: unexpected `cfg` condition value
-   --> $DIR/mix.rs:66:52
-    |
-    |
- LL |     cfg!(all(feature = "zebra", feature = "zebra", feature = "zebra"));
-    |
-    |
-    = note: expected values for `feature` are: bar, foo
- warning: 23 warnings emitted
+ warning: 15 warnings emitted
160 
161 
---
To only update this specific test, also pass `--test-args check-cfg/mix.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/check-cfg/mix.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/mix" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--check-cfg=names()" "--check-cfg=values(feature,\"foo\")" "--cfg" "feature=\"bar\"" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/mix/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/check-cfg/mix.rs:11:7
   |
   |
LL | #[cfg(widnows)]
   |       ^^^^^^^ help: did you mean: `windows`
   = note: `#[warn(unexpected_cfgs)]` on by default

warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:25:12
  --> /checkout/src/test/ui/check-cfg/mix.rs:25:12
   |
LL | #[cfg_attr(uu, test)]

warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:34:10
   |
   |
LL |     cfg!(widnows);
   |          ^^^^^^^ help: did you mean: `windows`
warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:40:10
   |
   |
LL |     cfg!(xxx = "foo");

warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:42:10
   |
   |
LL |     cfg!(xxx);

warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:44:14
   |
   |
LL |     cfg!(any(xxx, windows));

warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:48:23
   |
   |
LL |     cfg!(any(windows, xxx));

warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:50:20
   |
   |
LL |     cfg!(all(unix, xxx));

warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:52:14
   |
   |
LL |     cfg!(all(aa, bb));

warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:52:18
   |
   |
LL |     cfg!(all(aa, bb));

warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:55:14
   |
   |
LL |     cfg!(any(aa, bb));

warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:55:18
   |
   |
LL |     cfg!(any(aa, bb));

warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:60:14
   |
   |
LL |     cfg!(any(xxx, feature = "zebra"));

warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:63:14
   |
   |
LL |     cfg!(any(xxx, unix, xxx));

warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:63:25
   |
   |
LL |     cfg!(any(xxx, unix, xxx));

warning: 15 warnings emitted
------------------------------------------


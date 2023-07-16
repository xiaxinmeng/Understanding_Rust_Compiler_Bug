plain

---- [ui] src/test/ui/check-cfg/mix.rs stdout ----
diff of stderr:

28 LL | #[cfg_attr(uu, test)]
30 
30 
- warning: unexpected condition value `bar` for condition name `feature`
-    |
-    = help: was set with `--cfg` but isn't in the `--check-cfg` expected values
35 warning: unexpected `unknown_name` as condition name
36    |
36    |
37    = help: was set with `--cfg` but isn't in the `--check-cfg` expected names
+ 
+ 
+ warning: unexpected condition value `bar` for condition name `feature`
+    |
+    = help: was set with `--cfg` but isn't in the `--check-cfg` expected values
39 warning: unexpected `cfg` condition name
40   --> $DIR/mix.rs:35:10



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/mix/mix.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args check-cfg/mix.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/check-cfg/mix.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/mix" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--check-cfg=names()" "--check-cfg=values(feature,\"foo\")" "--cfg" "feature=\"bar\"" "--cfg" "unknown_name" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/mix/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/check-cfg/mix.rs:11:7
   |
   |
LL | #[cfg(widnows)]
   |       ^^^^^^^ help: did you mean: `windows`
   = note: `#[warn(unexpected_cfgs)]` on by default

warning: unexpected `cfg` condition value
  --> /checkout/src/test/ui/check-cfg/mix.rs:18:7
---

warning: unexpected `cfg` condition value
  --> /checkout/src/test/ui/check-cfg/mix.rs:22:7
   |
LL | #[cfg(feature = "zebra")]
   |
   = note: expected values for `feature` are: foo

warning: unexpected `cfg` condition name
warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:26:12
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
LL | #[cfg_attr(uu, test)]

warning: unexpected `unknown_name` as condition name
   |
   |
   = help: was set with `--cfg` but isn't in the `--check-cfg` expected names

warning: unexpected condition value `bar` for condition name `feature`
   |
   = help: was set with `--cfg` but isn't in the `--check-cfg` expected values
warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:35:10
   |
   |
LL |     cfg!(widnows);
   |          ^^^^^^^ help: did you mean: `windows`
warning: unexpected `cfg` condition value
  --> /checkout/src/test/ui/check-cfg/mix.rs:38:10
   |
   |
LL |     cfg!(feature = "bar");
   |
   = note: expected values for `feature` are: foo

warning: unexpected `cfg` condition value
warning: unexpected `cfg` condition value
  --> /checkout/src/test/ui/check-cfg/mix.rs:40:10
   |
LL |     cfg!(feature = "zebra");
   |
   = note: expected values for `feature` are: foo

warning: unexpected `cfg` condition name
warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:42:10
   |
LL |     cfg!(xxx = "foo");

warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:44:10
   |
   |
LL |     cfg!(xxx);

warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:46:14
   |
   |
LL |     cfg!(any(xxx, windows));

warning: unexpected `cfg` condition value
  --> /checkout/src/test/ui/check-cfg/mix.rs:48:14
   |
   |
LL |     cfg!(any(feature = "bad", windows));
   |
   = note: expected values for `feature` are: foo

warning: unexpected `cfg` condition name
warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:50:23
   |
LL |     cfg!(any(windows, xxx));

warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:52:20
   |
   |
LL |     cfg!(all(unix, xxx));

warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:54:14
   |
   |
LL |     cfg!(all(aa, bb));

warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:54:18
   |
   |
LL |     cfg!(all(aa, bb));

warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:57:14
   |
   |
LL |     cfg!(any(aa, bb));

warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:57:18
   |
   |
LL |     cfg!(any(aa, bb));

warning: unexpected `cfg` condition value
  --> /checkout/src/test/ui/check-cfg/mix.rs:60:20
   |
   |
LL |     cfg!(any(unix, feature = "zebra"));
   |
   = note: expected values for `feature` are: foo

warning: unexpected `cfg` condition name
warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:62:14
   |
LL |     cfg!(any(xxx, feature = "zebra"));

warning: unexpected `cfg` condition value
  --> /checkout/src/test/ui/check-cfg/mix.rs:62:19
   |
   |
LL |     cfg!(any(xxx, feature = "zebra"));
   |
   = note: expected values for `feature` are: foo

warning: unexpected `cfg` condition name
warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:65:14
   |
LL |     cfg!(any(xxx, unix, xxx));

warning: unexpected `cfg` condition name
  --> /checkout/src/test/ui/check-cfg/mix.rs:65:25
   |
   |
LL |     cfg!(any(xxx, unix, xxx));

warning: unexpected `cfg` condition value
  --> /checkout/src/test/ui/check-cfg/mix.rs:68:14
   |
   |
LL |     cfg!(all(feature = "zebra", feature = "zebra", feature = "zebra"));
   |
   = note: expected values for `feature` are: foo

warning: unexpected `cfg` condition value
warning: unexpected `cfg` condition value
  --> /checkout/src/test/ui/check-cfg/mix.rs:68:33
   |
LL |     cfg!(all(feature = "zebra", feature = "zebra", feature = "zebra"));
   |
   = note: expected values for `feature` are: foo

warning: unexpected `cfg` condition value
warning: unexpected `cfg` condition value
  --> /checkout/src/test/ui/check-cfg/mix.rs:68:52
   |
LL |     cfg!(all(feature = "zebra", feature = "zebra", feature = "zebra"));
   |
   = note: expected values for `feature` are: foo

warning: 27 warnings emitted

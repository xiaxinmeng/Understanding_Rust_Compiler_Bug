plain

---- [ui] src/test/ui/suggestions/unnamable-types.rs stdout ----
diff of stderr:

19 LL | const C: _ = || 42;
20    |          ^ not allowed in type signatures
21    |
- note: however, the inferred type `[closure@$DIR/unnamable-types.rs:17:14: 17:16]` cannot be named
+ note: however, the inferred type `[closure@unnamable-types.rs:17:14]` cannot be named
24    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
25 LL | const C: _ = || 42;

31 LL | const D = S { t: { let i = 0; move || -> i32 { i } } };
33    |
33    |
- note: however, the inferred type `S<[closure@$DIR/unnamable-types.rs:23:31: 23:45]>` cannot be named
+ note: however, the inferred type `S<[closure@unnamable-types.rs:23:31]>` cannot be named
36    |
36    |
37 LL | const D = S { t: { let i = 0; move || -> i32 { i } } };

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/unnamable-types/unnamable-types.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/unnamable-types.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/unnamable-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/unnamable-types" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/unnamable-types/auxiliary"
stdout: none
--- stderr -------------------------------
error: missing type for `const` item
   |
LL | const A = 5;
   |        ^ help: provide a type for the constant: `: i32`


error[E0121]: the placeholder `_` is not allowed within types on item signatures for static variables
   |
   |
LL | static B: _ = "abc";
   |           |
   |           not allowed in type signatures
   |           help: replace with the correct type: `&str`


error[E0121]: the placeholder `_` is not allowed within types on item signatures for constants
   |
   |
LL | const C: _ = || 42;
   |          ^ not allowed in type signatures
   |
note: however, the inferred type `[closure@unnamable-types.rs:17:14]` cannot be named
   |
   |
LL | const C: _ = || 42;

error: missing type for `const` item
  --> /checkout/src/test/ui/suggestions/unnamable-types.rs:23:8
   |
   |
LL | const D = S { t: { let i = 0; move || -> i32 { i } } };
   |
   |
note: however, the inferred type `S<[closure@unnamable-types.rs:23:31]>` cannot be named
   |
   |
LL | const D = S { t: { let i = 0; move || -> i32 { i } } };

error: missing type for `const` item
  --> /checkout/src/test/ui/suggestions/unnamable-types.rs:29:8
   |
   |
LL | const E = foo;
   |        ^ help: provide a type for the constant: `: fn() -> i32`
error: missing type for `const` item
  --> /checkout/src/test/ui/suggestions/unnamable-types.rs:32:8
   |
   |
LL | const F = S { t: foo };
   |        ^ help: provide a type for the constant: `: S<fn() -> i32>`
error: missing type for `const` item
  --> /checkout/src/test/ui/suggestions/unnamable-types.rs:37:8
   |
   |
LL | const G = || -> i32 { yield 0; return 1; };
   |
   |
note: however, the inferred type `[generator@/checkout/src/test/ui/suggestions/unnamable-types.rs:37:11: 37:20]` cannot be named
   |
   |
LL | const G = || -> i32 { yield 0; return 1; };

error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0121`.

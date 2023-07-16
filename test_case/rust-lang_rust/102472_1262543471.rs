plain
failures:

---- [ui] src/test/ui/lint/lint-incoherent-auto-trait-objects.rs stdout ----

error: /checkout/src/test/ui/lint/lint-incoherent-auto-trait-objects.rs:3: unexpected error: '3:6: 3:9: type annotations needed: cannot satisfy `(dyn Send + 'static): Foo` [E0283]'

error: /checkout/src/test/ui/lint/lint-incoherent-auto-trait-objects.rs:5: unexpected error: '5:6: 5:9: type annotations needed: cannot satisfy `(dyn Send + 'static): Foo` [E0283]'

error: /checkout/src/test/ui/lint/lint-incoherent-auto-trait-objects.rs:9: unexpected error: '9:6: 9:9: type annotations needed: cannot satisfy `(dyn Send + Sync + 'static): Foo` [E0283]'

error: /checkout/src/test/ui/lint/lint-incoherent-auto-trait-objects.rs:11: unexpected error: '11:6: 11:9: type annotations needed: cannot satisfy `(dyn Send + Sync + 'static): Foo` [E0283]'

error: /checkout/src/test/ui/lint/lint-incoherent-auto-trait-objects.rs:15: unexpected error: '15:6: 15:9: type annotations needed: cannot satisfy `(dyn Send + Sync + 'static): Foo` [E0283]'
error: 5 unexpected errors found, 0 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-incoherent-auto-trait-objects.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-incoherent-auto-trait-objects" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-incoherent-auto-trait-objects/auxiliary"
    Error {
        line_num: 3,
        kind: Some(
            Error,
            Error,
        ),
        msg: "3:6: 3:9: type annotations needed: cannot satisfy `(dyn Send + 'static): Foo` [E0283]",
    Error {
        line_num: 5,
        kind: Some(
            Error,
            Error,
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
        ),
        msg: "5:6: 5:9: type annotations needed: cannot satisfy `(dyn Send + 'static): Foo` [E0283]",
    Error {
        line_num: 9,
        kind: Some(
            Error,
            Error,
        ),
        msg: "9:6: 9:9: type annotations needed: cannot satisfy `(dyn Send + Sync + 'static): Foo` [E0283]",
    Error {
        line_num: 11,
        kind: Some(
            Error,
            Error,
        ),
        msg: "11:6: 11:9: type annotations needed: cannot satisfy `(dyn Send + Sync + 'static): Foo` [E0283]",
    Error {
        line_num: 15,
        kind: Some(
            Error,
            Error,
        ),
        msg: "15:6: 15:9: type annotations needed: cannot satisfy `(dyn Send + Sync + 'static): Foo` [E0283]",
]

thread '[ui] src/test/ui/lint/lint-incoherent-auto-trait-objects.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [ui] src/test/ui/traits/object/issue-33140-traitobject-crate.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/object/issue-33140-traitobject-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/issue-33140-traitobject-crate" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/issue-33140-traitobject-crate/auxiliary"
stdout: none
--- stderr -------------------------------
warning: conflicting implementations of trait `Trait` for type `(dyn std::marker::Send + std::marker::Sync + 'static)`: (E0119)
   |
   |
LL | unsafe impl Trait for dyn (::std::marker::Send) + Sync { }
   | ------------------------------------------------------ first implementation here
LL | unsafe impl Trait for dyn (::std::marker::Send) + Send + Sync { }
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + std::marker::Sync + 'static)`
note: the lint level is defined here
  --> /checkout/src/test/ui/traits/object/issue-33140-traitobject-crate.rs:3:9
   |
LL | #![warn(order_dependent_trait_objects)]
LL | #![warn(order_dependent_trait_objects)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #56484 <https://github.com/rust-lang/rust/issues/56484>

warning: conflicting implementations of trait `Trait` for type `(dyn std::marker::Send + std::marker::Sync + 'static)`: (E0119)
   |
   |
LL | unsafe impl Trait for dyn (::std::marker::Send) + Send + Sync { }
...
...
LL | unsafe impl Trait for dyn (::std::marker::Sync) + Send { }
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + std::marker::Sync + 'static)`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #56484 <https://github.com/rust-lang/rust/issues/56484>


warning: conflicting implementations of trait `Trait` for type `(dyn std::marker::Send + std::marker::Sync + 'static)`: (E0119)
   |
   |
LL | unsafe impl Trait for dyn (::std::marker::Sync) + Send { }
...
...
LL | unsafe impl Trait for dyn (::std::marker::Sync) + Send + Sync { }
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + std::marker::Sync + 'static)`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #56484 <https://github.com/rust-lang/rust/issues/56484>


error[E0283]: type annotations needed: cannot satisfy `(dyn Send + Sync + 'static): Trait`
   |
   |
LL | unsafe impl Trait for dyn (::std::marker::Send) + Sync { }
   |
   |
note: multiple `impl`s satisfying `(dyn Send + Sync + 'static): Trait` found
   |
   |
LL | unsafe impl Trait for dyn (::std::marker::Send) + Sync { }
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | unsafe impl Trait for dyn (::std::marker::Send) + Send + Sync { }
...
...
LL | unsafe impl Trait for dyn (::std::marker::Sync) + Send { }
...
...
LL | unsafe impl Trait for dyn (::std::marker::Sync) + Send + Sync { }


error[E0283]: type annotations needed: cannot satisfy `(dyn Send + Sync + 'static): Trait`
   |
   |
LL | unsafe impl Trait for dyn (::std::marker::Send) + Send + Sync { }
   |
   |
note: multiple `impl`s satisfying `(dyn Send + Sync + 'static): Trait` found
   |
   |
LL | unsafe impl Trait for dyn (::std::marker::Send) + Sync { }
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | unsafe impl Trait for dyn (::std::marker::Send) + Send + Sync { }
...
...
LL | unsafe impl Trait for dyn (::std::marker::Sync) + Send { }
...
...
LL | unsafe impl Trait for dyn (::std::marker::Sync) + Send + Sync { }


error[E0283]: type annotations needed: cannot satisfy `(dyn Send + Sync + 'static): Trait`
   |
   |
LL | unsafe impl Trait for dyn (::std::marker::Sync) + Send { }
   |
   |
note: multiple `impl`s satisfying `(dyn Send + Sync + 'static): Trait` found
   |
   |
LL | unsafe impl Trait for dyn (::std::marker::Send) + Sync { }
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | unsafe impl Trait for dyn (::std::marker::Send) + Send + Sync { }
...
...
LL | unsafe impl Trait for dyn (::std::marker::Sync) + Send { }
...
...
LL | unsafe impl Trait for dyn (::std::marker::Sync) + Send + Sync { }


error[E0283]: type annotations needed: cannot satisfy `(dyn Send + Sync + 'static): Trait`
   |
   |
LL | unsafe impl Trait for dyn (::std::marker::Sync) + Send + Sync { }
   |
   |
note: multiple `impl`s satisfying `(dyn Send + Sync + 'static): Trait` found
   |
   |
LL | unsafe impl Trait for dyn (::std::marker::Send) + Sync { }
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | unsafe impl Trait for dyn (::std::marker::Send) + Send + Sync { }
...
...
LL | unsafe impl Trait for dyn (::std::marker::Sync) + Send { }
...
...
LL | unsafe impl Trait for dyn (::std::marker::Sync) + Send + Sync { }

error: aborting due to 4 previous errors; 3 warnings emitted

For more information about this error, try `rustc --explain E0283`.

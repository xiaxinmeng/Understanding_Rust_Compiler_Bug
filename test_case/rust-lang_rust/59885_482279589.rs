plain
[01:03:42] 
[01:03:42] ---- [ui] ui/privacy/pub-priv-dep/pub-priv1.rs stdout ----
[01:03:42] diff of stderr:
[01:03:42] 
[01:03:42] - error: type `priv_dep::OtherType` from private dependency 'priv_dep' in public interface
[01:03:42] -   --> $DIR/pub-priv1.rs:21:5
[01:03:42] -    |
[01:03:42] - LL |     pub field: OtherType,
[01:03:42] -    |
[01:03:42] - note: lint level defined here
[01:03:42] -   --> $DIR/pub-priv1.rs:4:9
[01:03:42] -    |
[01:03:42] -    |
[01:03:42] - LL | #![deny(exported_private_dependencies)]
[01:03:42] -    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:42] + error: extern location for priv_dep does not exist: $TEST_BUILD_DIR/privacy/pub-priv-dep/pub-priv1/auxiliary/libpriv_dep.so
[01:03:42] 12 
[01:03:42] - error: type `priv_dep::OtherType` from private dependency 'priv_dep' in public interface
[01:03:42] -   --> $DIR/pub-priv1.rs:28:5
[01:03:42] + error[E0463]: can't find crate for `priv_dep`
[01:03:42] +   --> $DIR/pub-priv1.rs:7:1
[01:03:42] 15    |
[01:03:42] - LL |     pub fn pub_fn(param: OtherType) {}
[01:03:42] -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:03:42] + LL | extern crate priv_dep;
[01:03:42] 18 
[01:03:42] 18 
[01:03:42] - error: trait `priv_dep::OtherTrait` from private dependency 'priv_dep' in public interface
[01:03:42] -   --> $DIR/pub-priv1.rs:34:1
[01:03:42] -    |
[01:03:42] - LL | / pub trait MyPubTrait {
[01:03:42] - LL | |     type Foo: OtherTrait;
[01:03:42] - LL | | }
[01:03:42] + error: aborting due to 2 previous errors
[01:03:42] 26 
[01:03:42] - error: aborting due to 3 previous errors
[01:03:42] - 
[01:03:42] - 
[01:03:42] + For more information about this error, try `rustc --explain E0463`.
[01:03:42] 29 
[01:03:42] 
[01:03:42] 
[01:03:42] The actual stderr differed from the expected stderr.
[01:03:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/pub-priv-dep/pub-priv1/pub-priv1.stderr
[01:03:42] To update references, rerun the tests and pass the `--bless` flag
[01:03:42] To only update this specific test, also pass `--test-args privacy/pub-priv-dep/pub-priv1.rs`
[01:03:42] error: 1 errors occurred comparing output.
[01:03:42] status: exit code: 1
[01:03:42] status: exit code: 1
[01:03:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/pub-priv-dep/pub-priv1.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/pub-priv-dep/pub-priv1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/pub-priv-dep/pub-priv1/auxiliary" "-A" "unused" "--extern-private" "priv_dep=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/pub-priv-dep/pub-priv1/auxiliary/libpriv_dep.so"
[01:03:42] ------------------------------------------
[01:03:42] 
[01:03:42] ------------------------------------------
[01:03:42] stderr:
[01:03:42] stderr:
[01:03:42] ------------------------------------------
[01:03:42] {"message":"extern location for priv_dep does not exist: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/pub-priv-dep/pub-priv1/auxiliary/libpriv_dep.so","code":null,"level":"error","spans":[],"children":[],"rendered":"error: extern location for priv_dep does not exist: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/pub-priv-dep/pub-priv1/auxiliary/libpriv_dep.so\n\n"}
[01:03:42] {"message":"can't find crate for `priv_dep`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n
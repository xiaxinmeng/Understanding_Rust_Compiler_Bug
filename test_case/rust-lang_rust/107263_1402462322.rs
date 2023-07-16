plain
........................................................................................ 12232/14235
........................................................................................ 12320/14235
........................................................................................ 12408/14235
........................................................................................ 12496/14235
.F............F...i.......i.........i....i......................i....................... 12584/14235
........................................................................................ 12760/14235
........................................................................................ 12848/14235
........................................................................................ 12936/14235
........................................................................................ 13024/14235
---
failures:

---- [ui] tests/ui/privacy/private-in-public-ill-formed.rs stdout ----

error: /checkout/tests/ui/privacy/private-in-public-ill-formed.rs:15: unexpected error: '15:9: 15:28: private type `aliases_pub::Priv` in public interface [E0446]'

error: /checkout/tests/ui/privacy/private-in-public-ill-formed.rs:15: unexpected error: '15:9: 15:28: private trait `aliases_pub::PrivTr` in public interface [E0445]'

error: /checkout/tests/ui/privacy/private-in-public-ill-formed.rs:15: unexpected error: '15:9: 15:28: private type `aliases_pub::Priv` in public interface [E0446]'

error: /checkout/tests/ui/privacy/private-in-public-ill-formed.rs:33: unexpected error: '33:9: 33:28: private type `aliases_priv::Priv` in public interface [E0446]'

error: /checkout/tests/ui/privacy/private-in-public-ill-formed.rs:33: unexpected error: '33:9: 33:28: private trait `aliases_priv::PrivTr` in public interface [E0445]'

error: /checkout/tests/ui/privacy/private-in-public-ill-formed.rs:33: unexpected error: '33:9: 33:28: private type `aliases_priv::Priv` in public interface [E0446]'
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

error: /checkout/tests/ui/privacy/private-in-public-ill-formed.rs:16: expected error not found: private type `aliases_pub::Priv` in public interface

error: /checkout/tests/ui/privacy/private-in-public-ill-formed.rs:17: expected error not found: private type `aliases_pub::Priv` in public interface
error: 6 unexpected errors found, 2 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/privacy/private-in-public-ill-formed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-in-public-ill-formed" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-in-public-ill-formed/auxiliary"
    Error {
        line_num: 15,
        kind: Some(
            Error,
            Error,
        ),
        msg: "15:9: 15:28: private type `aliases_pub::Priv` in public interface [E0446]",
    Error {
        line_num: 15,
        kind: Some(
            Error,
            Error,
        ),
        msg: "15:9: 15:28: private trait `aliases_pub::PrivTr` in public interface [E0445]",
    Error {
        line_num: 15,
        kind: Some(
            Error,
            Error,
        ),
        msg: "15:9: 15:28: private type `aliases_pub::Priv` in public interface [E0446]",
    Error {
        line_num: 33,
        kind: Some(
            Error,
            Error,
        ),
        msg: "33:9: 33:28: private type `aliases_priv::Priv` in public interface [E0446]",
    Error {
        line_num: 33,
        kind: Some(
            Error,
            Error,
        ),
        msg: "33:9: 33:28: private trait `aliases_priv::PrivTr` in public interface [E0445]",
    Error {
        line_num: 33,
        kind: Some(
            Error,
            Error,
        ),
        msg: "33:9: 33:28: private type `aliases_priv::Priv` in public interface [E0446]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 16,
        kind: Some(
            Error,
        ),
        msg: "private type `aliases_pub::Priv` in public interface",
    Error {
        line_num: 17,
        kind: Some(
            Error,
            Error,
        ),
        msg: "private type `aliases_pub::Priv` in public interface",
]

thread '[ui] tests/ui/privacy/private-in-public-ill-formed.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1422:13


---- [ui] tests/ui/symbol-names/const-generics.rs#legacy stdout ----

error in revision `legacy`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/symbol-names/const-generics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics.legacy" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics.legacy/auxiliary" "-Z" "unstable-options" "-C" "symbol-mangling-version=legacy" "--crate-type=lib"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when finding all inherent impls defined in crate
   |
   = note: ...which requires normalizing `I8<{i8::MIN}>`...
note: ...which requires evaluating type-level constant...
  --> fake-test-src-base/symbol-names/const-generics.rs:20:9
   |
LL | impl I8<{i8::MIN}> {
   |         ^^^^^^^^^
note: ...which requires const-evaluating + checking `<impl at fake-test-src-base/symbol-names/const-generics.rs:20:1: 20:19>::{constant#0}`...
  --> fake-test-src-base/symbol-names/const-generics.rs:20:9
   |
LL | impl I8<{i8::MIN}> {
   |         ^^^^^^^^^
note: ...which requires const-evaluating + checking `<impl at fake-test-src-base/symbol-names/const-generics.rs:20:1: 20:19>::{constant#0}`...
  --> fake-test-src-base/symbol-names/const-generics.rs:20:9
   |
LL | impl I8<{i8::MIN}> {
   |         ^^^^^^^^^
note: ...which requires caching MIR for CTFE of the const argument `<impl at fake-test-src-base/symbol-names/const-generics.rs:20:1: 20:19>::{constant#0}`...
  --> fake-test-src-base/symbol-names/const-generics.rs:20:9
   |
LL | impl I8<{i8::MIN}> {
   |         ^^^^^^^^^
note: ...which requires elaborating drops for `<impl at fake-test-src-base/symbol-names/const-generics.rs:20:1: 20:19>::{constant#0}`...
  --> fake-test-src-base/symbol-names/const-generics.rs:20:9
   |
LL | impl I8<{i8::MIN}> {
   |         ^^^^^^^^^
note: ...which requires borrow-checking the const argument`<impl at fake-test-src-base/symbol-names/const-generics.rs:20:1: 20:19>::{constant#0}`...
  --> fake-test-src-base/symbol-names/const-generics.rs:20:9
   |
LL | impl I8<{i8::MIN}> {
   |         ^^^^^^^^^
note: ...which requires processing MIR for the const argument `<impl at fake-test-src-base/symbol-names/const-generics.rs:20:1: 20:19>::{constant#0}`...
  --> fake-test-src-base/symbol-names/const-generics.rs:20:9
   |
LL | impl I8<{i8::MIN}> {
   |         ^^^^^^^^^
note: ...which requires const checking the const argument `<impl at fake-test-src-base/symbol-names/const-generics.rs:20:1: 20:19>::{constant#0}`...
  --> fake-test-src-base/symbol-names/const-generics.rs:20:9
   |
LL | impl I8<{i8::MIN}> {
   |         ^^^^^^^^^
note: ...which requires preparing the const argument `<impl at fake-test-src-base/symbol-names/const-generics.rs:20:1: 20:19>::{constant#0}` for borrow checking...
  --> fake-test-src-base/symbol-names/const-generics.rs:20:9
   |
LL | impl I8<{i8::MIN}> {
   |         ^^^^^^^^^
note: ...which requires unsafety-checking the const argument `<impl at fake-test-src-base/symbol-names/const-generics.rs:20:1: 20:19>::{constant#0}`...
  --> fake-test-src-base/symbol-names/const-generics.rs:20:9
   |
LL | impl I8<{i8::MIN}> {
   |         ^^^^^^^^^
note: ...which requires building MIR for `<impl at fake-test-src-base/symbol-names/const-generics.rs:20:1: 20:19>::{constant#0}`...
  --> fake-test-src-base/symbol-names/const-generics.rs:20:9
   |
LL | impl I8<{i8::MIN}> {
   |         ^^^^^^^^^
note: ...which requires building THIR for `<impl at fake-test-src-base/symbol-names/const-generics.rs:20:1: 20:19>::{constant#0}`...
  --> fake-test-src-base/symbol-names/const-generics.rs:20:9
   |
LL | impl I8<{i8::MIN}> {
   |         ^^^^^^^^^
note: ...which requires type-checking the const argument `<impl at fake-test-src-base/symbol-names/const-generics.rs:20:1: 20:19>::{constant#0}`...
  --> fake-test-src-base/symbol-names/const-generics.rs:20:9
   |
LL | impl I8<{i8::MIN}> {
   |         ^^^^^^^^^
   = note: ...which requires collecting all inherent impls for `IntSimplifiedType(I8)`...
   = note: ...which requires collecting all impls for a type in a crate...
   = note: ...which again requires finding all inherent impls defined in crate, completing the cycle
   = note: cycle used when running analysis passes on this crate
error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/symbol-names/const-generics.rs#v0 stdout ----

error in revision `v0`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/symbol-names/const-generics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "v0" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics.v0" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics.v0/auxiliary" "-C" "symbol-mangling-version=v0" "--crate-type=lib"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when finding all inherent impls defined in crate
   |
   = note: ...which requires normalizing `I8<{i8::MIN}>`...
note: ...which requires evaluating type-level constant...
  --> fake-test-src-base/symbol-names/const-generics.rs:20:9
   |
LL | impl I8<{i8::MIN}> {
   |         ^^^^^^^^^
note: ...which requires const-evaluating + checking `<impl at fake-test-src-base/symbol-names/const-generics.rs:20:1: 20:19>::{constant#0}`...
  --> fake-test-src-base/symbol-names/const-generics.rs:20:9
   |
LL | impl I8<{i8::MIN}> {
   |         ^^^^^^^^^
note: ...which requires const-evaluating + checking `<impl at fake-test-src-base/symbol-names/const-generics.rs:20:1: 20:19>::{constant#0}`...
  --> fake-test-src-base/symbol-names/const-generics.rs:20:9
   |
LL | impl I8<{i8::MIN}> {
   |         ^^^^^^^^^
note: ...which requires caching MIR for CTFE of the const argument `<impl at fake-test-src-base/symbol-names/const-generics.rs:20:1: 20:19>::{constant#0}`...
  --> fake-test-src-base/symbol-names/const-generics.rs:20:9
   |
LL | impl I8<{i8::MIN}> {
   |         ^^^^^^^^^
note: ...which requires elaborating drops for `<impl at fake-test-src-base/symbol-names/const-generics.rs:20:1: 20:19>::{constant#0}`...
  --> fake-test-src-base/symbol-names/const-generics.rs:20:9
   |
LL | impl I8<{i8::MIN}> {
   |         ^^^^^^^^^
note: ...which requires borrow-checking the const argument`<impl at fake-test-src-base/symbol-names/const-generics.rs:20:1: 20:19>::{constant#0}`...
  --> fake-test-src-base/symbol-names/const-generics.rs:20:9
   |
LL | impl I8<{i8::MIN}> {
   |         ^^^^^^^^^
note: ...which requires processing MIR for the const argument `<impl at fake-test-src-base/symbol-names/const-generics.rs:20:1: 20:19>::{constant#0}`...
  --> fake-test-src-base/symbol-names/const-generics.rs:20:9
   |
LL | impl I8<{i8::MIN}> {
   |         ^^^^^^^^^
note: ...which requires const checking the const argument `<impl at fake-test-src-base/symbol-names/const-generics.rs:20:1: 20:19>::{constant#0}`...
  --> fake-test-src-base/symbol-names/const-generics.rs:20:9
   |
LL | impl I8<{i8::MIN}> {
   |         ^^^^^^^^^
note: ...which requires preparing the const argument `<impl at fake-test-src-base/symbol-names/const-generics.rs:20:1: 20:19>::{constant#0}` for borrow checking...
  --> fake-test-src-base/symbol-names/const-generics.rs:20:9
   |
LL | impl I8<{i8::MIN}> {
   |         ^^^^^^^^^
note: ...which requires unsafety-checking the const argument `<impl at fake-test-src-base/symbol-names/const-generics.rs:20:1: 20:19>::{constant#0}`...
  --> fake-test-src-base/symbol-names/const-generics.rs:20:9
   |
LL | impl I8<{i8::MIN}> {
   |         ^^^^^^^^^
note: ...which requires building MIR for `<impl at fake-test-src-base/symbol-names/const-generics.rs:20:1: 20:19>::{constant#0}`...
  --> fake-test-src-base/symbol-names/const-generics.rs:20:9
   |
LL | impl I8<{i8::MIN}> {
   |         ^^^^^^^^^
note: ...which requires building THIR for `<impl at fake-test-src-base/symbol-names/const-generics.rs:20:1: 20:19>::{constant#0}`...
  --> fake-test-src-base/symbol-names/const-generics.rs:20:9
   |
LL | impl I8<{i8::MIN}> {
   |         ^^^^^^^^^
note: ...which requires type-checking the const argument `<impl at fake-test-src-base/symbol-names/const-generics.rs:20:1: 20:19>::{constant#0}`...
  --> fake-test-src-base/symbol-names/const-generics.rs:20:9
   |
LL | impl I8<{i8::MIN}> {
   |         ^^^^^^^^^
   = note: ...which requires collecting all inherent impls for `IntSimplifiedType(I8)`...
   = note: ...which requires collecting all impls for a type in a crate...
   = note: ...which again requires finding all inherent impls defined in crate, completing the cycle
   = note: cycle used when running analysis passes on this crate
error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
------------------------------------------

plain
.................................................................................................... 3600/11269
.................................................................................................... 3700/11269
.................................................................................................... 3800/11269
...........................................................i........................................ 3900/11269
.F....................F............................................................................. 4000/11269
.................................................................................................... 4200/11269
............ii..................................................................................i... 4300/11269
.................................................................................................... 4400/11269
.................................................................................................... 4500/11269
---
..............................................................................................i..... 8100/11269
.................................................................................................... 8200/11269
.................................................................................................... 8300/11269
..........i......................................................................................... 8400/11269
......................................................FF.....................................F...... 8500/11269
F....................................................................iiii.iiii...................... 8600/11269
.................................................................................................... 8800/11269
.................................................................................................... 8900/11269
.................................................................................................... 9000/11269
.................................................................................................... 9100/11269
---
...........................................................i.i...................................... 11200/11269
.....................................................................
failures:

---- [ui] ui/const-generics/eval-privacy.rs stdout ----

error: /checkout/src/test/ui/const-generics/eval-privacy.rs:12: unexpected warning: '12:1: 23:2: private function `fn(u8) -> u8 {my_const_fn}` in public interface (error E0446) [private_in_public]'

error: /checkout/src/test/ui/const-generics/eval-privacy.rs:18: unexpected error: '18:5: 18:46: private function `fn(u8) -> u8 {my_const_fn}` in public interface [E0446]'
error: /checkout/src/test/ui/const-generics/eval-privacy.rs:12: expected warning not found: private type

error: /checkout/src/test/ui/const-generics/eval-privacy.rs:18: expected error not found: private type


error: 2 unexpected errors found, 2 expected errors not found
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/eval-privacy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/eval-privacy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/eval-privacy/auxiliary"
    Error {
        line_num: 12,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "12:1: 23:2: private function `fn(u8) -> u8 {my_const_fn}` in public interface (error E0446) [private_in_public]",
    Error {
        line_num: 18,
        kind: Some(
            Error,
            Error,
        ),
        msg: "18:5: 18:46: private function `fn(u8) -> u8 {my_const_fn}` in public interface [E0446]",
]

not found errors (from test file): [
    Error {
---
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [ui] ui/hygiene/impl_items.rs stdout ----

error: /checkout/src/test/ui/hygiene/impl_items.rs:12: unexpected error: '12:23: 12:24: function `for<'r> fn(&'r foo::S) {foo::S::f}` is private'

error: /checkout/src/test/ui/hygiene/impl_items.rs:12: expected error not found: type `for<'r> fn(&'r foo::S) {foo::S::f}` is private
error: 1 unexpected errors found, 1 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/impl_items.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/impl_items" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/impl_items/auxiliary"
    Error {
        line_num: 12,
        kind: Some(
            Error,
            Error,
        ),
        msg: "12:23: 12:24: function `for<\'r> fn(&\'r foo::S) {foo::S::f}` is private",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 12,
        kind: Some(
            Error,
        ),
        msg: "type `for<\'r> fn(&\'r foo::S) {foo::S::f}` is private",
]

thread '[ui] ui/hygiene/impl_items.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13


---- [ui] ui/hygiene/intercrate.rs stdout ----

error: /checkout/src/test/ui/hygiene/intercrate.rs:10: unexpected error: '10:16: 10:37: function `fn() -> u32 {foo::bar::f}` is private'

error: /checkout/src/test/ui/hygiene/intercrate.rs:10: expected error not found: type `fn() -> u32 {foo::bar::f}` is private
error: 1 unexpected errors found, 1 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/intercrate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/intercrate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/intercrate/auxiliary"
    Error {
        line_num: 10,
        kind: Some(
            Error,
            Error,
        ),
        msg: "10:16: 10:37: function `fn() -> u32 {foo::bar::f}` is private",
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 10,
        kind: Some(
            Error,
        ),
        msg: "type `fn() -> u32 {foo::bar::f}` is private",
]

thread '[ui] ui/hygiene/intercrate.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13


---- [ui] ui/privacy/associated-item-privacy-inherent.rs stdout ----

error: /checkout/src/test/ui/privacy/associated-item-privacy-inherent.rs:13: unexpected error: '13:21: 13:32: function `for<'r> fn(&'r priv_nominal::Pub) {priv_nominal::Pub::method}` is private'

error: /checkout/src/test/ui/privacy/associated-item-privacy-inherent.rs:15: unexpected error: '15:9: 15:14: function `for<'r> fn(&'r priv_nominal::Pub) {priv_nominal::Pub::method}` is private'

error: /checkout/src/test/ui/privacy/associated-item-privacy-inherent.rs:17: unexpected error: '17:13: 17:19: function `for<'r> fn(&'r priv_nominal::Pub) {priv_nominal::Pub::method}` is private'

error: /checkout/src/test/ui/privacy/associated-item-privacy-inherent.rs:13: expected error not found: type `for<'r> fn(&'r priv_nominal::Pub) {priv_nominal::Pub::method}` is private

error: /checkout/src/test/ui/privacy/associated-item-privacy-inherent.rs:15: expected error not found: type `for<'r> fn(&'r priv_nominal::Pub) {priv_nominal::Pub::method}` is private

error: /checkout/src/test/ui/privacy/associated-item-privacy-inherent.rs:17: expected error not found: type `for<'r> fn(&'r priv_nominal::Pub) {priv_nominal::Pub::method}` is private
error: 3 unexpected errors found, 3 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/associated-item-privacy-inherent.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/associated-item-privacy-inherent" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/associated-item-privacy-inherent/auxiliary"
    Error {
        line_num: 13,
        kind: Some(
            Error,
            Error,
        ),
        msg: "13:21: 13:32: function `for<\'r> fn(&\'r priv_nominal::Pub) {priv_nominal::Pub::method}` is private",
    Error {
        line_num: 15,
        kind: Some(
            Error,
            Error,
        ),
        msg: "15:9: 15:14: function `for<\'r> fn(&\'r priv_nominal::Pub) {priv_nominal::Pub::method}` is private",
    Error {
        line_num: 17,
        kind: Some(
            Error,
            Error,
        ),
        msg: "17:13: 17:19: function `for<\'r> fn(&\'r priv_nominal::Pub) {priv_nominal::Pub::method}` is private",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 13,
        kind: Some(
            Error,
        ),
        msg: "type `for<\'r> fn(&\'r priv_nominal::Pub) {priv_nominal::Pub::method}` is private",
    Error {
        line_num: 15,
        kind: Some(
            Error,
            Error,
        ),
        msg: "type `for<\'r> fn(&\'r priv_nominal::Pub) {priv_nominal::Pub::method}` is private",
    Error {
        line_num: 17,
        kind: Some(
            Error,
            Error,
        ),
        msg: "type `for<\'r> fn(&\'r priv_nominal::Pub) {priv_nominal::Pub::method}` is private",
]

thread '[ui] ui/privacy/associated-item-privacy-inherent.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13


---- [ui] ui/privacy/associated-item-privacy-trait.rs stdout ----

error: /checkout/src/test/ui/privacy/associated-item-privacy-trait.rs:17: unexpected error: '17:21: 17:44: function `for<'r> fn(&'r priv_trait::Pub) {<priv_trait::Pub as PrivTr>::method}` is private'

error: /checkout/src/test/ui/privacy/associated-item-privacy-trait.rs:19: unexpected error: '19:9: 19:14: function `for<'r> fn(&'r priv_trait::Pub) {<priv_trait::Pub as PrivTr>::method}` is private'

error: /checkout/src/test/ui/privacy/associated-item-privacy-trait.rs:21: unexpected error: '21:13: 21:19: function `for<'r> fn(&'r Self) {<Self as PrivTr>::method}` is private'

error: /checkout/src/test/ui/privacy/associated-item-privacy-trait.rs:17: expected error not found: type `for<'r> fn(&'r priv_trait::Pub) {<priv_trait::Pub as PrivTr>::method}` is private

error: /checkout/src/test/ui/privacy/associated-item-privacy-trait.rs:19: expected error not found: type `for<'r> fn(&'r priv_trait::Pub) {<priv_trait::Pub as PrivTr>::method}` is private

error: /checkout/src/test/ui/privacy/associated-item-privacy-trait.rs:21: expected error not found: type `for<'r> fn(&'r Self) {<Self as PrivTr>::method}` is private
error: 3 unexpected errors found, 3 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/associated-item-privacy-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/associated-item-privacy-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/associated-item-privacy-trait/auxiliary"
    Error {
        line_num: 17,
        kind: Some(
            Error,
            Error,
        ),
        msg: "17:21: 17:44: function `for<\'r> fn(&\'r priv_trait::Pub) {<priv_trait::Pub as PrivTr>::method}` is private",
    Error {
        line_num: 19,
        kind: Some(
            Error,
            Error,
        ),
        msg: "19:9: 19:14: function `for<\'r> fn(&\'r priv_trait::Pub) {<priv_trait::Pub as PrivTr>::method}` is private",
    Error {
        line_num: 21,
        kind: Some(
            Error,
            Error,
        ),
        msg: "21:13: 21:19: function `for<\'r> fn(&\'r Self) {<Self as PrivTr>::method}` is private",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 17,
        kind: Some(
            Error,
        ),
        msg: "type `for<\'r> fn(&\'r priv_trait::Pub) {<priv_trait::Pub as PrivTr>::method}` is private",
    Error {
        line_num: 19,
        kind: Some(
            Error,
            Error,
        ),
        msg: "type `for<\'r> fn(&\'r priv_trait::Pub) {<priv_trait::Pub as PrivTr>::method}` is private",
    Error {
        line_num: 21,
        kind: Some(
            Error,
            Error,
        ),
        msg: "type `for<\'r> fn(&\'r Self) {<Self as PrivTr>::method}` is private",
]

thread '[ui] ui/privacy/associated-item-privacy-trait.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13


---- [ui] ui/privacy/private-inferred-type.rs stdout ----

error: /checkout/src/test/ui/privacy/private-inferred-type.rs:39: unexpected error: '39:9: 39:16: function `fn() {priv_fn}` is private'

error: /checkout/src/test/ui/privacy/private-inferred-type.rs:43: unexpected error: '43:9: 43:34: function `fn() {<u8 as PrivTrait>::method}` is private'

error: /checkout/src/test/ui/privacy/private-inferred-type.rs:45: unexpected error: '45:9: 45:24: function `fn(u8) -> PrivTupleStruct {PrivTupleStruct}` is private'

error: /checkout/src/test/ui/privacy/private-inferred-type.rs:47: unexpected error: '47:9: 47:23: function `fn(u8) -> PubTupleStruct {PubTupleStruct}` is private'

error: /checkout/src/test/ui/privacy/private-inferred-type.rs:49: unexpected error: '49:18: 49:29: function `for<'r> fn(&'r Pub<u8>) {Pub::<u8>::priv_method}` is private'

error: /checkout/src/test/ui/privacy/private-inferred-type.rs:39: expected error not found: type `fn() {priv_fn}` is private

error: /checkout/src/test/ui/privacy/private-inferred-type.rs:43: expected error not found: type `fn() {<u8 as PrivTrait>::method}` is private

error: /checkout/src/test/ui/privacy/private-inferred-type.rs:45: expected error not found: type `fn(u8) -> PrivTupleStruct {PrivTupleStruct}` is private

error: /checkout/src/test/ui/privacy/private-inferred-type.rs:47: expected error not found: type `fn(u8) -> PubTupleStruct {PubTupleStruct}` is private

error: /checkout/src/test/ui/privacy/private-inferred-type.rs:49: expected error not found: type `for<'r> fn(&'r Pub<u8>) {Pub::<u8>::priv_method}` is private
error: 5 unexpected errors found, 5 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/private-inferred-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-inferred-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-inferred-type/auxiliary"
    Error {
        line_num: 39,
        kind: Some(
            Error,
            Error,
        ),
        msg: "39:9: 39:16: function `fn() {priv_fn}` is private",
    Error {
        line_num: 43,
        kind: Some(
            Error,
            Error,
        ),
        msg: "43:9: 43:34: function `fn() {<u8 as PrivTrait>::method}` is private",
    Error {
        line_num: 45,
        kind: Some(
            Error,
            Error,
        ),
        msg: "45:9: 45:24: function `fn(u8) -> PrivTupleStruct {PrivTupleStruct}` is private",
    Error {
        line_num: 47,
        kind: Some(
            Error,
            Error,
        ),
        msg: "47:9: 47:23: function `fn(u8) -> PubTupleStruct {PubTupleStruct}` is private",
    Error {
        line_num: 49,
        kind: Some(
            Error,
            Error,
        ),
        msg: "49:18: 49:29: function `for<\'r> fn(&\'r Pub<u8>) {Pub::<u8>::priv_method}` is private",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 39,
        kind: Some(
            Error,
        ),
        msg: "type `fn() {priv_fn}` is private",
    Error {
        line_num: 43,
        kind: Some(
            Error,
            Error,
        ),
        msg: "type `fn() {<u8 as PrivTrait>::method}` is private",
    Error {
        line_num: 45,
        kind: Some(
            Error,
            Error,
        ),
        msg: "type `fn(u8) -> PrivTupleStruct {PrivTupleStruct}` is private",
    Error {
        line_num: 47,
        kind: Some(
            Error,
            Error,
        ),
        msg: "type `fn(u8) -> PubTupleStruct {PubTupleStruct}` is private",
    Error {
        line_num: 49,
        kind: Some(
            Error,
            Error,
        ),
        msg: "type `for<\'r> fn(&\'r Pub<u8>) {Pub::<u8>::priv_method}` is private",
]

thread '[ui] ui/privacy/private-inferred-type.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13


---- [ui] ui/privacy/private-inferred-type-3.rs stdout ----

error: error pattern 'type `fn() {ext::priv_fn}` is private' not found!

error: error pattern 'type `fn() {<u8 as ext::PrivTrait>::method}` is private' not found!

error: error pattern 'type `fn(u8) -> ext::PrivTupleStruct {ext::PrivTupleStruct}` is private' not found!

error: error pattern 'type `fn(u8) -> PubTupleStruct {PubTupleStruct}` is private' not found!

error: error pattern 'type `for<'r> fn(&'r Pub<u8>) {Pub::<u8>::priv_method}` is private' not found!

error: multiple error patterns not found
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/private-inferred-type-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-inferred-type-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-inferred-type-3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: function `fn() {ext::priv_fn}` is private
   |
LL |     ext::m!();
   |     ^^^^^^^^^^ private function
   |
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: static `PRIV_STATIC` is private
   |
LL |     ext::m!();
   |     ^^^^^^^^^^ private static
   |
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: type `ext::PrivEnum` is private
   |
LL |     ext::m!();
   |     ^^^^^^^^^^ private type
   |
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: function `fn() {<u8 as ext::PrivTrait>::method}` is private
   |
LL |     ext::m!();
   |     ^^^^^^^^^^ private function
   |
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: function `fn(u8) -> ext::PrivTupleStruct {ext::PrivTupleStruct}` is private
   |
LL |     ext::m!();
   |     ^^^^^^^^^^ private function
   |
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: function `fn(u8) -> PubTupleStruct {PubTupleStruct}` is private
   |
LL |     ext::m!();
   |     ^^^^^^^^^^ private function
   |
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: function `for<'r> fn(&'r Pub<u8>) {Pub::<u8>::priv_method}` is private
   |
LL |     ext::m!();
   |     ^^^^^^^^^^ private function
   |
---
test result: FAILED. 11175 passed; 7 failed; 87 ignored; 0 measured; 0 filtered out; finished in 137.03s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:06

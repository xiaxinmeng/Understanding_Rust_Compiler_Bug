plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between b4151a41a0b275dee59ffbbc115e7bfc5be8a8c3 and 0baa57ad786d14ee4be3a9470591d9d969013b68
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
tests/fail/weak_memory/racing_mixed_size.rs  ... ok
tests/fail/panic/double_panic.rs  ... ok

tests/fail/dangling_pointers/null_pointer_write_zst.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "-Aunused" "--sysroot" "/home/user/.cache/miri/HOST" "-Zui-testing" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-1fd7bc8ace8b19d6.rmeta" "--extern" "getrandom_2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-28b21595824ceb29.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-939b1abf4012e1ce.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-02d594a0068c74d5.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-c0be874f2cc201b5.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-9b15165aa8ec1acf.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-f60083e028003752" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-193d9ef40856b5f0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-a0ff605b584f67a4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-4cab3f2a29ec0f70" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-399f873fd86b1ba6" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-de8d28cfa3a1467f" "tests/fail/dangling_pointers/null_pointer_write_zst.rs" "--error-format=json" "-Zmir-opt-level=0"
actual output differed from expected tests/fail/dangling_pointers/null_pointer_write_zst.stderr
Diff < left / right > :
 error: Undefined Behavior: memory access failed: null pointer is a dangling pointer (it has no provenance)
<  --> RUSTLIB/core/src/ptr/mod.rs:LL:CC
---



tests/fail/intrinsics/write_bytes_overflow.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "-Aunused" "--sysroot" "/home/user/.cache/miri/HOST" "-Zui-testing" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-1fd7bc8ace8b19d6.rmeta" "--extern" "getrandom_2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-28b21595824ceb29.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-939b1abf4012e1ce.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-02d594a0068c74d5.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-c0be874f2cc201b5.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-9b15165aa8ec1acf.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-f60083e028003752" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-193d9ef40856b5f0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-a0ff605b584f67a4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-4cab3f2a29ec0f70" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-399f873fd86b1ba6" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-de8d28cfa3a1467f" "tests/fail/intrinsics/write_bytes_overflow.rs" "--error-format=json"
actual output differed from expected tests/fail/intrinsics/write_bytes_overflow.stderr
Diff < left / right > :
 error: Undefined Behavior: overflow computing total size of `write_bytes`
<  --> RUSTLIB/core/src/intrinsics.rs:LL:CC
---
.......... (60/67)
.......    (67/67)


/checkout/src/test/rustdoc-gui/item-info-width.goml item-info-width... FAILED
[ERROR] (line 6) Error: Evaluation failed: The following errors happened (for selector `.item-info`): [expected `790px` for key `width`, found `288.891px` (or `289px`)]: for command `assert-css: (".item-info", {"width": "790px"})`

/checkout/src/test/rustdoc-gui/item-info-overflow.goml item-info-overflow... FAILED
[ERROR] (line 6) Error: Evaluation failed: scrollWidth: `890` !== `668`: for command `compare-elements-property: (".docblock.item-decl", ".item-info", ["scrollWidth"])`
[ERROR] (line 7) Error: Evaluation failed: property `scrollWidth` (`668`) isn't equal to `890` for selector `.item-info`: for command `assert-property: (".item-info", {"scrollWidth": "890"})`
[ERROR] (line 17) Error: Evaluation failed: scrollWidth: `668` !== `866`: for command `compare-elements-property: (
    "#impl-SimpleTrait-for-LongItemInfo2 .item-info",
    "#impl-SimpleTrait-for-LongItemInfo2 + .docblock",
    ["scrollWidth"],
)`
[ERROR] (line 22) Error: Evaluation failed: property `scrollWidth` (`668`) isn't equal to `866` for selector `#impl-SimpleTrait-for-LongItemInfo2 .item-info`: for command `assert-property: (
    "#impl-SimpleTrait-for-LongItemInfo2 .item-info",
    {"scrollWidth": "866"},

Build completed unsuccessfully in 0:00:23

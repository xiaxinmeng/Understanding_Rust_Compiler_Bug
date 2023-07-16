plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.081 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i.....ii.........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.46s

 finished in 2.536 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Building stage0 tool linkchecker (x86_64-unknown-linux-gnu)
   Compiling once_cell v1.4.1
   Compiling linkchecker v0.1.0 (/checkout/src/tools/linkchecker)
    Finished release [optimized] target(s) in 1.92s
std/vec/struct.Vec.html:1998: broken link - std/vec/%5BT%5D::binary_search_by
std/vec/struct.Vec.html:1998: broken link - std/vec/%5BT%5D::binary_search_by_key
std/vec/struct.Vec.html:1998: broken link - std/vec/%5BT%5D::partition_point
std/io/struct.IoSlice.html:575: broken link - std/io/%5BT%5D::binary_search_by
std/io/struct.IoSlice.html:575: broken link - std/io/%5BT%5D::binary_search_by_key
std/io/struct.IoSlice.html:575: broken link - std/io/%5BT%5D::partition_point
std/io/struct.IoSliceMut.html:975: broken link - std/io/%5BT%5D::binary_search_by
std/io/struct.IoSliceMut.html:975: broken link - std/io/%5BT%5D::binary_search_by_key
std/io/struct.IoSliceMut.html:975: broken link - std/io/%5BT%5D::partition_point
std/primitive.slice.html:975: broken link - std/%5BT%5D::binary_search_by
std/primitive.slice.html:975: broken link - std/%5BT%5D::binary_search_by_key
std/primitive.slice.html:975: broken link - std/%5BT%5D::partition_point
thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:102:9


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
expected success, got: exit code: 101

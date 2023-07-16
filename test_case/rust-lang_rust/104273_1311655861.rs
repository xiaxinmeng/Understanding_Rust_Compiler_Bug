plain
    Finished release [optimized] target(s) in 13.36s
[TIMING] tool::ToolBuild { compiler: Compiler { stage: 0, host: x86_64-apple-darwin }, target: x86_64-apple-darwin, tool: "html-checker", path: "src/tools/html-checker", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 13.397
[TIMING] tool::HtmlChecker { compiler: Compiler { stage: 0, host: x86_64-apple-darwin }, target: x86_64-apple-darwin } -- 0.000
Running HTML checker...
=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/core/sync/struct.Exclusive.html` (error code: 1) <=
line 66 column 552 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/std/array/struct.IntoIter.html` (error code: 1) <=
line 93 column 443 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/std/slice/fn.from_mut.html` (error code: 1) <=
line 1 column 4612 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/std/slice/struct.IterMut.html` (error code: 1) <=
line 62 column 443 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/std/io/struct.BorrowedCursor.html` (error code: 1) <=
line 20 column 432 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/std/io/struct.IoSliceMut.html` (error code: 1) <=
line 1920 column 440 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 2108 column 4308 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/std/simd/struct.Simd.html` (error code: 1) <=
line 225 column 55460 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/std/vec/struct.IntoIter.html` (error code: 1) <=
line 14 column 544 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/std/vec/struct.Vec.html` (error code: 1) <=
line 657 column 538 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 3015 column 440 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 3203 column 1626 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 3203 column 3346 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 3203 column 6503 - Warning: unescaped & or unknown entity "&Vec" (UNKNOWN_ENTITY)
line 3203 column 9633 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 3205 column 3107 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 3308 column 6549 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/std/ops/struct.RangeFull.html` (error code: 1) <=
line 34 column 765 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/std/ops/struct.RangeToInclusive.html` (error code: 1) <=
line 49 column 765 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/std/ops/struct.RangeFrom.html` (error code: 1) <=
line 79 column 765 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 98 column 1255 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/std/ops/struct.RangeTo.html` (error code: 1) <=
line 49 column 765 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/std/ops/struct.Range.html` (error code: 1) <=
line 94 column 769 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 129 column 1239 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/std/ops/struct.RangeInclusive.html` (error code: 1) <=
line 133 column 769 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 153 column 1266 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/std/primitive.slice.html` (error code: 1) <=
line 2032 column 434 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 2229 column 1374 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 2229 column 2673 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 2229 column 4648 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 2229 column 6275 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 2229 column 34273 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 2229 column 35862 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 2237 column 1479 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 2363 column 754 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 2373 column 750 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 2383 column 754 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 2393 column 754 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 2403 column 754 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 2413 column 754 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/std/string/struct.String.html` (error code: 1) <=
line 732 column 629 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 936 column 543 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/std/primitive.str.html` (error code: 1) <=
line 119 column 537 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 1535 column 1223 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 1554 column 1231 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 1591 column 1250 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/std/sync/atomic/struct.AtomicU8.html` (error code: 1) <=
line 32 column 562 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/std/sync/struct.Exclusive.html` (error code: 1) <=
line 66 column 547 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/std/collections/vec_deque/struct.VecDeque.html` (error code: 1) <=
line 593 column 573 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/std/collections/struct.VecDeque.html` (error code: 1) <=
line 593 column 567 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/std/mem/union.MaybeUninit.html` (error code: 1) <=
line 644 column 684 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)

=> Errors for `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/doc/std/primitive.array.html` (error code: 1) <=
line 229 column 534 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 349 column 3285 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
line 349 column 10212 - Warning: unescaped & or unknown entity "&mut" (UNKNOWN_ENTITY)
Error: "HTML check failed: 54 errors"
Done! Read 30910 files...
Build completed unsuccessfully in 2:22:24

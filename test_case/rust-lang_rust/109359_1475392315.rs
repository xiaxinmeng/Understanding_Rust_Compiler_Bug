plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.87
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:353:5
    |
353 |       simd_shuffle!(a, cmpss(b, a, 1), [4, 1, 2, 3])


error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:367:5
    |
367 |       simd_shuffle!(a, cmpss(b, a, 2), [4, 1, 2, 3])


error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:423:5
    |
423 |       simd_shuffle!(a, cmpss(b, a, 5), [4, 1, 2, 3])


error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:437:5
    |
437 |       simd_shuffle!(a, cmpss(b, a, 6), [4, 1, 2, 3])


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
1022 | |         ],
1023 | |     )
     | |_____- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:1035:5
     |
1035 |       simd_shuffle!(a, b, [2, 6, 3, 7])


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:1047:5
     |
1047 |       simd_shuffle!(a, b, [0, 4, 1, 5])


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:1060:5
     |
1060 |       simd_shuffle!(a, b, [6, 7, 2, 3])


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:1072:5
     |
1072 |       simd_shuffle!(a, b, [0, 1, 4, 5])


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:1201:5
     |
1201 |       simd_shuffle!(a, a, [3, 2, 1, 0])


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:1253:21
     |
1253 |       let b: __m128 = simd_shuffle!(a, a, [0, 0, 0, 0]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:1329:21
     |
1329 |       let b: __m128 = simd_shuffle!(a, a, [3, 2, 1, 0]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:1347:5
     |
1347 |       simd_shuffle!(a, b, [4, 1, 2, 3])


error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:444:27
    |
444 |       transmute::<i8x16, _>(simd_shuffle!(
445 | |         zero,
445 | |         zero,
446 | |         a.as_i8x16(),
...   |
464 | |         ],
465 | |     ))
    | |_____- in this macro invocation
    | |_____- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:647:20
    |
647 |       let x: i8x16 = simd_shuffle!(
    |  ____________________-
648 | |         a.as_i8x16(),
650 | |         [
...   |
667 | |         ],
668 | |     );
668 | |     );
    | |_____- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:907:33
    |
907 |       simd_cast::<i32x2, __m128d>(simd_shuffle!(a, a, [0, 1]))


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1315:20
     |
1315 |       let r: i64x2 = simd_shuffle!(a.as_i64x2(), zero.as_i64x2(), [0, 2]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1405:20
     |
1405 |       let x: i32x4 = simd_shuffle!(
1406 | |         a,
1407 | |         a,
1408 | |         [
...    |
...    |
1413 | |         ],
1414 | |     );
     | |_____- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1433:20
     |
1433 |       let x: i16x8 = simd_shuffle!(
1434 | |         a,
1435 | |         a,
1436 | |         [
...    |
...    |
1445 | |         ],
1446 | |     );
     | |_____- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1465:20
     |
1465 |       let x: i16x8 = simd_shuffle!(
1466 | |         a,
1467 | |         a,
1468 | |         [
...    |
...    |
1477 | |         ],
1478 | |     );
     | |_____- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1490:27
     |
1490 |       transmute::<i8x16, _>(simd_shuffle!(
     |  ___________________________-
1491 | |         a.as_i8x16(),
1492 | |         b.as_i8x16(),
1493 | |         [8, 24, 9, 25, 10, 26, 11, 27, 12, 28, 13, 29, 14, 30, 15, 31],
     | |_____- in this macro invocation


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1505:13
     |
1505 |       let x = simd_shuffle!(a.as_i16x8(), b.as_i16x8(), [4, 12, 5, 13, 6, 14, 7, 15]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1517:27
     |
1517 |       transmute::<i32x4, _>(simd_shuffle!(a.as_i32x4(), b.as_i32x4(), [2, 6, 3, 7]))


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1528:27
     |
1528 |       transmute::<i64x2, _>(simd_shuffle!(a.as_i64x2(), b.as_i64x2(), [1, 3]))


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1539:27
     |
1539 |       transmute::<i8x16, _>(simd_shuffle!(
     |  ___________________________-
1540 | |         a.as_i8x16(),
1541 | |         b.as_i8x16(),
1542 | |         [0, 16, 1, 17, 2, 18, 3, 19, 4, 20, 5, 21, 6, 22, 7, 23],
     | |_____- in this macro invocation


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1554:13
     |
1554 |       let x = simd_shuffle!(a.as_i16x8(), b.as_i16x8(), [0, 8, 1, 9, 2, 10, 3, 11]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1566:27
     |
1566 |       transmute::<i32x4, _>(simd_shuffle!(a.as_i32x4(), b.as_i32x4(), [0, 4, 1, 5]))


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
---
129 | |         ],
130 | |     )
    | |_____- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
156 | |         ],
157 | |     )
    | |_____- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
474 | |         ],
475 | |     )
    | |_____- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
501 | |         ],
502 | |     )
    | |_____- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:933:5
    |
933 | /     simd_shuffle!(
934 | |         a,
935 | |         _mm256_undefined_ps(),
936 | |         [[0, 1, 2, 3], [4, 5, 6, 7]][IMM1 as usize],
    | |_____- in this macro invocation


error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:954:5
    |
954 |       simd_shuffle!(a, _mm256_undefined_pd(), [[0, 1], [2, 3]][IMM1 as usize])


error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:970:22
    |
970 |       let dst: i64x2 = simd_shuffle!(
    |  ______________________-
971 | |         a.as_i64x4(),
972 | |         _mm256_undefined_si256().as_i64x4(),
973 | |         [[0, 1], [2, 3]][IMM1 as usize],
    | |_____- in this macro invocation


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1036:5
     |
1036 | /     simd_shuffle!(
1037 | |         a,
1038 | |         _mm256_undefined_ps(),
...    |
1048 | |         ],
1049 | |     )
     | |_____- in this macro invocation
     | |_____- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1063:5
     |
1063 | /     simd_shuffle!(
1064 | |         a,
1065 | |         _mm_undefined_ps(),
...    |
1071 | |         ],
1072 | |     )
     | |_____- in this macro invocation
     | |_____- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1110:5
     |
1110 | /     simd_shuffle!(
1111 | |         a,
1112 | |         _mm256_undefined_pd(),
...    |
1118 | |         ],
1119 | |     )
     | |_____- in this macro invocation
     | |_____- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1133:5
     |
1133 | /     simd_shuffle!(
1134 | |         a,
1135 | |         _mm_undefined_pd(),
1136 | |         [(IMM2 as u32) & 1, (IMM2 as u32 >> 1) & 1],
     | |_____- in this macro invocation


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1260:5
     |
1260 | /     simd_shuffle!(
1261 | |         a,
1262 | |         _mm256_castps128_ps256(b),
1263 | |         [[8, 9, 10, 11, 4, 5, 6, 7], [0, 1, 2, 3, 8, 9, 10, 11]][IMM1 as usize],
     | |_____- in this macro invocation


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1282:5
     |
1282 | /     simd_shuffle!(
1283 | |         a,
1284 | |         _mm256_castpd128_pd256(b),
1285 | |         [[4, 5, 2, 3], [0, 1, 4, 5]][IMM1 as usize],
     | |_____- in this macro invocation


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1303:22
     |
1303 |       let dst: i64x4 = simd_shuffle!(
     |  ______________________-
1304 | |         a.as_i64x4(),
1305 | |         _mm256_castsi128_si256(b).as_i64x4(),
1306 | |         [[4, 5, 2, 3], [0, 1, 4, 5]][IMM1 as usize],
     | |_____- in this macro invocation


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1642:5
     |
1642 |       simd_shuffle!(a, a, [1, 1, 3, 3, 5, 5, 7, 7])


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1654:5
     |
1654 |       simd_shuffle!(a, a, [0, 0, 2, 2, 4, 4, 6, 6])


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1666:5
     |
1666 |       simd_shuffle!(a, a, [0, 0, 2, 2])


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1759:5
     |
1759 |       simd_shuffle!(a, b, [1, 5, 3, 7])


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1771:5
     |
1771 |       simd_shuffle!(a, b, [2, 10, 3, 11, 6, 14, 7, 15])


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1783:5
     |
1783 |       simd_shuffle!(a, b, [0, 4, 2, 6])


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1795:5
     |
1795 |       simd_shuffle!(a, b, [0, 8, 1, 9, 4, 12, 5, 13])


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:2574:5
     |
2574 |       simd_shuffle!(a, a, [0, 1, 2, 3])


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:2586:5
     |
2586 |       simd_shuffle!(a, a, [0, 1])


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:2599:22
     |
2599 |       let dst: i64x2 = simd_shuffle!(a, a, [0, 1]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:2614:5
     |
2614 |       simd_shuffle!(a, a, [0, 1, 2, 3, 0, 0, 0, 0])


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:2628:5
     |
2628 |       simd_shuffle!(a, a, [0, 1, 0, 0])


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
---
184 | |             ],
185 | |         ),
    | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
192 | |             ],
193 | |         ),
    | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
200 | |             ],
201 | |         ),
    | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
208 | |             ],
209 | |         ),
    | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
216 | |             ],
217 | |         ),
    | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
224 | |             ],
225 | |         ),
    | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
232 | |             ],
233 | |         ),
    | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
240 | |             ],
241 | |         ),
    | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
248 | |             ],
249 | |         ),
    | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
256 | |             ],
257 | |         ),
    | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
264 | |             ],
265 | |         ),
    | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
272 | |             ],
273 | |         ),
    | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
280 | |             ],
281 | |         ),
    | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
288 | |             ],
289 | |         ),
    | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
296 | |             ],
297 | |         ),
    | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
304 | |             ],
305 | |         ),
    | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:373:20
    |
373 |       let r: i32x4 = simd_shuffle!(
374 | |         a,
375 | |         b,
376 | |         [
...   |
...   |
381 | |         ],
382 | |     );
    | |_____- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:398:20
    |
398 |       let r: i32x8 = simd_shuffle!(
399 | |         a,
400 | |         b,
401 | |         [
...   |
...   |
410 | |         ],
411 | |     );
    | |_____- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:428:21
    |
428 |       let r: i16x16 = simd_shuffle!(
429 | |         a,
430 | |         b,
431 | |         [
...   |
...   |
448 | |         ],
449 | |     );
    | |_____- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:474:15
    |
474 |       let ret = simd_shuffle!(a.as_i8x16(), zero.as_i8x16(), [0_u32; 16]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:488:15
    |
488 |       let ret = simd_shuffle!(a.as_i8x16(), zero.as_i8x16(), [0_u32; 32]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:504:15
    |
504 |       let ret = simd_shuffle!(a.as_i32x4(), zero.as_i32x4(), [0_u32; 4]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:520:15
    |
520 |       let ret = simd_shuffle!(a.as_i32x4(), zero.as_i32x4(), [0_u32; 8]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:535:15
    |
535 |       let ret = simd_shuffle!(a.as_i64x2(), a.as_i64x2(), [0_u32; 2]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:548:15
    |
548 |       let ret = simd_shuffle!(a.as_i64x2(), a.as_i64x2(), [0_u32; 4]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:561:5
    |
561 |       simd_shuffle!(a, _mm_setzero_pd(), [0_u32; 2])


error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:573:5
    |
573 |       simd_shuffle!(a, _mm_setzero_pd(), [0_u32; 4])


error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:587:15
    |
587 |       let ret = simd_shuffle!(a.as_i64x2(), zero.as_i64x2(), [0, 1, 0, 1]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:600:5
    |
600 |       simd_shuffle!(a, _mm_setzero_ps(), [0_u32; 4])


error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:612:5
    |
612 |       simd_shuffle!(a, _mm_setzero_ps(), [0_u32; 8])


error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:625:15
    |
625 |       let ret = simd_shuffle!(a.as_i16x8(), zero.as_i16x8(), [0_u32; 8]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:639:15
    |
639 |       let ret = simd_shuffle!(a.as_i16x8(), zero.as_i16x8(), [0_u32; 16]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:751:22
    |
751 |       let v64: i16x4 = simd_shuffle!(a, a, [0, 1, 2, 3]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:786:21
    |
786 |       let v64: i8x8 = simd_shuffle!(a, a, [0, 1, 2, 3, 4, 5, 6, 7]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:799:21
    |
799 |       let v32: i8x4 = simd_shuffle!(a, a, [0, 1, 2, 3]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:825:22
    |
825 |       let v64: u16x4 = simd_shuffle!(a, a, [0, 1, 2, 3]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:861:21
    |
861 |       let v64: u8x8 = simd_shuffle!(a, a, [0, 1, 2, 3, 4, 5, 6, 7]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:875:21
    |
875 |       let v32: u8x4 = simd_shuffle!(a, a, [0, 1, 2, 3]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ `_` not allowed here
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:894:22
    |
894 |       let dst: i64x2 = simd_shuffle!(a, b, [[0, 1], [2, 3]][IMM1 as usize]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
---
2825 | |             ],
2826 | |         ),
     | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
2833 | |             ],
2834 | |         ),
     | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
2841 | |             ],
2842 | |         ),
     | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
2849 | |             ],
2850 | |         ),
     | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
2857 | |             ],
2858 | |         ),
     | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
2865 | |             ],
2866 | |         ),
     | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
2873 | |             ],
2874 | |         ),
     | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
2881 | |             ],
2882 | |         ),
     | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
2889 | |             ],
2890 | |         ),
     | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
2897 | |             ],
2898 | |         ),
     | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
2905 | |             ],
2906 | |         ),
     | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
2913 | |             ],
2914 | |         ),
     | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
2921 | |             ],
2922 | |         ),
     | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
2929 | |             ],
2930 | |         ),
     | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
2937 | |             ],
2938 | |         ),
     | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
2945 | |             ],
2946 | |         ),
     | |_________- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:3217:20
     |
3217 |       let r: i8x32 = simd_shuffle!(a.as_i8x32(), b.as_i8x32(), [
3218 | |             8, 40, 9, 41, 10, 42, 11, 43,
3219 | |             12, 44, 13, 45, 14, 46, 15, 47,
3220 | |             24, 56, 25, 57, 26, 58, 27, 59,
3221 | |             28, 60, 29, 61, 30, 62, 31, 63,
3221 | |             28, 60, 29, 61, 30, 62, 31, 63,
3222 | |     ]);
     | |______- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:3270:20
     |
3270 |       let r: i8x32 = simd_shuffle!(a.as_i8x32(), b.as_i8x32(), [
3271 | |         0, 32, 1, 33, 2, 34, 3, 35,
3272 | |         4, 36, 5, 37, 6, 38, 7, 39,
3273 | |         16, 48, 17, 49, 18, 50, 19, 51,
3274 | |         20, 52, 21, 53, 22, 54, 23, 55,
3274 | |         20, 52, 21, 53, 22, 54, 23, 55,
3275 | |     ]);
     | |______- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:3318:21
     |
3318 |       let r: i16x16 = simd_shuffle!(
     |  _____________________-
3319 | |         a.as_i16x16(),
3320 | |         b.as_i16x16(),
3321 | |         [4, 20, 5, 21, 6, 22, 7, 23, 12, 28, 13, 29, 14, 30, 15, 31],
     | |_____- in this macro invocation


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:3366:21
     |
3366 |       let r: i16x16 = simd_shuffle!(
     |  _____________________-
3367 | |         a.as_i16x16(),
3368 | |         b.as_i16x16(),
3369 | |         [0, 16, 1, 17, 2, 18, 3, 19, 8, 24, 9, 25, 10, 26, 11, 27],
     | |_____- in this macro invocation


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:3407:20
     |
3407 |       let r: i32x8 = simd_shuffle!(a.as_i32x8(), b.as_i32x8(), [2, 10, 3, 11, 6, 14, 7, 15]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:3444:20
     |
3444 |       let r: i32x8 = simd_shuffle!(a.as_i32x8(), b.as_i32x8(), [0, 8, 1, 9, 4, 12, 5, 13]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:3481:20
     |
3481 |       let r: i64x4 = simd_shuffle!(a.as_i64x4(), b.as_i64x4(), [1, 5, 3, 7]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ `_` not allowed here
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:3518:20
     |
3518 |       let r: i64x4 = simd_shuffle!(a.as_i64x4(), b.as_i64x4(), [0, 4, 2, 6]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ `_` not allowed here
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:10564:5
10564 | /     simd_shuffle!(
10565 | |         r,
10565 | |         r,
10566 | |         _mm256_setzero_ps().as_f32x8(),
10567 | |         [0, 1, 2, 3, 4, 5, 6, 7, 8, 8, 8, 8, 8, 8, 8, 8],
      | |_____- in this macro invocation


error: in expressions, `_` can only be used on the left-hand side of an assignment
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ `_` not allowed here
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:10584:5
10584 | /     simd_shuffle!(
10585 | |         r,
10585 | |         r,
10586 | |         _mm256_setzero_ps().as_f32x8(),
10587 | |         [0, 1, 2, 3, 4, 5, 6, 7, 8, 8, 8, 8, 8, 8, 8, 8],
      | |_____- in this macro invocation


error: in expressions, `_` can only be used on the left-hand side of an assignment
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ `_` not allowed here
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:10679:21
      |
10679 |       let v64: i8x8 = simd_shuffle!(a, a, [0, 1, 2, 3, 4, 5, 6, 7]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ `_` not allowed here
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:10840:21
      |
10840 |       let v64: u8x8 = simd_shuffle!(a, a, [0, 1, 2, 3, 4, 5, 6, 7]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ `_` not allowed here
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:11663:22
      |
11663 |       let u64: u32x2 = simd_shuffle!(a, a, [0, 1]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ `_` not allowed here
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:11698:23
      |
11698 |       let v256: i32x8 = simd_shuffle!(v2, v2, [0, 1, 2, 3, 4, 5, 6, 7]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ `_` not allowed here
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:11721:23
      |
11721 |       let v256: u32x8 = simd_shuffle!(v2, v2, [0, 1, 2, 3, 4, 5, 6, 7]);


error: in expressions, `_` can only be used on the left-hand side of an assignment
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ `_` not allowed here
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:19370:5
19370 | /     simd_shuffle!(
19371 | |         a,
19372 | |         a,
19373 | |         [
19373 | |         [
...     |
19390 | |         ],
19391 | |     )
      | |_____- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ `_` not allowed here
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:19488:5
19488 | /     simd_shuffle!(
19489 | |         a,
19490 | |         a,
19491 | |         [
19491 | |         [
...     |
19500 | |         ],
19501 | |     )
      | |_____- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ `_` not allowed here
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:19606:5
19606 | /     simd_shuffle!(
19607 | |         a,
19608 | |         a,
19609 | |         [
19609 | |         [
...     |
19618 | |         ],
19619 | |     )
      | |_____- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ `_` not allowed here
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:19662:5
19662 | /     simd_shuffle!(
19663 | |         a,
19664 | |         a,
19665 | |         [
19665 | |         [
...     |
19670 | |         ],
19671 | |     )
      | |_____- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ `_` not allowed here
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:19714:5
19714 | /     simd_shuffle!(
19715 | |         a,
19716 | |         a,
19717 | |         [
19717 | |         [
...     |
19726 | |         ],
19727 | |     )
      | |_____- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ `_` not allowed here
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:19768:5
19768 | /     simd_shuffle!(
19769 | |         a,
19770 | |         a,
19771 | |         [
19771 | |         [
...     |
19776 | |         ],
19777 | |     )
      | |_____- in this macro invocation

error: in expressions, `_` can only be used on the left-hand side of an assignment
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ `_` not allowed here
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `4`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:1035:5
     |
1035 |       simd_shuffle!(a, b, [2, 6, 3, 7])
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `4`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:1047:5
     |
1047 |       simd_shuffle!(a, b, [0, 4, 1, 5])
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `4`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:1060:5
     |
1060 |       simd_shuffle!(a, b, [6, 7, 2, 3])
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `4`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:1072:5
     |
1072 |       simd_shuffle!(a, b, [0, 1, 4, 5])
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `4`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:1201:5
     |
1201 |       simd_shuffle!(a, a, [3, 2, 1, 0])
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `4`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:1253:21
     |
1253 |       let b: __m128 = simd_shuffle!(a, a, [0, 0, 0, 0]);
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `4`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:1329:21
     |
1329 |       let b: __m128 = simd_shuffle!(a, a, [3, 2, 1, 0]);
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `4`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:1347:5
     |
1347 |       simd_shuffle!(a, b, [4, 1, 2, 3])
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `16`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:444:27
    |
444 |       transmute::<i8x16, _>(simd_shuffle!(
445 | |         zero,
445 | |         zero,
446 | |         a.as_i8x16(),
...   |
464 | |         ],
465 | |     ))
    | |_____- in this macro invocation
    | |_____- in this macro invocation
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `16`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:647:20
    |
647 |       let x: i8x16 = simd_shuffle!(
    |  ____________________-
648 | |         a.as_i8x16(),
650 | |         [
...   |
667 | |         ],
668 | |     );
668 | |     );
    | |_____- in this macro invocation
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `2`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:907:33
    |
907 |       simd_cast::<i32x2, __m128d>(simd_shuffle!(a, a, [0, 1]))
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `2`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1315:20
     |
1315 |       let r: i64x2 = simd_shuffle!(a.as_i64x2(), zero.as_i64x2(), [0, 2]);
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `4`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1405:20
     |
1405 |       let x: i32x4 = simd_shuffle!(
1406 | |         a,
1407 | |         a,
1408 | |         [
...    |
...    |
1413 | |         ],
1414 | |     );
     | |_____- in this macro invocation
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `8`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1433:20
     |
1433 |       let x: i16x8 = simd_shuffle!(
1434 | |         a,
1435 | |         a,
1436 | |         [
...    |
...    |
1445 | |         ],
1446 | |     );
     | |_____- in this macro invocation
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `8`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1465:20
     |
1465 |       let x: i16x8 = simd_shuffle!(
1466 | |         a,
1467 | |         a,
1468 | |         [
...    |
...    |
1477 | |         ],
1478 | |     );
     | |_____- in this macro invocation
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `16`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1490:27
     |
1490 |       transmute::<i8x16, _>(simd_shuffle!(
     |  ___________________________-
1491 | |         a.as_i8x16(),
1492 | |         b.as_i8x16(),
1493 | |         [8, 24, 9, 25, 10, 26, 11, 27, 12, 28, 13, 29, 14, 30, 15, 31],
     | |_____- in this macro invocation
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `8`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1505:13
     |
1505 |       let x = simd_shuffle!(a.as_i16x8(), b.as_i16x8(), [4, 12, 5, 13, 6, 14, 7, 15]);
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `4`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1517:27
     |
1517 |       transmute::<i32x4, _>(simd_shuffle!(a.as_i32x4(), b.as_i32x4(), [2, 6, 3, 7]))
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `2`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1528:27
     |
1528 |       transmute::<i64x2, _>(simd_shuffle!(a.as_i64x2(), b.as_i64x2(), [1, 3]))
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `16`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs:1539:27
     |
1539 |       transmute::<i8x16, _>(simd_shuffle!(
     |  ___________________________-
---
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `8`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `4`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `8`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `8`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1036:5
     |
1036 | /     simd_shuffle!(
1037 | |         a,
1038 | |         _mm256_undefined_ps(),
...    |
1048 | |         ],
1049 | |     )
     | |_____- in this macro invocation
     | |_____- in this macro invocation
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `4`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1063:5
     |
1063 | /     simd_shuffle!(
1064 | |         a,
1065 | |         _mm_undefined_ps(),
...    |
1071 | |         ],
1072 | |     )
     | |_____- in this macro invocation
     | |_____- in this macro invocation
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `4`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1110:5
     |
1110 | /     simd_shuffle!(
1111 | |         a,
1112 | |         _mm256_undefined_pd(),
...    |
1118 | |         ],
1119 | |     )
     | |_____- in this macro invocation
     | |_____- in this macro invocation
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `2`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1133:5
     |
1133 | /     simd_shuffle!(
1134 | |         a,
1135 | |         _mm_undefined_pd(),
1136 | |         [(IMM2 as u32) & 1, (IMM2 as u32 >> 1) & 1],
     | |_____- in this macro invocation
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `8`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1642:5
     |
1642 |       simd_shuffle!(a, a, [1, 1, 3, 3, 5, 5, 7, 7])
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `8`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1654:5
     |
1654 |       simd_shuffle!(a, a, [0, 0, 2, 2, 4, 4, 6, 6])
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `4`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1666:5
     |
1666 |       simd_shuffle!(a, a, [0, 0, 2, 2])
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `4`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1759:5
     |
1759 |       simd_shuffle!(a, b, [1, 5, 3, 7])
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `8`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1771:5
     |
1771 |       simd_shuffle!(a, b, [2, 10, 3, 11, 6, 14, 7, 15])
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `4`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1783:5
     |
1783 |       simd_shuffle!(a, b, [0, 4, 2, 6])
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `8`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1795:5
     |
1795 |       simd_shuffle!(a, b, [0, 8, 1, 9, 4, 12, 5, 13])
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `4`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:2574:5
     |
2574 |       simd_shuffle!(a, a, [0, 1, 2, 3])
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `2`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:2586:5
     |
2586 |       simd_shuffle!(a, a, [0, 1])
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `2`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:2599:22
     |
2599 |       let dst: i64x2 = simd_shuffle!(a, a, [0, 1]);
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `8`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:2614:5
     |
2614 |       simd_shuffle!(a, a, [0, 1, 2, 3, 0, 0, 0, 0])
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `4`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:2628:5
     |
2628 |       simd_shuffle!(a, a, [0, 1, 0, 0])
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `4`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:2643:22
     |
2643 |       let dst: i64x4 = simd_shuffle!(a, a, [0, 1, 0, 0]);
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `8`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:2658:5
     |
2658 |       simd_shuffle!(a, _mm_setzero_ps(), [0, 1, 2, 3, 4, 5, 6, 7])
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `4`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:2673:22
     |
2673 |       let dst: i64x4 = simd_shuffle!(a.as_i64x2(), b, [0, 1, 2, 3]);
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `4`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:2689:5
     |
2689 |       simd_shuffle!(a, _mm_setzero_pd(), [0, 1, 2, 3])
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `8`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `32`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `32`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `32`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `32`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `32`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `32`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `32`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `32`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `32`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `32`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `32`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `32`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `32`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `32`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `32`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
---
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `4`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:373:20
    |
373 |       let r: i32x4 = simd_shuffle!(
374 | |         a,
375 | |         b,
376 | |         [
...   |
...   |
381 | |         ],
382 | |     );
    | |_____- in this macro invocation
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `8`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:398:20
    |
398 |       let r: i32x8 = simd_shuffle!(
399 | |         a,
400 | |         b,
401 | |         [
...   |
...   |
410 | |         ],
411 | |     );
    | |_____- in this macro invocation
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `16`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:428:21
    |
428 |       let r: i16x16 = simd_shuffle!(
429 | |         a,
430 | |         b,
431 | |         [
...   |
...   |
448 | |         ],
449 | |     );
    | |_____- in this macro invocation
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `16`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:474:15
    |
474 |       let ret = simd_shuffle!(a.as_i8x16(), zero.as_i8x16(), [0_u32; 16]);
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `32`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:488:15
    |
488 |       let ret = simd_shuffle!(a.as_i8x16(), zero.as_i8x16(), [0_u32; 32]);
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `4`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:504:15
    |
504 |       let ret = simd_shuffle!(a.as_i32x4(), zero.as_i32x4(), [0_u32; 4]);
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `8`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:520:15
    |
520 |       let ret = simd_shuffle!(a.as_i32x4(), zero.as_i32x4(), [0_u32; 8]);
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `2`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:535:15
    |
535 |       let ret = simd_shuffle!(a.as_i64x2(), a.as_i64x2(), [0_u32; 2]);
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `4`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:548:15
    |
548 |       let ret = simd_shuffle!(a.as_i64x2(), a.as_i64x2(), [0_u32; 4]);
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `2`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:561:5
    |
561 |       simd_shuffle!(a, _mm_setzero_pd(), [0_u32; 2])
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `4`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:573:5
    |
573 |       simd_shuffle!(a, _mm_setzero_pd(), [0_u32; 4])
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `4`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:587:15
    |
587 |       let ret = simd_shuffle!(a.as_i64x2(), zero.as_i64x2(), [0, 1, 0, 1]);
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `4`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:600:5
    |
600 |       simd_shuffle!(a, _mm_setzero_ps(), [0_u32; 4])
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `8`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:612:5
    |
612 |       simd_shuffle!(a, _mm_setzero_ps(), [0_u32; 8])
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `8`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:625:15
    |
625 |       let ret = simd_shuffle!(a.as_i16x8(), zero.as_i16x8(), [0_u32; 8]);
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `16`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:639:15
    |
639 |       let ret = simd_shuffle!(a.as_i16x8(), zero.as_i16x8(), [0_u32; 16]);
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `4`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:751:22
    |
751 |       let v64: i16x4 = simd_shuffle!(a, a, [0, 1, 2, 3]);
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `8`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:786:21
    |
786 |       let v64: i8x8 = simd_shuffle!(a, a, [0, 1, 2, 3, 4, 5, 6, 7]);
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `4`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:799:21
    |
799 |       let v32: i8x4 = simd_shuffle!(a, a, [0, 1, 2, 3]);
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `4`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:825:22
    |
825 |       let v64: u16x4 = simd_shuffle!(a, a, [0, 1, 2, 3]);
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
    | |                              ^ help: consider specifying the array length: `8`
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    |
    |
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `32`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `32`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `32`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `32`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `32`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `32`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `32`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `32`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `32`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `32`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `32`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `32`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `32`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `32`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `32`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `32`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:3217:20
     |
3217 |       let r: i8x32 = simd_shuffle!(a.as_i8x32(), b.as_i8x32(), [
3218 | |             8, 40, 9, 41, 10, 42, 11, 43,
3219 | |             12, 44, 13, 45, 14, 46, 15, 47,
3220 | |             24, 56, 25, 57, 26, 58, 27, 59,
3221 | |             28, 60, 29, 61, 30, 62, 31, 63,
3221 | |             28, 60, 29, 61, 30, 62, 31, 63,
3222 | |     ]);
     | |______- in this macro invocation
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `32`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:3270:20
     |
3270 |       let r: i8x32 = simd_shuffle!(a.as_i8x32(), b.as_i8x32(), [
3271 | |         0, 32, 1, 33, 2, 34, 3, 35,
3272 | |         4, 36, 5, 37, 6, 38, 7, 39,
3273 | |         16, 48, 17, 49, 18, 50, 19, 51,
3274 | |         20, 52, 21, 53, 22, 54, 23, 55,
3274 | |         20, 52, 21, 53, 22, 54, 23, 55,
3275 | |     ]);
     | |______- in this macro invocation
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `16`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:3318:21
     |
3318 |       let r: i16x16 = simd_shuffle!(
     |  _____________________-
3319 | |         a.as_i16x16(),
3320 | |         b.as_i16x16(),
3321 | |         [4, 20, 5, 21, 6, 22, 7, 23, 12, 28, 13, 29, 14, 30, 15, 31],
     | |_____- in this macro invocation
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `16`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:3366:21
     |
3366 |       let r: i16x16 = simd_shuffle!(
     |  _____________________-
3367 | |         a.as_i16x16(),
3368 | |         b.as_i16x16(),
3369 | |         [0, 16, 1, 17, 2, 18, 3, 19, 8, 24, 9, 25, 10, 26, 11, 27],
     | |_____- in this macro invocation
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `8`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:3407:20
     |
3407 |       let r: i32x8 = simd_shuffle!(a.as_i32x8(), b.as_i32x8(), [2, 10, 3, 11, 6, 14, 7, 15]);
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `8`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:3444:20
     |
3444 |       let r: i32x8 = simd_shuffle!(a.as_i32x8(), b.as_i32x8(), [0, 8, 1, 9, 4, 12, 5, 13]);
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `4`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:3481:20
     |
3481 |       let r: i64x4 = simd_shuffle!(a.as_i64x4(), b.as_i64x4(), [1, 5, 3, 7]);
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `4`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:3518:20
     |
3518 |       let r: i64x4 = simd_shuffle!(a.as_i64x4(), b.as_i64x4(), [0, 4, 2, 6]);
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `16`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:10564:5
10564 | /     simd_shuffle!(
10565 | |         r,
10565 | |         r,
10566 | |         _mm256_setzero_ps().as_f32x8(),
10567 | |         [0, 1, 2, 3, 4, 5, 6, 7, 8, 8, 8, 8, 8, 8, 8, 8],
      | |_____- in this macro invocation
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `16`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:10584:5
10584 | /     simd_shuffle!(
10585 | |         r,
10585 | |         r,
10586 | |         _mm256_setzero_ps().as_f32x8(),
10587 | |         [0, 1, 2, 3, 4, 5, 6, 7, 8, 8, 8, 8, 8, 8, 8, 8],
      | |_____- in this macro invocation
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `8`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:10679:21
      |
10679 |       let v64: i8x8 = simd_shuffle!(a, a, [0, 1, 2, 3, 4, 5, 6, 7]);
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `8`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:10840:21
      |
10840 |       let v64: u8x8 = simd_shuffle!(a, a, [0, 1, 2, 3, 4, 5, 6, 7]);
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `2`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:11663:22
      |
11663 |       let u64: u32x2 = simd_shuffle!(a, a, [0, 1]);
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `8`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:11698:23
      |
11698 |       let v256: i32x8 = simd_shuffle!(v2, v2, [0, 1, 2, 3, 4, 5, 6, 7]);
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `8`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:11721:23
      |
11721 |       let v256: u32x8 = simd_shuffle!(v2, v2, [0, 1, 2, 3, 4, 5, 6, 7]);
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `16`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:19370:5
19370 | /     simd_shuffle!(
19371 | |         a,
19372 | |         a,
19373 | |         [
---
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `8`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:19488:5
19488 | /     simd_shuffle!(
19489 | |         a,
19490 | |         a,
19491 | |         [
---
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `8`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:19606:5
19606 | /     simd_shuffle!(
19607 | |         a,
19608 | |         a,
19609 | |         [
---
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `4`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:19662:5
19662 | /     simd_shuffle!(
19663 | |         a,
19664 | |         a,
19665 | |         [
---
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `8`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:19714:5
19714 | /     simd_shuffle!(
19715 | |         a,
19716 | |         a,
19717 | |         [
---
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `4`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:19768:5
19768 | /     simd_shuffle!(
19769 | |         a,
19770 | |         a,
19771 | |         [
---
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `16`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:21022:21
      |
21022 |       let r: i32x16 = simd_shuffle!(
      |  _____________________-
21023 | |         a.as_i32x16(),
21024 | |         a.as_i32x16(),
...     |
21042 | |         ],
21043 | |     );
      | |_____- in this macro invocation
      | |_____- in this macro invocation
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `16`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:21158:5
21158 | /     simd_shuffle!(
21159 | |         a,
21160 | |         b,
21161 | |         [
---
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `8`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:21295:5
21295 | /     simd_shuffle!(
21296 | |         a,
21297 | |         b,
21298 | |         [
---
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `16`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:21430:21
      |
21430 |       let r: i32x16 = simd_shuffle!(
21431 | |         a,
21432 | |         b,
21433 | |         [
...     |
...     |
21450 | |         ],
21451 | |     );
      | |_____- in this macro invocation
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `8`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:21502:20
      |
21502 |       let r: i32x8 = simd_shuffle!(
21503 | |         a,
21504 | |         b,
21505 | |         [
...     |
...     |
21514 | |         ],
21515 | |     );
      | |_____- in this macro invocation
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `8`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:21566:20
      |
21566 |       let r: i64x8 = simd_shuffle!(
21567 | |         a,
21568 | |         b,
21569 | |         [
...     |
...     |
21578 | |         ],
21579 | |     );
      | |_____- in this macro invocation
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `4`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:21630:20
      |
21630 |       let r: i64x4 = simd_shuffle!(
21631 | |         a,
21632 | |         b,
21633 | |         [
...     |
...     |
21638 | |         ],
21639 | |     );
      | |_____- in this macro invocation
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `16`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:21690:21
      |
21690 |       let r: f32x16 = simd_shuffle!(
21691 | |         a,
21692 | |         b,
21693 | |         [
...     |
...     |
21710 | |         ],
21711 | |     );
      | |_____- in this macro invocation
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `8`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:21762:20
      |
21762 |       let r: f32x8 = simd_shuffle!(
21763 | |         a,
21764 | |         b,
21765 | |         [
...     |
...     |
21774 | |         ],
21775 | |     );
      | |_____- in this macro invocation
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `8`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:21826:20
      |
21826 |       let r: f64x8 = simd_shuffle!(
21827 | |         a,
21828 | |         b,
21829 | |         [
...     |
...     |
21838 | |         ],
21839 | |     );
      | |_____- in this macro invocation
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `4`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:21890:20
      |
21890 |       let r: f64x4 = simd_shuffle!(
21891 | |         a,
21892 | |         b,
21893 | |         [
...     |
...     |
21898 | |         ],
21899 | |     );
      | |_____- in this macro invocation
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `4`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:21952:14
      |
21952 |           0 => simd_shuffle!(a, _mm512_undefined_ps(), [0, 1, 2, 3]),
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `4`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:21953:14
      |
21953 |           1 => simd_shuffle!(a, _mm512_undefined_ps(), [4, 5, 6, 7]),
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `4`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:21954:14
      |
21954 |           2 => simd_shuffle!(a, _mm512_undefined_ps(), [8, 9, 10, 11]),
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `4`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:21955:14
      |
21955 |           _ => simd_shuffle!(a, _mm512_undefined_ps(), [12, 13, 14, 15]),
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `4`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:22009:14
      |
22009 |           0 => simd_shuffle!(a, _mm256_undefined_ps(), [0, 1, 2, 3]),
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `4`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:22010:14
      |
22010 |           _ => simd_shuffle!(a, _mm256_undefined_ps(), [4, 5, 6, 7]),
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `4`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:22064:14
      |
22064 |           0 => simd_shuffle!(a, _mm512_set1_epi64(0), [0, 1, 2, 3]),
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `4`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:22065:14
      |
22065 |           _ => simd_shuffle!(a, _mm512_set1_epi64(0), [4, 5, 6, 7]),
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `4`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:22119:14
      |
22119 |           0 => simd_shuffle!(a, _mm512_undefined_pd(), [0, 1, 2, 3]),
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `4`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:22120:14
      |
22120 |           _ => simd_shuffle!(a, _mm512_undefined_pd(), [4, 5, 6, 7]),
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
67    | / macro_rules! simd_shuffle {
68    | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70    | |             $x,
...     |
...     |
73    | |                 let v: [u32; _] = $idx;
      | |                              ^ help: consider specifying the array length: `4`
77    | |     }};
78    | | }
      | |_- in this expansion of `simd_shuffle!`
      |
      |
     ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:22176:14
      |
22176 |           0 => simd_shuffle!(a, undefined, [0, 1, 2, 3]),
      |
      = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
      = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
      |
67    | / macro_rules! simd_shuffle {
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9296:14
9296 |           1 => simd_shuffle!(
     |  ______________-
9297 | |             a,
9298 | |             zero,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9305:14
9305 |           2 => simd_shuffle!(
     |  ______________-
9306 | |             a,
9307 | |             zero,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9314:14
9314 |           3 => simd_shuffle!(
     |  ______________-
9315 | |             a,
9316 | |             zero,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9324:14
9324 |           4 => simd_shuffle!(
     |  ______________-
9325 | |             a,
9326 | |             zero,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9334:14
9334 |           5 => simd_shuffle!(
     |  ______________-
9335 | |             a,
9336 | |             zero,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9344:14
9344 |           6 => simd_shuffle!(
     |  ______________-
9345 | |             a,
9346 | |             zero,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9354:14
9354 |           7 => simd_shuffle!(
     |  ______________-
9355 | |             a,
9356 | |             zero,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9364:14
9364 |           8 => simd_shuffle!(
     |  ______________-
9365 | |             a,
9366 | |             zero,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9374:14
9374 |           9 => simd_shuffle!(
     |  ______________-
9375 | |             a,
9376 | |             zero,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9384:15
9384 |           10 => simd_shuffle!(
     |  _______________-
9385 | |             a,
9386 | |             zero,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9394:15
9394 |           11 => simd_shuffle!(
     |  _______________-
9395 | |             a,
9396 | |             zero,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9404:15
9404 |           12 => simd_shuffle!(
     |  _______________-
9405 | |             a,
9406 | |             zero,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9414:15
9414 |           13 => simd_shuffle!(
     |  _______________-
9415 | |             a,
9416 | |             zero,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9424:15
9424 |           14 => simd_shuffle!(
     |  _______________-
9425 | |             a,
9426 | |             zero,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9434:15
9434 |           15 => simd_shuffle!(
     |  _______________-
9435 | |             a,
9436 | |             zero,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9473:14
9473 |           0 => simd_shuffle!(
     |  ______________-
9474 | |             b,
9475 | |             a,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9482:14
9482 |           1 => simd_shuffle!(
     |  ______________-
9483 | |             b,
9484 | |             a,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9491:14
9491 |           2 => simd_shuffle!(
     |  ______________-
9492 | |             b,
9493 | |             a,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9500:14
9500 |           3 => simd_shuffle!(
     |  ______________-
9501 | |             b,
9502 | |             a,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9510:14
9510 |           4 => simd_shuffle!(
     |  ______________-
9511 | |             b,
9512 | |             a,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9520:14
9520 |           5 => simd_shuffle!(
     |  ______________-
9521 | |             b,
9522 | |             a,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9530:14
9530 |           6 => simd_shuffle!(
     |  ______________-
9531 | |             b,
9532 | |             a,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9540:14
9540 |           7 => simd_shuffle!(
     |  ______________-
9541 | |             b,
9542 | |             a,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9550:14
9550 |           8 => simd_shuffle!(
     |  ______________-
9551 | |             b,
9552 | |             a,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9560:14
9560 |           9 => simd_shuffle!(
     |  ______________-
9561 | |             b,
9562 | |             a,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9570:15
9570 |           10 => simd_shuffle!(
     |  _______________-
9571 | |             b,
9572 | |             a,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9580:15
9580 |           11 => simd_shuffle!(
     |  _______________-
9581 | |             b,
9582 | |             a,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9590:15
9590 |           12 => simd_shuffle!(
     |  _______________-
9591 | |             b,
9592 | |             a,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9600:15
9600 |           13 => simd_shuffle!(
     |  _______________-
9601 | |             b,
9602 | |             a,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9610:15
9610 |           14 => simd_shuffle!(
     |  _______________-
9611 | |             b,
9612 | |             a,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
     | |                              ^ help: consider specifying the array length: `64`
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     |
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx512bw.rs:9620:15
9620 |           15 => simd_shuffle!(
     |  _______________-
9621 | |             b,
9622 | |             a,
---
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
...   |
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    | |_- in this expansion of `simd_shuffle!`
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:954:5
    |
954 |       simd_shuffle!(a, _mm256_undefined_pd(), [[0, 1], [2, 3]][IMM1 as usize])
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
...   |
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    | |_- in this expansion of `simd_shuffle!`
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:970:22
    |
970 |       let dst: i64x2 = simd_shuffle!(
    |  ______________________-
971 | |         a.as_i64x4(),
972 | |         _mm256_undefined_si256().as_i64x4(),
973 | |         [[0, 1], [2, 3]][IMM1 as usize],
    | |_____- in this macro invocation
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
...    |
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     | |_- in this expansion of `simd_shuffle!`
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1282:5
     |
1282 | /     simd_shuffle!(
1283 | |         a,
1284 | |         _mm256_castpd128_pd256(b),
1285 | |         [[4, 5, 2, 3], [0, 1, 4, 5]][IMM1 as usize],
     | |_____- in this macro invocation
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
...    |
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     | |_- in this expansion of `simd_shuffle!`
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1303:22
     |
1303 |       let dst: i64x4 = simd_shuffle!(
     |  ______________________-
1304 | |         a.as_i64x4(),
1305 | |         _mm256_castsi128_si256(b).as_i64x4(),
1306 | |         [[4, 5, 2, 3], [0, 1, 4, 5]][IMM1 as usize],
     | |_____- in this macro invocation
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
...    |
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     | |_- in this expansion of `simd_shuffle!`
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:1716:22
     |
1716 |       let dst: i64x4 = simd_shuffle!(a, b, [[4, 5, 2, 3], [0, 1, 4, 5]][IMM1 as usize]);
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
...   |
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    | |_- in this expansion of `simd_shuffle!`
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:894:22
    |
894 |       let dst: i64x2 = simd_shuffle!(a, b, [[0, 1], [2, 3]][IMM1 as usize]);
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable


error[E0658]: using `_` for array lengths is unstable
     |
67   | / macro_rules! simd_shuffle {
67   | / macro_rules! simd_shuffle {
68   | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70   | |             $x,
...    |
...    |
73   | |                 let v: [u32; _] = $idx;
...    |
77   | |     }};
78   | | }
     | |_- in this expansion of `simd_shuffle!`
     | |_- in this expansion of `simd_shuffle!`
     |
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1260:5
     |
1260 | /     simd_shuffle!(
1261 | |         a,
1262 | |         _mm256_castps128_ps256(b),
1263 | |         [[8, 9, 10, 11, 4, 5, 6, 7], [0, 1, 2, 3, 8, 9, 10, 11]][IMM1 as usize],
     | |_____- in this macro invocation
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error[E0658]: using `_` for array lengths is unstable
    |
67  | / macro_rules! simd_shuffle {
67  | / macro_rules! simd_shuffle {
68  | |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
70  | |             $x,
...   |
...   |
73  | |                 let v: [u32; _] = $idx;
...   |
77  | |     }};
78  | | }
    | |_- in this expansion of `simd_shuffle!`
    | |_- in this expansion of `simd_shuffle!`
    |
   ::: library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:933:5
    |
933 | /     simd_shuffle!(
934 | |         a,
935 | |         _mm256_undefined_ps(),
936 | |         [[0, 1, 2, 3], [4, 5, 6, 7]][IMM1 as usize],
    | |_____- in this macro invocation
    |
    = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
    = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

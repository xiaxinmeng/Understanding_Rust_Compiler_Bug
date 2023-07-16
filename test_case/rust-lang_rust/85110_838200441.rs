plain
   Compiling libc v0.2.93
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: last argument of `simd_shuffle` is required to be a `const` item
    |
    |
353 |     simd_shuffle4(a, cmpss(b, a, 1), [4, 1, 2, 3])


error: last argument of `simd_shuffle` is required to be a `const` item
    |
    |
367 |     simd_shuffle4(a, cmpss(b, a, 2), [4, 1, 2, 3])


error: last argument of `simd_shuffle` is required to be a `const` item
    |
    |
423 |     simd_shuffle4(a, cmpss(b, a, 5), [4, 1, 2, 3])


error: last argument of `simd_shuffle` is required to be a `const` item
    |
    |
437 |     simd_shuffle4(a, cmpss(b, a, 6), [4, 1, 2, 3])


error: last argument of `simd_shuffle` is required to be a `const` item
     |
1014 | /     simd_shuffle4(
1015 | |         a,
1016 | |         b,
1016 | |         b,
1017 | |         [
...    |
1022 | |         ],
1023 | |     )
     | |_____^

error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
1035 |     simd_shuffle4(a, b, [2, 6, 3, 7])


error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
1047 |     simd_shuffle4(a, b, [0, 4, 1, 5])


error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
1060 |     simd_shuffle4(a, b, [6, 7, 2, 3])


error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
1072 |     simd_shuffle4(a, b, [0, 1, 4, 5])


error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
1204 |     simd_shuffle4(a, a, [3, 2, 1, 0])


error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
1256 |     let b: __m128 = simd_shuffle4(a, a, [0, 0, 0, 0]);


error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
1332 |     let b: __m128 = simd_shuffle4(a, a, [3, 2, 1, 0]);


error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
1350 |     simd_shuffle4(a, b, [4, 1, 2, 3])


error: last argument of `simd_shuffle` is required to be a `const` item
    |
    |
435 |       transmute(simd_shuffle16::<i8x16, i8x16>(
436 | |         zero,
436 | |         zero,
437 | |         a.as_i8x16(),
...   |
455 | |         ],
456 | |     ))
    | |_____^
    | |_____^

error: last argument of `simd_shuffle` is required to be a `const` item
    |
    |
638 |       let x: i8x16 = simd_shuffle16(
    |  ____________________^
639 | |         a.as_i8x16(),
641 | |         [
...   |
658 | |         ],
659 | |     );
659 | |     );
    | |_____^

error: last argument of `simd_shuffle` is required to be a `const` item
    |
    |
898 |     simd_cast::<i32x2, __m128d>(simd_shuffle2(a, a, [0, 1]))


error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
1306 |     let r: i64x2 = simd_shuffle2(a.as_i64x2(), zero.as_i64x2(), [0, 2]);


error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
1394 |       let x: i32x4 = simd_shuffle4(
1395 | |         a,
1396 | |         a,
1397 | |         [
...    |
...    |
1402 | |         ],
1403 | |     );
     | |_____^

error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
1422 |       let x: i16x8 = simd_shuffle8(
1423 | |         a,
1424 | |         a,
1425 | |         [
...    |
...    |
1434 | |         ],
1435 | |     );
     | |_____^

error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
1454 |       let x: i16x8 = simd_shuffle8(
1455 | |         a,
1456 | |         a,
1457 | |         [
...    |
...    |
1466 | |         ],
1467 | |     );
     | |_____^

error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
1479 |       transmute::<i8x16, _>(simd_shuffle16(
     |  ___________________________^
1480 | |         a.as_i8x16(),
1481 | |         b.as_i8x16(),
1482 | |         [8, 24, 9, 25, 10, 26, 11, 27, 12, 28, 13, 29, 14, 30, 15, 31],
     | |_____^


error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
1494 |     let x = simd_shuffle8(a.as_i16x8(), b.as_i16x8(), [4, 12, 5, 13, 6, 14, 7, 15]);


error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
1506 |     transmute::<i32x4, _>(simd_shuffle4(a.as_i32x4(), b.as_i32x4(), [2, 6, 3, 7]))


error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
1517 |     transmute::<i64x2, _>(simd_shuffle2(a.as_i64x2(), b.as_i64x2(), [1, 3]))


error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
1528 |       transmute::<i8x16, _>(simd_shuffle16(
     |  ___________________________^
1529 | |         a.as_i8x16(),
1530 | |         b.as_i8x16(),
1531 | |         [0, 16, 1, 17, 2, 18, 3, 19, 4, 20, 5, 21, 6, 22, 7, 23],
     | |_____^


error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
1543 |     let x = simd_shuffle8(a.as_i16x8(), b.as_i16x8(), [0, 8, 1, 9, 2, 10, 3, 11]);


error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
1555 |     transmute::<i32x4, _>(simd_shuffle4(a.as_i32x4(), b.as_i32x4(), [0, 4, 1, 5]))


error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
1566 |     transmute::<i64x2, _>(simd_shuffle2(a.as_i64x2(), b.as_i64x2(), [0, 2]))


error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
2522 |     let b: __m128d = simd_shuffle2(a, a, [0, 0]);


error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
2536 |     let b: __m128d = simd_shuffle2(a, a, [0, 0]);


error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
2551 |     let b: __m128d = simd_shuffle2(a, a, [1, 0]);


error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
2615 |     simd_shuffle2(a, a, [1, 0])


error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
2656 |     simd_shuffle2(a, b, [MASK as u32 & 0b1, ((MASK as u32 >> 1) & 0b1) + 2])


error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
2780 |     simd_shuffle2(a, b, [1, 3])


error: last argument of `simd_shuffle` is required to be a `const` item
     |
     |
2795 |     simd_shuffle2(a, b, [0, 2])


error: last argument of `simd_shuffle` is required to be a `const` item
    |
    |
109 |     simd_shuffle2(a, a, [0, 0])


error: last argument of `simd_shuffle` is required to be a `const` item
    |
    |
133 |     simd_shuffle4(a, a, [1, 1, 3, 3])


error: last argument of `simd_shuffle` is required to be a `const` item
    |
    |
145 |     simd_shuffle4(a, a, [0, 0, 2, 2])


error: last argument of `simd_shuffle` is required to be a `const` item
    |
    |
116 |       let r: i8x16 = simd_shuffle16(
    |  ____________________^
117 | |         b.as_i8x16(),
118 | |         a.as_i8x16(),
...   |
136 | |         ],
137 | |     );
    | |_____^
    | |_____^

error: last argument of `simd_shuffle` is required to be a `const` item
    |
    |
382 |     let a = simd_shuffle8::<_, i8x8>(a, a, [0, 1, 2, 3, 4, 5, 6, 7]);


error: last argument of `simd_shuffle` is required to be a `const` item
    |
    |
395 |     let a = simd_shuffle4::<_, i8x4>(a, a, [0, 1, 2, 3]);


error: last argument of `simd_shuffle` is required to be a `const` item
    |
    |
409 |     let a = simd_shuffle2::<_, i8x2>(a, a, [0, 1]);


error: last argument of `simd_shuffle` is required to be a `const` item
    |
    |
422 |     let a = simd_shuffle4::<_, i16x4>(a, a, [0, 1, 2, 3]);


error: last argument of `simd_shuffle` is required to be a `const` item
    |
    |
435 |     let a = simd_shuffle2::<_, i16x2>(a, a, [0, 1]);


error: last argument of `simd_shuffle` is required to be a `const` item
    |
    |
448 |     let a = simd_shuffle2::<_, i32x2>(a, a, [0, 1]);


error: last argument of `simd_shuffle` is required to be a `const` item
    |
    |
461 |     let a = simd_shuffle8::<_, u8x8>(a, a, [0, 1, 2, 3, 4, 5, 6, 7]);


error: last argument of `simd_shuffle` is required to be a `const` item
    |
    |
474 |     let a = simd_shuffle4::<_, u8x4>(a, a, [0, 1, 2, 3]);


error: last argument of `simd_shuffle` is required to be a `const` item
    |
    |
487 |     let a = simd_shuffle2::<_, u8x2>(a, a, [0, 1]);


error: last argument of `simd_shuffle` is required to be a `const` item
    |
    |
501 |     let a = simd_shuffle4::<_, u16x4>(a, a, [0, 1, 2, 3]);


error: last argument of `simd_shuffle` is required to be a `const` item
    |
    |
515 |     let a = simd_shuffle2::<_, u16x2>(a, a, [0, 1]);


error: last argument of `simd_shuffle` is required to be a `const` item
    |
    |
529 |     let a = simd_shuffle2::<_, u32x2>(a, a, [0, 1]);


error: last argument of `simd_shuffle` is required to be a `const` item
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:121:5
121 | /     simd_shuffle4(
122 | |         a,
123 | |         b,
124 | |         [
124 | |         [
...   |
129 | |         ],
130 | |     )
    | |_____^

error: last argument of `simd_shuffle` is required to be a `const` item
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:144:5
144 | /     simd_shuffle8(
145 | |         a,
146 | |         b,
147 | |         [
147 | |         [
...   |
156 | |         ],
157 | |     )
    | |_____^

error: last argument of `simd_shuffle` is required to be a `const` item
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:466:5
466 | /     simd_shuffle4(
467 | |         a,
468 | |         b,
469 | |         [
469 | |         [
...   |
474 | |         ],
475 | |     )
    | |_____^

error: last argument of `simd_shuffle` is required to be a `const` item
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:489:5
489 | /     simd_shuffle8(
490 | |         a,
491 | |         b,
492 | |         [
492 | |         [
...   |
501 | |         ],
502 | |     )
    | |_____^

error: last argument of `simd_shuffle` is required to be a `const` item
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:933:5
933 | /     simd_shuffle4(
934 | |         a,
934 | |         a,
935 | |         _mm256_undefined_ps(),
936 | |         [[0, 1, 2, 3], [4, 5, 6, 7]][IMM1 as usize],
    | |_____^


error: last argument of `simd_shuffle` is required to be a `const` item
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:954:5
    |
954 |     simd_shuffle2(a, _mm256_undefined_pd(), [[0, 1], [2, 3]][IMM1 as usize])


error: last argument of `simd_shuffle` is required to be a `const` item
   --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:970:22
    |
970 |       let dst: i64x2 = simd_shuffle2(
    |  ______________________^
971 | |         a.as_i64x4(),
972 | |         _mm256_undefined_si256().as_i64x4(),
973 | |         [[0, 1], [2, 3]][IMM1 as usize],
    | |_____^


error: last argument of `simd_shuffle` is required to be a `const` item
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1036:5
1036 | /     simd_shuffle8(
1037 | |         a,
1037 | |         a,
1038 | |         _mm256_undefined_ps(),
...    |
1048 | |         ],
1049 | |     )
     | |_____^
     | |_____^

error: last argument of `simd_shuffle` is required to be a `const` item
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1063:5
1063 | /     simd_shuffle4(
1064 | |         a,
1064 | |         a,
1065 | |         _mm_undefined_ps(),
...    |
1071 | |         ],
1072 | |     )
     | |_____^
     | |_____^

error: last argument of `simd_shuffle` is required to be a `const` item
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1110:5
1110 | /     simd_shuffle4(
1111 | |         a,
1111 | |         a,
1112 | |         _mm256_undefined_pd(),
...    |
1118 | |         ],
1119 | |     )
     | |_____^
     | |_____^

error: last argument of `simd_shuffle` is required to be a `const` item
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1133:5
1133 | /     simd_shuffle2(
1134 | |         a,
1134 | |         a,
1135 | |         _mm_undefined_pd(),
1136 | |         [(IMM2 as u32) & 1, (IMM2 as u32 >> 1) & 1],
     | |_____^


error: last argument of `simd_shuffle` is required to be a `const` item
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:1260:5
1260 | /     simd_shuffle8(
1261 | |         a,
1261 | |         a,
1262 | |         _mm256_castps128_ps256(b),
1263 | |         [[8, 9, 10, 11, 4, 5, 6, 7], [0, 1, 2, 3, 8, 9, 10, 11]][IMM1 as usize],
     | |_____^


error: last argument of `simd_shuffle` is required to be a `const` item

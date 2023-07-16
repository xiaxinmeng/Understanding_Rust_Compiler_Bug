plain
[00:19:54]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:20:00]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:20:00]    Compiling cmake v0.1.31
[00:20:00]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:20:01] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:01]    --> libcore/../stdsimd/coresimd/ppsv/api/from_bits.rs:8:20
[00:20:01]     |
[00:20:01] 8   |               impl ::simd::FromBits<$from> for $to {
[00:20:01]     |                      ^^^^ Did you mean `coresimd::simd`?
[00:20:01]     | 
[00:20:01]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:179:1
[00:20:01] 179 | / impl_from_bits!(
[00:20:01] 180 | |     u64x2: u64,
[00:20:01] 181 | |     u64x2_from_bits,
[00:20:01] 181 | |     u64x2_from_bits,
[00:20:01] 182 | |     test_v128 | i64x2,
[00:20:01] 194 | |     m8x16
[00:20:01] 195 | | );
[00:20:01]     | |__- in this macro invocation
[00:20:01] 
[00:20:01] 
[00:20:01] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:01]    --> libcore/../stdsimd/coresimd/ppsv/api/from_bits.rs:8:20
[00:20:01]     |
[00:20:01] 8   |             impl ::simd::FromBits<$from> for $to {
[00:20:01]     |                    ^^^^ Did you mean `coresimd::simd`?
[00:20:01]     | 
[00:20:01]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:196:1
[00:20:01]     |
[00:20:01] 196 | from_bits_x86!(u64x2, u64, u64x2_from_bits_x86);
[00:20:01]     | ------------------------------------------------ in this macro invocation
[00:20:01] 
[00:20:01] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:01]    --> libcore/../stdsimd/coresimd/ppsv/api/from_bits.rs:8:20
[00:20:01]     |
[00:20:01] 8   |               impl ::simd::FromBits<$from> for $to {
[00:20:01]     |                      ^^^^ Did you mean `coresimd::simd`?
[00:20:01]     | 
[00:20:01]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:199:1
[00:20:01] 199 | / impl_from_bits!(
[00:20:01] 199 | / impl_from_bits!(
[00:20:01] 200 | |     i64x2: i64,
[00:20:01] 201 | |     i64x2_from_bits,
[00:20:01] 202 | |     test_v128 | u64x2,
[00:20:01] 214 | |     m8x16
[00:20:01] 215 | | );
[00:20:01]     | |__- in this macro invocation
[00:20:01] 
[00:20:01] 
[00:20:02] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:02]    --> libcore/../stdsimd/coresimd/ppsv/api/from_bits.rs:8:20
[00:20:02]     |
[00:20:02] 8   |             impl ::simd::FromBits<$from> for $to {
[00:20:02]     |                    ^^^^ Did you mean `coresimd::simd`?
[00:20:02]     | 
[00:20:02]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:216:1
[00:20:02]     |
[00:20:02] 216 | from_bits_x86!(i64x2, i64, i64x2_from_bits_x86);
[00:20:02]     | ------------------------------------------------ in this macro invocation
[00:20:02] 
[00:20:02] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:02]    --> libcore/../stdsimd/coresimd/ppsv/api/from_bits.rs:8:20
[00:20:02]     |
[00:20:02] 8   |               impl ::simd::FromBits<$from> for $to {
[00:20:02]     |                      ^^^^ Did you mean `coresimd::simd`?
[00:20:02]     | 
[00:20:02]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:219:1
[00:20:02] 219 | / impl_from_bits!(
[00:20:02] 219 | / impl_from_bits!(
[00:20:02] 220 | |     f64x2: f64,
[00:20:02] 221 | |     f64x2_from_bits,
[00:20:02] 222 | |     test_v128 | i64x2,
[00:20:02] 234 | |     m8x16
[00:20:02] 235 | | );
[00:20:02]     | |__- in this macro invocation
[00:20:02] 
[00:20:02] 
[00:20:02] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:02]    --> libcore/../stdsimd/coresimd/ppsv/api/from_bits.rs:8:20
[00:20:02]     |
[00:20:02] 8   |             impl ::simd::FromBits<$from> for $to {
[00:20:02]     |                    ^^^^ Did you mean `coresimd::simd`?
[00:20:02]     | 
[00:20:02]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:236:1
[00:20:02]     |
[00:20:02] 236 | from_bits_x86!(f64x2, f64, f64x2_from_bits_x86);
[00:20:02]     | ------------------------------------------------ in this macro invocation
[00:20:02] 
[00:20:02] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:02]    --> libcore/../stdsimd/coresimd/ppsv/api/from_bits.rs:8:20
[00:20:02]     |
[00:20:02] 8   |               impl ::simd::FromBits<$from> for $to {
[00:20:02]     |                      ^^^^ Did you mean `coresimd::simd`?
[00:20:02]     | 
[00:20:02]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:239:1
[00:20:02] 239 | / impl_from_bits!(
[00:20:02] 240 | |     u32x4: u32,
[00:20:02] 241 | |     u32x4_from_bits,
[00:20:02] 241 | |     u32x4_from_bits,
[00:20:02] 242 | |     test_v128 | u64x2,
[00:20:02] 254 | |     m8x16
[00:20:02] 255 | | );
[00:20:02]     | |__- in this macro invocation
[00:20:02] 
[00:20:02] 
[00:20:03] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:03]    --> libcore/../stdsimd/coresimd/ppsv/api/from_bits.rs:8:20
[00:20:03]     |
[00:20:03] 8   |             impl ::simd::FromBits<$from> for $to {
[00:20:03]     |                    ^^^^ Did you mean `coresimd::simd`?
[00:20:03]     | 
[00:20:03]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:256:1
[00:20:03]     |
[00:20:03] 256 | from_bits_x86!(u32x4, u32, u32x4_from_bits_x86);
[00:20:03]     | ------------------------------------------------ in this macro invocation
[00:20:03] 
[00:20:03] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:03]    --> libcore/../stdsimd/coresimd/ppsv/api/from_bits.rs:8:20
[00:20:03]     |
[00:20:03] 8   |               impl ::simd::FromBits<$from> for $to {
[00:20:03]     |                      ^^^^ Did you mean `coresimd::simd`?
[00:20:03]     | 
[00:20:03]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:259:1
[00:20:03] 259 | / impl_from_bits!(
[00:20:03] 259 | / impl_from_bits!(
[00:20:03] 260 | |     i32x4: i32,
[00:20:03] 261 | |     i32x4_from_bits,
[00:20:03] 262 | |     test_v128 | u64x2,
[00:20:03] 274 | |     m8x16
[00:20:03] 275 | | );
[00:20:03]     | |__- in this macro invocation
[00:20:03] 
[00:20:03] 
[00:20:03] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:03]    --> libcore/../stdsimd/coresimd/ppsv/api/from_bits.rs:8:20
[00:20:03]     |
[00:20:03] 8   |             impl ::simd::FromBits<$from> for $to {
[00:20:03]     |                    ^^^^ Did you mean `coresimd::simd`?
[00:20:03]     | 
[00:20:03]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:276:1
[00:20:03]     |
[00:20:03] 276 | from_bits_x86!(i32x4, i32, i32x4_from_bits_x86);
[00:20:03]     | ------------------------------------------------ in this macro invocation
[00:20:03] 
[00:20:03] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:03]    --> libcore/../stdsimd/coresimd/ppsv/api/from_bits.rs:8:20
[00:20:03]     |
[00:20:03] 8   |               impl ::simd::FromBits<$from> for $to {
[00:20:03]     |                      ^^^^ Did you mean `coresimd::simd`?
[00:20:03]     | 
[00:20:03]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:279:1
[00:20:03] 279 | / impl_from_bits!(
[00:20:03] 279 | / impl_from_bits!(
[00:20:03] 280 | |     f32x4: f32,
[00:20:03] 281 | |     f32x4_from_bits,
[00:20:03] 282 | |     test_v128 | u64x2,
[00:20:03] 294 | |     m8x16
[00:20:03] 295 | | );
[00:20:03]     | |__- in this macro invocation
[00:20:03] 
[00:20:03] 
[00:20:04] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:04]    --> libcore/../stdsimd/coresimd/ppsv/api/from_bits.rs:8:20
[00:20:04]     |
[00:20:04] 8   |             impl ::simd::FromBits<$from> for $to {
[00:20:04]     |                    ^^^^ Did you mean `coresimd::simd`?
[00:20:04]     | 
[00:20:04]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:296:1
[00:20:04]     |
[00:20:04] 296 | from_bits_x86!(f32x4, f32, f32x4_from_bits_x86);
[00:20:04]     | ------------------------------------------------ in this macro invocation
[00:20:04] 
[00:20:04] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:04]    --> libcore/../stdsimd/coresimd/ppsv/api/from_bits.rs:8:20
[00:20:04]     |
[00:20:04] 8   |               impl ::simd::FromBits<$from> for $to {
[00:20:04]     |                      ^^^^ Did you mean `coresimd::simd`?
[00:20:04]     | 
[00:20:04]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:299:1
[00:20:04] 299 | / impl_from_bits!(
[00:20:04] 300 | |     u16x8: u16,
[00:20:04] 301 | |     u16x8_from_bits,
[00:20:04] 301 | |     u16x8_from_bits,
[00:20:04] 302 | |     test_v128 | u64x2,
[00:20:04] 314 | |     m8x16
[00:20:04] 315 | | );
[00:20:04]     | |__- in this macro invocation
[00:20:04] 
[00:20:04] 
[00:20:04]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:20:04]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:20:04] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:04]    --> libcore/../stdsimd/coresimd/ppsv/api/from_bits.rs:8:20
[00:20:04]     |
[00:20:04] 8   |             impl ::simd::FromBits<$from> for $to {
[00:20:04]     |                    ^^^^ Did you mean `coresimd::simd`?
[00:20:04]     | 
[00:20:04]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:316:1
[00:20:04]     |
[00:20:04] 316 | from_bits_x86!(u16x8, u16, u16x8_from_bits_x86);
[00:20:04]     | ------------------------------------------------ in this macro invocation
[00:20:04] 
[00:20:04] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:04]    --> libcore/../stdsimd/coresimd/ppsv/api/from_bits.rs:8:20
[00:20:04]     |
[00:20:04] 8   |               impl ::simd::FromBits<$from> for $to {
[00:20:04]     |                      ^^^^ Did you mean `coresimd::simd`?
[00:20:04]     | 
[00:20:04]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:319:1
[00:20:04] 319 | / impl_from_bits!(
[00:20:04] 319 | / impl_from_bits!(
[00:20:04] 320 | |     i16x8: i16,
[00:20:04] 321 | |     i16x8_from_bits,
[00:20:04] 322 | |     test_v128 | u64x2,
[00:20:04] 334 | |     m8x16
[00:20:04] 335 | | );
[00:20:04]     | |__- in this macro invocation
[00:20:04] 
[00:20:04] 
[00:20:05] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:05]    --> libcore/../stdsimd/coresimd/ppsv/api/from_bits.rs:8:20
[00:20:05]     |
[00:20:05] 8   |             impl ::simd::FromBits<$from> for $to {
[00:20:05]     |                    ^^^^ Did you mean `coresimd::simd`?
[00:20:05]     | 
[00:20:05]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:336:1
[00:20:05]     |
[00:20:05] 336 | from_bits_x86!(i16x8, i16, i16x8_from_bits_x86);
[00:20:05]     | ------------------------------------------------ in this macro invocation
[00:20:05] 
[00:20:05] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:05]    --> libcore/../stdsimd/coresimd/ppsv/api/from_bits.rs:8:20
[00:20:05]     |
[00:20:05] 8   |               impl ::simd::FromBits<$from> for $to {
[00:20:05]     |                      ^^^^ Did you mean `coresimd::simd`?
[00:20:05]     | 
[00:20:05]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:339:1
[00:20:05] 339 | / impl_from_bits!(
[00:20:05] 340 | |     u8x16: u8,
[00:20:05] 341 | |     u8x16_from_bits,
[00:20:05] 341 | |     u8x16_from_bits,
[00:20:05] 342 | |     test_v128 | u64x2,
[00:20:05] 354 | |     m8x16
[00:20:05] 355 | | );
[00:20:05]     | |__- in this macro invocation
[00:20:05] 
[00:20:05] 
[00:20:05] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:05]    --> libcore/../stdsimd/coresimd/ppsv/api/from_bits.rs:8:20
[00:20:05]     |
[00:20:05] 8   |             impl ::simd::FromBits<$from> for $to {
[00:20:05]     |                    ^^^^ Did you mean `coresimd::simd`?
[00:20:05]     | 
[00:20:05]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:356:1
[00:20:05]     |
[00:20:05] 356 | from_bits_x86!(u8x16, u8, u8x16_from_bits_x86);
[00:20:05]     | ----------------------------------------------- in this macro invocation
[00:20:05] 
[00:20:05] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:05]    --> libcore/../stdsimd/coresimd/ppsv/api/from_bits.rs:8:20
[00:20:05]     |
[00:20:05] 8   |               impl ::simd::FromBits<$from> for $to {
[00:20:05]     |                      ^^^^ Did you mean `coresimd::simd`?
[00:20:05]     | 
[00:20:05]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:359:1
[00:20:05] 359 | / impl_from_bits!(
[00:20:05] 359 | / impl_from_bits!(
[00:20:05] 360 | |     i8x16: i8,
[00:20:05] 361 | |     i8x16_from_bits,
[00:20:05] 362 | |     test_v128 | u64x2,
[00:20:05] 374 | |     m8x16
[00:20:05] 375 | | );
[00:20:05]     | |__- in this macro invocation
[00:20:05] 
[00:20:05] 
[00:20:05]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:20:06] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:06]    --> libcore/../stdsimd/coresimd/ppsv/api/from_bits.rs:8:20
[00:20:06]     |
[00:20:06] 8   |             impl ::simd::FromBits<$from> for $to {
[00:20:06]     |                    ^^^^ Did you mean `coresimd::simd`?
[00:20:06]     | 
[00:20:06]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:376:1
[00:20:06]     |
[00:20:06] 376 | from_bits_x86!(i8x16, i8, i8x16_from_bits_x86);
[00:20:06]     | ----------------------------------------------- in this macro invocation
[00:20:06]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:20:06]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:20:06] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:06]    --> libcore/../stdsimd/coresimd/ppsv/api/from.rs:7:32
[00:20:06]     |
[00:20:06] 7   |           impl ::convert::From<::simd::$from> for $to {
[00:20:06]     |                                  ^^^^ Did you mean `coresimd::simd`?
[00:20:06]     | 
[00:20:06]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:379:1
[00:20:06] 379 | / impl_from!(
[00:20:06] 379 | / impl_from!(
[00:20:06] 380 | |     f64x2: f64,
[00:20:06] 381 | |     f64x2_from,
[00:20:06] 382 | |     test_v128 | f32x2,
[00:20:06] 394 | |     m8x2
[00:20:06] 395 | | );
[00:20:06]     | |__- in this macro invocation
[00:20:06] 
[00:20:06] 
[00:20:06] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:06]    --> libcore/../stdsimd/coresimd/ppsv/api/from.rs:9:26
[00:20:06]     |
[00:20:06] 9   |               fn from(f: ::simd::$from) -> $to {
[00:20:06]     |                            ^^^^ Did you mean `coresimd::simd`?
[00:20:06]     | 
[00:20:06]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:379:1
[00:20:06] 379 | / impl_from!(
[00:20:06] 379 | / impl_from!(
[00:20:06] 380 | |     f64x2: f64,
[00:20:06] 381 | |     f64x2_from,
[00:20:06] 382 | |     test_v128 | f32x2,
[00:20:06] 394 | |     m8x2
[00:20:06] 395 | | );
[00:20:06]     | |__- in this macro invocation
[00:20:06] 
[00:20:06] 
[00:20:06] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:06]    --> libcore/../stdsimd/coresimd/ppsv/api/from.rs:7:32
[00:20:06]     |
[00:20:06] 7   |           impl ::convert::From<::simd::$from> for $to {
[00:20:06]     |                                  ^^^^ Did you mean `coresimd::simd`?
[00:20:06]     | 
[00:20:06]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:379:1
[00:20:06] 379 | / impl_from!(
[00:20:06] 379 | / impl_from!(
[00:20:06] 380 | |     f64x2: f64,
[00:20:06] 381 | |     f64x2_from,
[00:20:06] 382 | |     test_v128 | f32x2,
[00:20:06] 394 | |     m8x2
[00:20:06] 395 | | );
[00:20:06]     | |__- in this macro invocation
[00:20:06] 
[00:20:06] 
[00:20:06] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:06]    --> libcore/../stdsimd/coresimd/ppsv/api/from.rs:9:26
[00:20:06]     |
[00:20:06] 9   |               fn from(f: ::simd::$from) -> $to {
[00:20:06]     |                            ^^^^ Did you mean `coresimd::simd`?
[00:20:06]     | 
[00:20:06]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:379:1
[00:20:06] 379 | / impl_from!(
[00:20:06] 379 | / impl_from!(
[00:20:06] 380 | |     f64x2: f64,
[00:20:06] 381 | |     f64x2_from,
[00:20:06] 382 | |     test_v128 | f32x2,
[00:20:06] 394 | |     m8x2
[00:20:06] 395 | | );
[00:20:06]     | |__- in this macro invocation
[00:20:06] 
[00:20:06] 
[00:20:06] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:06]    --> libcore/../stdsimd/coresimd/ppsv/api/from.rs:7:32
[00:20:06]     |
[00:20:06] 7   |           impl ::convert::From<::simd::$from> for $to {
[00:20:06]     |                                  ^^^^ Did you mean `coresimd::simd`?
[00:20:06]     | 
[00:20:06]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:379:1
[00:20:06] 379 | / impl_from!(
[00:20:06] 379 | / impl_from!(
[00:20:06] 380 | |     f64x2: f64,
[00:20:06] 381 | |     f64x2_from,
[00:20:06] 382 | |     test_v128 | f32x2,
[00:20:06] 394 | |     m8x2
[00:20:06] 395 | | );
[00:20:06]     | |__- in this macro invocation
[00:20:06] 
[00:20:06] 
[00:20:06] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:06]    --> libcore/../stdsimd/coresimd/ppsv/api/from.rs:9:26
[00:20:06]     |
[00:20:06] 9   |               fn from(f: ::simd::$from) -> $to {
[00:20:06]     |                            ^^^^ Did you mean `coresimd::simd`?
[00:20:06]     | 
[00:20:06]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:379:1
[00:20:06] 379 | / impl_from!(
[00:20:06] 379 | / impl_from!(
[00:20:06] 380 | |     f64x2: f64,
[00:20:06] 381 | |     f64x2_from,
[00:20:06] 382 | |     test_v128 | f32x2,
[00:20:06] 394 | |     m8x2
[00:20:06] 395 | | );
[00:20:06]     | |__- in this macro invocation
[00:20:06] 
[00:20:06] 
[00:20:06] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:06]    --> libcore/../stdsimd/coresimd/ppsv/api/from.rs:7:32
[00:20:06]     |
[00:20:06] 7   |           impl ::convert::From<::simd::$from> for $to {
[00:20:06]     |                                  ^^^^ Did you mean `coresimd::simd`?
[00:20:06]     | 
[00:20:06]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:379:1
[00:20:06] 379 | / impl_from!(
[00:20:06] 379 | / impl_from!(
[00:20:06] 380 | |     f64x2: f64,
[00:20:06] 381 | |     f64x2_from,
[00:20:06] 382 | |     test_v128 | f32x2,
[00:20:06] 394 | |     m8x2
[00:20:06] 395 | | );
[00:20:06]     | |__- in this macro invocation
[00:20:06] 
[00:20:06] 
[00:20:06] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:06]    --> libcore/../stdsimd/coresimd/ppsv/api/from.rs:9:26
[00:20:06]     |
[00:20:06] 9   |               fn from(f: ::simd::$from) -> $to {
[00:20:06]     |                            ^^^^ Did you mean `coresimd::simd`?
[00:20:06]     | 
[00:20:06]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:379:1
[00:20:06] 379 | / impl_from!(
[00:20:06] 379 | / impl_from!(
[00:20:06] 380 | |     f64x2: f64,
[00:20:06] 381 | |     f64x2_from,
[00:20:06] 382 | |     test_v128 | f32x2,
[00:20:06] 394 | |     m8x2
[00:20:06] 395 | | );
[00:20:06]     | |__- in this macro invocation
[00:20:06] 
[00:20:06] 
[00:20:06] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:06]    --> libcore/../stdsimd/coresimd/ppsv/api/from.rs:7:32
[00:20:06]     |
[00:20:06] 7   |           impl ::convert::From<::simd::$from> for $to {
[00:20:06]     |                                  ^^^^ Did you mean `coresimd::simd`?
[00:20:06]     | 
[00:20:06]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:379:1
[00:20:06] 379 | / impl_from!(
[00:20:06] 379 | / impl_from!(
[00:20:06] 380 | |     f64x2: f64,
[00:20:06] 381 | |     f64x2_from,
[00:20:06] 382 | |     test_v128 | f32x2,
[00:20:06] 394 | |     m8x2
[00:20:06] 395 | | );
[00:20:06]     | |__- in this macro invocation
[00:20:06] 
[00:20:06] 
[00:20:06] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:06]    --> libcore/../stdsimd/coresimd/ppsv/api/from.rs:9:26
[00:20:06]     |
[00:20:06] 9   |               fn from(f: ::simd::$from) -> $to {
[00:20:06]     |                            ^^^^ Did you mean `coresimd::simd`?
[00:20:06]     | 
[00:20:06]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:379:1
[00:20:06] 379 | / impl_from!(
[00:20:06] 379 | / impl_from!(
[00:20:06] 380 | |     f64x2: f64,
[00:20:06] 381 | |     f64x2_from,
[00:20:06] 382 | |     test_v128 | f32x2,
[00:20:06] 394 | |     m8x2
[00:20:06] 395 | | );
[00:20:06]     | |__- in this macro invocation
[00:20:06] 
[00:20:06] 
[00:20:06] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:06]    --> libcore/../stdsimd/coresimd/ppsv/api/from.rs:7:32
[00:20:06]     |
[00:20:06] 7   |           impl ::convert::From<::simd::$from> for $to {
[00:20:06]     |                                  ^^^^ Did you mean `coresimd::simd`?
[00:20:06]     | 
[00:20:06]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:379:1
[00:20:06] 379 | / impl_from!(
[00:20:06] 379 | / impl_from!(
[00:20:06] 380 | |     f64x2: f64,
[00:20:06] 381 | |     f64x2_from,
[00:20:06] 382 | |     test_v128 | f32x2,
[00:20:06] 394 | |     m8x2
[00:20:06] 395 | | );
[00:20:06]     | |__- in this macro invocation
[00:20:06] 
[00:20:06] 
[00:20:06] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:06]    --> libcore/../stdsimd/coresimd/ppsv/api/from.rs:9:26
[00:20:06]     |
[00:20:06] 9   |               fn from(f: ::simd::$from) -> $to {
[00:20:06]     |                            ^^^^ Did you mean `coresimd::simd`?
[00:20:06]     | 
[00:20:06]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:379:1
[00:20:06] 379 | / impl_from!(
[00:20:06] 379 | / impl_from!(
[00:20:06] 380 | |     f64x2: f64,
[00:20:06] 381 | |     f64x2_from,
[00:20:06] 382 | |     test_v128 | f32x2,
[00:20:06] 394 | |     m8x2
[00:20:06] 395 | | );
[00:20:06]     | |__- in this macro invocation
[00:20:06] 
[00:20:06] 
[00:20:06] error[E0433]: failed to resolve. Did you mean `coresimd::simd`?
[00:20:06]    --> libcore/../stdsimd/coresimd/ppsv/api/from.rs:7:32
[00:20:06]     |
[00:20:06] 7   |           impl ::convert::From<::simd::$from> for $to {
[00:20:06]     |                                  ^^^^ Did you mean `coresimd::simd`?
[00:20:06]     | 
[00:20:06]    ::: libcore/../stdsimd/coresimd/ppsv/v128.rs:379:1
[00:20:06] 379 | / impl_from!(

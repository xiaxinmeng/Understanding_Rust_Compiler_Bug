plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:ece894d15649f0f9d7884f915dc821f00bd0418b)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
     |
note: previously defined here
    --> /checkout/library/core/src/macros/mod.rs:1310:5
     |
1310 | /     macro_rules! cfg {
1311 | |         ($($cfg:tt)*) => {
1313 | |         };
1314 | |     }
     | |_____^


error[E0425]: cannot find function `vld2_s8` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:8772:15
     |
8772 |     transmute(vld2_s8(transmute(a)))
     |               ^^^^^^^ help: a function with a similar name exists: `vld2_p8`
...
8849 | pub unsafe fn vld2_p8(a: *const p8) -> poly8x8x2_t {
     | -------------------------------------------------- similarly named function `vld2_p8` defined here

error[E0425]: cannot find function `vld2_s16` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:8785:15
     |
8785 |     transmute(vld2_s16(transmute(a)))
     |               ^^^^^^^^ help: a function with a similar name exists: `vld2_p16`
...
8862 | pub unsafe fn vld2_p16(a: *const p16) -> poly16x4x2_t {
     | ----------------------------------------------------- similarly named function `vld2_p16` defined here

error[E0425]: cannot find function `vld2_s32` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:8798:15
     |
8798 |     transmute(vld2_s32(transmute(a)))


error[E0425]: cannot find function `vld2q_s8` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:8811:15
     |
8811 |     transmute(vld2q_s8(transmute(a)))
     |               ^^^^^^^^ help: a function with a similar name exists: `vld2q_p8`
...
8875 | pub unsafe fn vld2q_p8(a: *const p8) -> poly8x16x2_t {
     | ---------------------------------------------------- similarly named function `vld2q_p8` defined here

error[E0425]: cannot find function `vld2q_s16` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:8824:15
     |
8824 |     transmute(vld2q_s16(transmute(a)))
     |               ^^^^^^^^^ help: a function with a similar name exists: `vld2q_p16`
...
8888 | pub unsafe fn vld2q_p16(a: *const p16) -> poly16x8x2_t {
     | ------------------------------------------------------ similarly named function `vld2q_p16` defined here

error[E0425]: cannot find function `vld2q_s32` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:8837:15
     |
8837 |     transmute(vld2q_s32(transmute(a)))


error[E0425]: cannot find function `vld2_s8` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:8850:15
     |
8850 |     transmute(vld2_s8(transmute(a)))


error[E0425]: cannot find function `vld2_s16` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:8863:15
     |
8863 |     transmute(vld2_s16(transmute(a)))


error[E0425]: cannot find function `vld2q_s8` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:8876:15
     |
8876 |     transmute(vld2q_s8(transmute(a)))


error[E0425]: cannot find function `vld2q_s16` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:8889:15
     |
8889 |     transmute(vld2q_s16(transmute(a)))


error[E0425]: cannot find function `vld2_s64` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:8902:15
     |
8902 |     transmute(vld2_s64(transmute(a)))
     |               ^^^^^^^^ help: a function with a similar name exists: `vld2_p64`
...
8914 | pub unsafe fn vld2_p64(a: *const p64) -> poly64x1x2_t {
     | ----------------------------------------------------- similarly named function `vld2_p64` defined here

error[E0425]: cannot find function `vld2_s64` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:8915:15
     |
8915 |     transmute(vld2_s64(transmute(a)))


error[E0425]: cannot find function `vld2_dup_s8` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9225:15
     |
9225 |     transmute(vld2_dup_s8(transmute(a)))
     |               ^^^^^^^^^^^ help: a function with a similar name exists: `vld1_dup_s8`
    ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1364:1
     |
     |
1364 | pub unsafe fn vld1_dup_s8(ptr: *const i8) -> int8x8_t {
     | ----------------------------------------------------- similarly named function `vld1_dup_s8` defined here

error[E0425]: cannot find function `vld2_dup_s16` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9238:15
     |
9238 |     transmute(vld2_dup_s16(transmute(a)))
     |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1_dup_s16`
    ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1394:1
     |
     |
1394 | pub unsafe fn vld1_dup_s16(ptr: *const i16) -> int16x4_t {
     | -------------------------------------------------------- similarly named function `vld1_dup_s16` defined here

error[E0425]: cannot find function `vld2_dup_s32` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9251:15
     |
9251 |     transmute(vld2_dup_s32(transmute(a)))
     |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1_dup_s32`
    ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1424:1
     |
     |
1424 | pub unsafe fn vld1_dup_s32(ptr: *const i32) -> int32x2_t {
     | -------------------------------------------------------- similarly named function `vld1_dup_s32` defined here

error[E0425]: cannot find function `vld2q_dup_s8` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9264:15
     |
9264 |     transmute(vld2q_dup_s8(transmute(a)))
     |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1q_dup_s8`
    ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1379:1
     |
     |
1379 | pub unsafe fn vld1q_dup_s8(ptr: *const i8) -> int8x16_t {
     | ------------------------------------------------------- similarly named function `vld1q_dup_s8` defined here

error[E0425]: cannot find function `vld2q_dup_s16` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9277:15
     |
9277 |     transmute(vld2q_dup_s16(transmute(a)))
     |               ^^^^^^^^^^^^^ help: a function with a similar name exists: `vld1q_dup_s16`
    ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1409:1
     |
     |
1409 | pub unsafe fn vld1q_dup_s16(ptr: *const i16) -> int16x8_t {
     | --------------------------------------------------------- similarly named function `vld1q_dup_s16` defined here

error[E0425]: cannot find function `vld2q_dup_s32` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9290:15
     |
9290 |     transmute(vld2q_dup_s32(transmute(a)))
     |               ^^^^^^^^^^^^^ help: a function with a similar name exists: `vld1q_dup_s32`
    ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1439:1
     |
     |
1439 | pub unsafe fn vld1q_dup_s32(ptr: *const i32) -> int32x4_t {
     | --------------------------------------------------------- similarly named function `vld1q_dup_s32` defined here

error[E0425]: cannot find function `vld2_dup_s8` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9303:15
     |
9303 |     transmute(vld2_dup_s8(transmute(a)))
     |               ^^^^^^^^^^^ help: a function with a similar name exists: `vld1_dup_s8`
    ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1364:1
     |
     |
1364 | pub unsafe fn vld1_dup_s8(ptr: *const i8) -> int8x8_t {
     | ----------------------------------------------------- similarly named function `vld1_dup_s8` defined here

error[E0425]: cannot find function `vld2_dup_s16` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9316:15
     |
9316 |     transmute(vld2_dup_s16(transmute(a)))
     |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1_dup_s16`
    ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1394:1
     |
     |
1394 | pub unsafe fn vld1_dup_s16(ptr: *const i16) -> int16x4_t {
     | -------------------------------------------------------- similarly named function `vld1_dup_s16` defined here

error[E0425]: cannot find function `vld2q_dup_s8` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9329:15
     |
9329 |     transmute(vld2q_dup_s8(transmute(a)))
     |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1q_dup_s8`
    ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1379:1
     |
     |
1379 | pub unsafe fn vld1q_dup_s8(ptr: *const i8) -> int8x16_t {
     | ------------------------------------------------------- similarly named function `vld1q_dup_s8` defined here

error[E0425]: cannot find function `vld2q_dup_s16` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9342:15
     |
9342 |     transmute(vld2q_dup_s16(transmute(a)))
     |               ^^^^^^^^^^^^^ help: a function with a similar name exists: `vld1q_dup_s16`
    ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1409:1
     |
     |
1409 | pub unsafe fn vld1q_dup_s16(ptr: *const i16) -> int16x8_t {
     | --------------------------------------------------------- similarly named function `vld1q_dup_s16` defined here

error[E0425]: cannot find function `vld2_dup_s64` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9355:15
     |
9355 |     transmute(vld2_dup_s64(transmute(a)))
     |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1_dup_s64`
    ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1454:1
     |
     |
1454 | pub unsafe fn vld1_dup_s64(ptr: *const i64) -> int64x1_t {
     | -------------------------------------------------------- similarly named function `vld1_dup_s64` defined here

error[E0425]: cannot find function `vld2_dup_s64` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9368:15
     |
9368 |     transmute(vld2_dup_s64(transmute(a)))
     |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1_dup_s64`
    ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1454:1
     |
     |
1454 | pub unsafe fn vld1_dup_s64(ptr: *const i64) -> int64x1_t {
     | -------------------------------------------------------- similarly named function `vld1_dup_s64` defined here

error[E0425]: cannot find function `vld2_lane_s8` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9634:15
     |
9634 |     transmute(vld2_lane_s8::<LANE>(transmute(a), transmute(b)))
     |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1_lane_s8`
    ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:977:1
     |
     |
977  | pub unsafe fn vld1_lane_s8<const LANE: i32>(ptr: *const i8, src: int8x8_t) -> int8x8_t {
     | -------------------------------------------------------------------------------------- similarly named function `vld1_lane_s8` defined here

error[E0425]: cannot find function `vld2_lane_s16` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9649:15
     |
9649 |     transmute(vld2_lane_s16::<LANE>(transmute(a), transmute(b)))
     |               ^^^^^^^^^^^^^ help: a function with a similar name exists: `vld1_lane_s16`
    ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1009:1
     |
     |
1009 | pub unsafe fn vld1_lane_s16<const LANE: i32>(ptr: *const i16, src: int16x4_t) -> int16x4_t {
     | ------------------------------------------------------------------------------------------ similarly named function `vld1_lane_s16` defined here

error[E0425]: cannot find function `vld2_lane_s32` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9664:15
     |
9664 |     transmute(vld2_lane_s32::<LANE>(transmute(a), transmute(b)))
     |               ^^^^^^^^^^^^^ help: a function with a similar name exists: `vld1_lane_s32`
    ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1041:1
     |
     |
1041 | pub unsafe fn vld1_lane_s32<const LANE: i32>(ptr: *const i32, src: int32x2_t) -> int32x2_t {
     | ------------------------------------------------------------------------------------------ similarly named function `vld1_lane_s32` defined here

error[E0425]: cannot find function `vld2q_lane_s16` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9679:15
     |
9679 |     transmute(vld2q_lane_s16::<LANE>(transmute(a), transmute(b)))
     |               ^^^^^^^^^^^^^^ help: a function with a similar name exists: `vld1q_lane_s16`
    ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1025:1
     |
     |
1025 | pub unsafe fn vld1q_lane_s16<const LANE: i32>(ptr: *const i16, src: int16x8_t) -> int16x8_t {
     | ------------------------------------------------------------------------------------------- similarly named function `vld1q_lane_s16` defined here

error[E0425]: cannot find function `vld2q_lane_s32` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9694:15
     |
9694 |     transmute(vld2q_lane_s32::<LANE>(transmute(a), transmute(b)))
     |               ^^^^^^^^^^^^^^ help: a function with a similar name exists: `vld1q_lane_s32`
    ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1057:1
     |
     |
1057 | pub unsafe fn vld1q_lane_s32<const LANE: i32>(ptr: *const i32, src: int32x4_t) -> int32x4_t {
     | ------------------------------------------------------------------------------------------- similarly named function `vld1q_lane_s32` defined here

error[E0425]: cannot find function `vld2_lane_s8` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9709:15
     |
9709 |     transmute(vld2_lane_s8::<LANE>(transmute(a), transmute(b)))
     |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1_lane_s8`
    ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:977:1
     |
     |
977  | pub unsafe fn vld1_lane_s8<const LANE: i32>(ptr: *const i8, src: int8x8_t) -> int8x8_t {
     | -------------------------------------------------------------------------------------- similarly named function `vld1_lane_s8` defined here

error[E0425]: cannot find function `vld2_lane_s16` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9724:15
     |
9724 |     transmute(vld2_lane_s16::<LANE>(transmute(a), transmute(b)))
     |               ^^^^^^^^^^^^^ help: a function with a similar name exists: `vld1_lane_s16`
    ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1009:1
     |
     |
1009 | pub unsafe fn vld1_lane_s16<const LANE: i32>(ptr: *const i16, src: int16x4_t) -> int16x4_t {
     | ------------------------------------------------------------------------------------------ similarly named function `vld1_lane_s16` defined here

error[E0425]: cannot find function `vld2q_lane_s16` in this scope
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:9739:15
     |
9739 |     transmute(vld2q_lane_s16::<LANE>(transmute(a), transmute(b)))
     |               ^^^^^^^^^^^^^^ help: a function with a similar name exists: `vld1q_lane_s16`
    ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1025:1
     |
     |
1025 | pub unsafe fn vld1q_lane_s16<const LANE: i32>(ptr: *const i16, src: int16x8_t) -> int16x8_t {
     | ------------------------------------------------------------------------------------------- similarly named function `vld1q_lane_s16` defined here

error[E0425]: cannot find function `vld3_s8` in this scope
     --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10057:15
      |
10057 |     transmute(vld3_s8(transmute(a)))
      |               ^^^^^^^ help: a function with a similar name exists: `vld3_p8`
...
10134 | pub unsafe fn vld3_p8(a: *const p8) -> poly8x8x3_t {
      | -------------------------------------------------- similarly named function `vld3_p8` defined here

error[E0425]: cannot find function `vld3_s16` in this scope
     --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10070:15
      |
10070 |     transmute(vld3_s16(transmute(a)))
      |               ^^^^^^^^ help: a function with a similar name exists: `vld3_p16`
...
10147 | pub unsafe fn vld3_p16(a: *const p16) -> poly16x4x3_t {
      | ----------------------------------------------------- similarly named function `vld3_p16` defined here

error[E0425]: cannot find function `vld3_s32` in this scope
     --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10083:15
      |
10083 |     transmute(vld3_s32(transmute(a)))


error[E0425]: cannot find function `vld3q_s8` in this scope
     --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10096:15
      |
10096 |     transmute(vld3q_s8(transmute(a)))
      |               ^^^^^^^^ help: a function with a similar name exists: `vld3q_p8`
...
10160 | pub unsafe fn vld3q_p8(a: *const p8) -> poly8x16x3_t {
      | ---------------------------------------------------- similarly named function `vld3q_p8` defined here

error[E0425]: cannot find function `vld3q_s16` in this scope
     --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10109:15
      |
10109 |     transmute(vld3q_s16(transmute(a)))
      |               ^^^^^^^^^ help: a function with a similar name exists: `vld3q_p16`
...
10173 | pub unsafe fn vld3q_p16(a: *const p16) -> poly16x8x3_t {
      | ------------------------------------------------------ similarly named function `vld3q_p16` defined here

error[E0425]: cannot find function `vld3q_s32` in this scope
     --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10122:15
      |
10122 |     transmute(vld3q_s32(transmute(a)))


error[E0425]: cannot find function `vld3_s8` in this scope
     --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10135:15
      |
10135 |     transmute(vld3_s8(transmute(a)))


error[E0425]: cannot find function `vld3_s16` in this scope
     --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10148:15
      |
10148 |     transmute(vld3_s16(transmute(a)))


error[E0425]: cannot find function `vld3q_s8` in this scope
     --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10161:15
      |
10161 |     transmute(vld3q_s8(transmute(a)))


error[E0425]: cannot find function `vld3q_s16` in this scope
     --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10174:15
      |
10174 |     transmute(vld3q_s16(transmute(a)))


error[E0425]: cannot find function `vld3_s64` in this scope
     --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10187:15
      |
10187 |     transmute(vld3_s64(transmute(a)))
      |               ^^^^^^^^ help: a function with a similar name exists: `vld3_p64`
...
10199 | pub unsafe fn vld3_p64(a: *const p64) -> poly64x1x3_t {
      | ----------------------------------------------------- similarly named function `vld3_p64` defined here

error[E0425]: cannot find function `vld3_s64` in this scope
     --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10200:15
      |
10200 |     transmute(vld3_s64(transmute(a)))


error[E0425]: cannot find function `vld3_dup_s8` in this scope
     --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10510:15
      |
10510 |     transmute(vld3_dup_s8(transmute(a)))
      |               ^^^^^^^^^^^ help: a function with a similar name exists: `vld1_dup_s8`
     ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1364:1
      |
      |
1364  | pub unsafe fn vld1_dup_s8(ptr: *const i8) -> int8x8_t {
      | ----------------------------------------------------- similarly named function `vld1_dup_s8` defined here

error[E0425]: cannot find function `vld3_dup_s16` in this scope
     --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10523:15
      |
10523 |     transmute(vld3_dup_s16(transmute(a)))
      |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1_dup_s16`
     ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1394:1
      |
      |
1394  | pub unsafe fn vld1_dup_s16(ptr: *const i16) -> int16x4_t {
      | -------------------------------------------------------- similarly named function `vld1_dup_s16` defined here

error[E0425]: cannot find function `vld3_dup_s32` in this scope
     --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10536:15
      |
10536 |     transmute(vld3_dup_s32(transmute(a)))
      |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1_dup_s32`
     ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1424:1
      |
      |
1424  | pub unsafe fn vld1_dup_s32(ptr: *const i32) -> int32x2_t {
      | -------------------------------------------------------- similarly named function `vld1_dup_s32` defined here

error[E0425]: cannot find function `vld3q_dup_s8` in this scope
     --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10549:15
      |
10549 |     transmute(vld3q_dup_s8(transmute(a)))
      |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1q_dup_s8`
     ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1379:1
      |
      |
1379  | pub unsafe fn vld1q_dup_s8(ptr: *const i8) -> int8x16_t {
      | ------------------------------------------------------- similarly named function `vld1q_dup_s8` defined here

error[E0425]: cannot find function `vld3q_dup_s16` in this scope
     --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10562:15
      |
10562 |     transmute(vld3q_dup_s16(transmute(a)))
      |               ^^^^^^^^^^^^^ help: a function with a similar name exists: `vld1q_dup_s16`
     ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1409:1
      |
      |
1409  | pub unsafe fn vld1q_dup_s16(ptr: *const i16) -> int16x8_t {
      | --------------------------------------------------------- similarly named function `vld1q_dup_s16` defined here

error[E0425]: cannot find function `vld3q_dup_s32` in this scope
     --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10575:15
      |
10575 |     transmute(vld3q_dup_s32(transmute(a)))
      |               ^^^^^^^^^^^^^ help: a function with a similar name exists: `vld1q_dup_s32`
     ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1439:1
      |
      |
1439  | pub unsafe fn vld1q_dup_s32(ptr: *const i32) -> int32x4_t {
      | --------------------------------------------------------- similarly named function `vld1q_dup_s32` defined here

error[E0425]: cannot find function `vld3_dup_s8` in this scope
     --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10588:15
      |
10588 |     transmute(vld3_dup_s8(transmute(a)))
      |               ^^^^^^^^^^^ help: a function with a similar name exists: `vld1_dup_s8`
     ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1364:1
      |
      |
1364  | pub unsafe fn vld1_dup_s8(ptr: *const i8) -> int8x8_t {
      | ----------------------------------------------------- similarly named function `vld1_dup_s8` defined here

error[E0425]: cannot find function `vld3_dup_s16` in this scope
     --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10601:15
      |
10601 |     transmute(vld3_dup_s16(transmute(a)))
      |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1_dup_s16`
     ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1394:1
      |
      |
1394  | pub unsafe fn vld1_dup_s16(ptr: *const i16) -> int16x4_t {
      | -------------------------------------------------------- similarly named function `vld1_dup_s16` defined here

error[E0425]: cannot find function `vld3q_dup_s8` in this scope
     --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/generated.rs:10614:15
      |
10614 |     transmute(vld3q_dup_s8(transmute(a)))
      |               ^^^^^^^^^^^^ help: a function with a similar name exists: `vld1q_dup_s8`
     ::: /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:1379:1
      |

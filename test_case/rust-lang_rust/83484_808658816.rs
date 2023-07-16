plain
   Compiling libc v0.2.88
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0107]: this function takes 2 type arguments but only 1 type argument was supplied
    --> library/core/src/ptr/metadata.rs:263:21
     |
263  |         crate::ptr::hash::<VTable, _>(self.vtable_ptr, hasher)
     |                     ^^^^   ------ supplied 1 type argument
     |                     expected 2 type arguments
     |
     |
note: function defined here, with 2 type parameters: `T`, `S`
     |
     |
1408 | pub fn hash<T: ?Sized, S: hash::Hasher>(hashee: *const T, into: &mut S) {
     |        ^^^^ -          -
help: add missing type argument
     |
263  |         crate::ptr::hash::<VTable, S, _>(self.vtable_ptr, hasher)


error[E0107]: this struct takes 2 generic arguments but only 1 generic argument was supplied
    |
    |
601 |     let mut guard: Guard<_, N> =
    |                    ^^^^^ - supplied 1 generic argument
    |                    expected 2 generic arguments
    |
    |
note: struct defined here, with 2 generic parameters: `T`, `N`
    |
    |
582 |     struct Guard<T, const N: usize> {
    |            ^^^^^ -        -
help: add missing generic argument
    |
601 |     let mut guard: Guard<_, N, N> =


error[E0107]: this enum takes 2 type arguments but 0 type arguments were supplied
    |
    |
215 |         iter.map(|x| x.ok_or(())).sum::<Result<_, _>>().ok()
    |                                         ^^^^^^ expected 2 type arguments
    |
note: enum defined here, with 2 type parameters: `T`, `E`
    |
    |
241 | pub enum Result<T, E> {
    |          ^^^^^^ -  -
help: add missing type arguments
    |
215 |         iter.map(|x| x.ok_or(())).sum::<Result<T, E_, _>>().ok()


error[E0107]: this enum takes 2 type arguments but 0 type arguments were supplied
    |
    |
231 |         iter.map(|x| x.ok_or(())).product::<Result<_, _>>().ok()
    |                                             ^^^^^^ expected 2 type arguments
    |
note: enum defined here, with 2 type parameters: `T`, `E`
    |
    |
241 | pub enum Result<T, E> {
    |          ^^^^^^ -  -
help: add missing type arguments
    |
231 |         iter.map(|x| x.ok_or(())).product::<Result<T, E_, _>>().ok()


error[E0107]: this enum takes 2 type arguments but 0 type arguments were supplied
     |
     |
1629 |         iter.into_iter().map(|x| x.ok_or(())).collect::<Result<_, _>>().ok()
     |                                                         ^^^^^^ expected 2 type arguments
     |
note: enum defined here, with 2 type parameters: `T`, `E`
     |
     |
241  | pub enum Result<T, E> {
     |          ^^^^^^ -  -
help: add missing type arguments
     |
1629 |         iter.into_iter().map(|x| x.ok_or(())).collect::<Result<T, E_, _>>().ok()


error[E0107]: this function takes 2 type arguments but only 1 type argument was supplied
     |
     |
862  |     transmute::<i8x16, _>(simd_eq(a.as_i8x16(), b.as_i8x16()))
     |     ^^^^^^^^^   ----- supplied 1 type argument
     |     expected 2 type arguments
     |
     |
note: function defined here, with 2 type parameters: `T`, `U`
     |
     |
1084 |     pub fn transmute<T, U>(e: T) -> U;
     |            ^^^^^^^^^ -  -
help: add missing type argument
     |
862  |     transmute::<i8x16, U, _>(simd_eq(a.as_i8x16(), b.as_i8x16()))


error[E0107]: this function takes 2 type arguments but only 1 type argument was supplied
     |
     |
873  |     transmute::<i16x8, _>(simd_eq(a.as_i16x8(), b.as_i16x8()))
     |     ^^^^^^^^^   ----- supplied 1 type argument
     |     expected 2 type arguments
     |
     |
note: function defined here, with 2 type parameters: `T`, `U`
     |
     |
1084 |     pub fn transmute<T, U>(e: T) -> U;
     |            ^^^^^^^^^ -  -
help: add missing type argument
     |
873  |     transmute::<i16x8, U, _>(simd_eq(a.as_i16x8(), b.as_i16x8()))


error[E0107]: this function takes 2 type arguments but only 1 type argument was supplied
     |
     |
884  |     transmute::<i32x4, _>(simd_eq(a.as_i32x4(), b.as_i32x4()))
     |     ^^^^^^^^^   ----- supplied 1 type argument
     |     expected 2 type arguments
     |
     |
note: function defined here, with 2 type parameters: `T`, `U`
     |
     |
1084 |     pub fn transmute<T, U>(e: T) -> U;
     |            ^^^^^^^^^ -  -
help: add missing type argument
     |
884  |     transmute::<i32x4, U, _>(simd_eq(a.as_i32x4(), b.as_i32x4()))


error[E0107]: this function takes 2 type arguments but only 1 type argument was supplied
     |
     |
895  |     transmute::<i8x16, _>(simd_gt(a.as_i8x16(), b.as_i8x16()))
     |     ^^^^^^^^^   ----- supplied 1 type argument
     |     expected 2 type arguments
     |
     |
note: function defined here, with 2 type parameters: `T`, `U`
     |
     |
1084 |     pub fn transmute<T, U>(e: T) -> U;
     |            ^^^^^^^^^ -  -
help: add missing type argument
     |
895  |     transmute::<i8x16, U, _>(simd_gt(a.as_i8x16(), b.as_i8x16()))


error[E0107]: this function takes 2 type arguments but only 1 type argument was supplied
     |
     |
906  |     transmute::<i16x8, _>(simd_gt(a.as_i16x8(), b.as_i16x8()))
     |     ^^^^^^^^^   ----- supplied 1 type argument
     |     expected 2 type arguments
     |
     |
note: function defined here, with 2 type parameters: `T`, `U`
     |
     |
1084 |     pub fn transmute<T, U>(e: T) -> U;
     |            ^^^^^^^^^ -  -
help: add missing type argument
     |
906  |     transmute::<i16x8, U, _>(simd_gt(a.as_i16x8(), b.as_i16x8()))


error[E0107]: this function takes 2 type arguments but only 1 type argument was supplied
     |
     |
917  |     transmute::<i32x4, _>(simd_gt(a.as_i32x4(), b.as_i32x4()))
     |     ^^^^^^^^^   ----- supplied 1 type argument
     |     expected 2 type arguments
     |
     |
note: function defined here, with 2 type parameters: `T`, `U`
     |
     |
1084 |     pub fn transmute<T, U>(e: T) -> U;
     |            ^^^^^^^^^ -  -
help: add missing type argument
     |
917  |     transmute::<i32x4, U, _>(simd_gt(a.as_i32x4(), b.as_i32x4()))


error[E0107]: this function takes 2 type arguments but only 1 type argument was supplied
     |
     |
928  |     transmute::<i8x16, _>(simd_lt(a.as_i8x16(), b.as_i8x16()))
     |     ^^^^^^^^^   ----- supplied 1 type argument
     |     expected 2 type arguments
     |
     |
note: function defined here, with 2 type parameters: `T`, `U`
     |
     |
1084 |     pub fn transmute<T, U>(e: T) -> U;
     |            ^^^^^^^^^ -  -
help: add missing type argument
     |
928  |     transmute::<i8x16, U, _>(simd_lt(a.as_i8x16(), b.as_i8x16()))


error[E0107]: this function takes 2 type arguments but only 1 type argument was supplied
     |
     |
939  |     transmute::<i16x8, _>(simd_lt(a.as_i16x8(), b.as_i16x8()))
     |     ^^^^^^^^^   ----- supplied 1 type argument
     |     expected 2 type arguments
     |
     |
note: function defined here, with 2 type parameters: `T`, `U`
     |
     |
1084 |     pub fn transmute<T, U>(e: T) -> U;
     |            ^^^^^^^^^ -  -
help: add missing type argument
     |
939  |     transmute::<i16x8, U, _>(simd_lt(a.as_i16x8(), b.as_i16x8()))


error[E0107]: this function takes 2 type arguments but only 1 type argument was supplied
     |
     |
950  |     transmute::<i32x4, _>(simd_lt(a.as_i32x4(), b.as_i32x4()))
     |     ^^^^^^^^^   ----- supplied 1 type argument
     |     expected 2 type arguments
     |
     |
note: function defined here, with 2 type parameters: `T`, `U`
     |
     |
1084 |     pub fn transmute<T, U>(e: T) -> U;
     |            ^^^^^^^^^ -  -
help: add missing type argument
     |
950  |     transmute::<i32x4, U, _>(simd_lt(a.as_i32x4(), b.as_i32x4()))


error[E0107]: this function takes 2 type arguments but only 1 type argument was supplied
     |
1421 | /     macro_rules! call {
1421 | /     macro_rules! call {
1422 | |         ($imm3:expr) => {
1423 | |             simd_extract::<_, u16>(a, $imm3) as i32
     | |             ^^^^^^^^^^^^   - supplied 1 type argument
     | |             expected 2 type arguments
1424 | |         };
1425 | |     }
1425 | |     }
     | |_____- in this expansion of `call!` (#2)
1426 |       constify_imm3!(imm8, call)
     |       -------------------------- in this macro invocation (#1)
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/macros.rs:67:1
     |
     |
67   | / macro_rules! constify_imm3 {
68   | |     ($imm8:expr, $expand:ident) => {
69   | |         #[allow(overflowing_literals)]
70   | |         match ($imm8) & 0b111 {
71   | |             0 => $expand!(0),
     | |                  ----------- in this macro invocation (#2)
80   | |     };
81   | | }
81   | | }
     | |_- in this expansion of `constify_imm3!` (#1)
     |
note: function defined here, with 2 type parameters: `T`, `U`
    --> library/core/src/../../stdarch/crates/core_arch/src/simd_llvm.rs:30:12
     |
30   |     pub fn simd_extract<T, U>(x: T, idx: u32) -> U;
     |            ^^^^^^^^^^^^ -  -
help: add missing type argument
     |
1423 |             simd_extract::<_, U, u16>(a, $imm3) as i32


error[E0107]: this function takes 2 type arguments but only 1 type argument was supplied
     |
1421 | /     macro_rules! call {
1421 | /     macro_rules! call {
1422 | |         ($imm3:expr) => {
1423 | |             simd_extract::<_, u16>(a, $imm3) as i32
     | |             ^^^^^^^^^^^^   - supplied 1 type argument
     | |             expected 2 type arguments
1424 | |         };
1425 | |     }
1425 | |     }
     | |_____- in this expansion of `call!` (#2)
1426 |       constify_imm3!(imm8, call)
     |       -------------------------- in this macro invocation (#1)
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/macros.rs:67:1
     |
     |
67   | / macro_rules! constify_imm3 {
68   | |     ($imm8:expr, $expand:ident) => {
69   | |         #[allow(overflowing_literals)]
70   | |         match ($imm8) & 0b111 {
71   | |             0 => $expand!(0),
72   | |             1 => $expand!(1),
     | |                  ----------- in this macro invocation (#2)
80   | |     };
81   | | }
81   | | }
     | |_- in this expansion of `constify_imm3!` (#1)
     |
note: function defined here, with 2 type parameters: `T`, `U`
    --> library/core/src/../../stdarch/crates/core_arch/src/simd_llvm.rs:30:12
     |
30   |     pub fn simd_extract<T, U>(x: T, idx: u32) -> U;
     |            ^^^^^^^^^^^^ -  -
help: add missing type argument
     |
1423 |             simd_extract::<_, U, u16>(a, $imm3) as i32


error[E0107]: this function takes 2 type arguments but only 1 type argument was supplied
     |
1421 | /     macro_rules! call {
1421 | /     macro_rules! call {
1422 | |         ($imm3:expr) => {
1423 | |             simd_extract::<_, u16>(a, $imm3) as i32
     | |             ^^^^^^^^^^^^   - supplied 1 type argument
     | |             expected 2 type arguments
1424 | |         };
1425 | |     }
1425 | |     }
     | |_____- in this expansion of `call!` (#2)
1426 |       constify_imm3!(imm8, call)
     |       -------------------------- in this macro invocation (#1)
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/macros.rs:67:1
     |
     |
67   | / macro_rules! constify_imm3 {
68   | |     ($imm8:expr, $expand:ident) => {
69   | |         #[allow(overflowing_literals)]
70   | |         match ($imm8) & 0b111 {
...    |
73   | |             2 => $expand!(2),
     | |                  ----------- in this macro invocation (#2)
80   | |     };
81   | | }
81   | | }
     | |_- in this expansion of `constify_imm3!` (#1)
     |
note: function defined here, with 2 type parameters: `T`, `U`
    --> library/core/src/../../stdarch/crates/core_arch/src/simd_llvm.rs:30:12
     |
30   |     pub fn simd_extract<T, U>(x: T, idx: u32) -> U;
     |            ^^^^^^^^^^^^ -  -
help: add missing type argument
     |
1423 |             simd_extract::<_, U, u16>(a, $imm3) as i32


error[E0107]: this function takes 2 type arguments but only 1 type argument was supplied
     |
1421 | /     macro_rules! call {
1421 | /     macro_rules! call {
1422 | |         ($imm3:expr) => {
1423 | |             simd_extract::<_, u16>(a, $imm3) as i32
     | |             ^^^^^^^^^^^^   - supplied 1 type argument
     | |             expected 2 type arguments
1424 | |         };
1425 | |     }
1425 | |     }
     | |_____- in this expansion of `call!` (#2)
1426 |       constify_imm3!(imm8, call)
     |       -------------------------- in this macro invocation (#1)
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/macros.rs:67:1
     |
     |
67   | / macro_rules! constify_imm3 {
68   | |     ($imm8:expr, $expand:ident) => {
69   | |         #[allow(overflowing_literals)]
70   | |         match ($imm8) & 0b111 {
...    |
74   | |             3 => $expand!(3),
     | |                  ----------- in this macro invocation (#2)
80   | |     };
81   | | }
81   | | }
     | |_- in this expansion of `constify_imm3!` (#1)
     |
note: function defined here, with 2 type parameters: `T`, `U`
    --> library/core/src/../../stdarch/crates/core_arch/src/simd_llvm.rs:30:12
     |
30   |     pub fn simd_extract<T, U>(x: T, idx: u32) -> U;
     |            ^^^^^^^^^^^^ -  -
help: add missing type argument
     |
1423 |             simd_extract::<_, U, u16>(a, $imm3) as i32


error[E0107]: this function takes 2 type arguments but only 1 type argument was supplied
     |
1421 | /     macro_rules! call {
1421 | /     macro_rules! call {
1422 | |         ($imm3:expr) => {
1423 | |             simd_extract::<_, u16>(a, $imm3) as i32
     | |             ^^^^^^^^^^^^   - supplied 1 type argument
     | |             expected 2 type arguments
1424 | |         };
1425 | |     }
1425 | |     }
     | |_____- in this expansion of `call!` (#2)
1426 |       constify_imm3!(imm8, call)
     |       -------------------------- in this macro invocation (#1)
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/macros.rs:67:1
     |
     |
67   | / macro_rules! constify_imm3 {
68   | |     ($imm8:expr, $expand:ident) => {
69   | |         #[allow(overflowing_literals)]
70   | |         match ($imm8) & 0b111 {
...    |
75   | |             4 => $expand!(4),
     | |                  ----------- in this macro invocation (#2)
80   | |     };
81   | | }
81   | | }
     | |_- in this expansion of `constify_imm3!` (#1)
     |
note: function defined here, with 2 type parameters: `T`, `U`
    --> library/core/src/../../stdarch/crates/core_arch/src/simd_llvm.rs:30:12
     |
30   |     pub fn simd_extract<T, U>(x: T, idx: u32) -> U;
     |            ^^^^^^^^^^^^ -  -
help: add missing type argument
     |
1423 |             simd_extract::<_, U, u16>(a, $imm3) as i32


error[E0107]: this function takes 2 type arguments but only 1 type argument was supplied
     |
1421 | /     macro_rules! call {
1421 | /     macro_rules! call {
1422 | |         ($imm3:expr) => {
1423 | |             simd_extract::<_, u16>(a, $imm3) as i32
     | |             ^^^^^^^^^^^^   - supplied 1 type argument
     | |             expected 2 type arguments
1424 | |         };
1425 | |     }
1425 | |     }
     | |_____- in this expansion of `call!` (#2)
1426 |       constify_imm3!(imm8, call)
     |       -------------------------- in this macro invocation (#1)
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/macros.rs:67:1
     |
     |
67   | / macro_rules! constify_imm3 {
68   | |     ($imm8:expr, $expand:ident) => {
69   | |         #[allow(overflowing_literals)]
70   | |         match ($imm8) & 0b111 {
...    |
76   | |             5 => $expand!(5),
     | |                  ----------- in this macro invocation (#2)
80   | |     };
81   | | }
81   | | }
     | |_- in this expansion of `constify_imm3!` (#1)
     |
note: function defined here, with 2 type parameters: `T`, `U`
    --> library/core/src/../../stdarch/crates/core_arch/src/simd_llvm.rs:30:12
     |
30   |     pub fn simd_extract<T, U>(x: T, idx: u32) -> U;
     |            ^^^^^^^^^^^^ -  -
help: add missing type argument
     |
1423 |             simd_extract::<_, U, u16>(a, $imm3) as i32


error[E0107]: this function takes 2 type arguments but only 1 type argument was supplied
     |
1421 | /     macro_rules! call {
1421 | /     macro_rules! call {
1422 | |         ($imm3:expr) => {
1423 | |             simd_extract::<_, u16>(a, $imm3) as i32
     | |             ^^^^^^^^^^^^   - supplied 1 type argument
     | |             expected 2 type arguments
1424 | |         };
1425 | |     }
1425 | |     }
     | |_____- in this expansion of `call!` (#2)
1426 |       constify_imm3!(imm8, call)
     |       -------------------------- in this macro invocation (#1)
    ::: library/core/src/../../stdarch/crates/core_arch/src/x86/macros.rs:67:1
     |
     |

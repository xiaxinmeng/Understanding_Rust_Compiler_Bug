plain
[00:21:46]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:21:48]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:21:48]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:21:48]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:544 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixsfsi[0]) (const Unevaluated(DefId(0/0:196 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGN_MASK[0]), [f32]):u32): bad constant user type TypeOf(DefId(0/0:196 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGN_MASK[0]), Canonical { variables: [], value: UserSubsts { substs: [f32], user_self_ty: None } }) vs u32: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:173:24
[00:22:21]     |
[00:22:21] 173 |         let sign_bit = <$fty>::SIGN_MASK;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 219 |         float_to_int!(f, f32, i32)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:544 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixsfsi[0]) (const Unevaluated(DefId(0/0:197 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGNIFICAND_MASK[0]), [f32]):u32): bad constant user type TypeOf(DefId(0/0:197 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGNIFICAND_MASK[0]), Canonical { variables: [], value: UserSubsts { substs: [f32], user_self_ty: None } }) vs u32: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:185:36
[00:22:21]     |
[00:22:21] 185 |         let significand = (a_abs & <$fty>::SIGNIFICAND_MASK) | <$fty>::IMPLICIT_BIT;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 219 |         float_to_int!(f, f32, i32)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:544 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixsfsi[0]) (const Unevaluated(DefId(0/0:198 ~ compiler_builtins[69d2]::float[0]::Float[0]::IMPLICIT_BIT[0]), [f32]):u32): bad constant user type TypeOf(DefId(0/0:198 ~ compiler_builtins[69d2]::float[0]::Float[0]::IMPLICIT_BIT[0]), Canonical { variables: [], value: UserSubsts { substs: [f32], user_self_ty: None } }) vs u32: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:185:64
[00:22:21]     |
[00:22:21] 185 |         let significand = (a_abs & <$fty>::SIGNIFICAND_MASK) | <$fty>::IMPLICIT_BIT;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 219 |         float_to_int!(f, f32, i32)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:545 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixsfdi[0]) (const Unevaluated(DefId(0/0:196 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGN_MASK[0]), [f32]):u32): bad constant user type TypeOf(DefId(0/0:196 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGN_MASK[0]), Canonical { variables: [], value: UserSubsts { substs: [f32], user_self_ty: None } }) vs u32: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:173:24
[00:22:21]     |
[00:22:21] 173 |         let sign_bit = <$fty>::SIGN_MASK;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 224 |         float_to_int!(f, f32, i64)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:545 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixsfdi[0]) (const Unevaluated(DefId(0/0:197 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGNIFICAND_MASK[0]), [f32]):u32): bad constant user type TypeOf(DefId(0/0:197 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGNIFICAND_MASK[0]), Canonical { variables: [], value: UserSubsts { substs: [f32], user_self_ty: None } }) vs u32: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:185:36
[00:22:21]     |
[00:22:21] 185 |         let significand = (a_abs & <$fty>::SIGNIFICAND_MASK) | <$fty>::IMPLICIT_BIT;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 224 |         float_to_int!(f, f32, i64)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:545 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixsfdi[0]) (const Unevaluated(DefId(0/0:198 ~ compiler_builtins[69d2]::float[0]::Float[0]::IMPLICIT_BIT[0]), [f32]):u32): bad constant user type TypeOf(DefId(0/0:198 ~ compiler_builtins[69d2]::float[0]::Float[0]::IMPLICIT_BIT[0]), Canonical { variables: [], value: UserSubsts { substs: [f32], user_self_ty: None } }) vs u32: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:185:64
[00:22:21]     |
[00:22:21] 185 |         let significand = (a_abs & <$fty>::SIGNIFICAND_MASK) | <$fty>::IMPLICIT_BIT;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 224 |         float_to_int!(f, f32, i64)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:546 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixsfti[0]) (const Unevaluated(DefId(0/0:196 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGN_MASK[0]), [f32]):u32): bad constant user type TypeOf(DefId(0/0:196 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGN_MASK[0]), Canonical { variables: [], value: UserSubsts { substs: [f32], user_self_ty: None } }) vs u32: NoSolution
[00:22:21] ...
[00:22:21] ...
[00:22:21] 229 |         float_to_int!(f, f32, i128)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:546 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixsfti[0]) (const Unevaluated(DefId(0/0:198 ~ compiler_builtins[69d2]::float[0]::Float[0]::IMPLICIT_BIT[0]), [f32]):u32): bad constant user type TypeOf(DefId(0/0:198 ~ compiler_builtins[69d2]::float[0]::Float[0]::IMPLICIT_BIT[0]), Canonical { variables: [], value: UserSubsts { substs: [f32], user_self_ty: None } }) vs u32: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:185:64
[00:22:21]     |
[00:22:21] 185 |         let significand = (a_abs & <$fty>::SIGNIFICAND_MASK) | <$fty>::IMPLICIT_BIT;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 229 |         float_to_int!(f, f32, i128)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:547 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixdfsi[0]) (const Unevaluated(DefId(0/0:196 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGN_MASK[0]), [f64]):u64): bad constant user type TypeOf(DefId(0/0:196 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGN_MASK[0]), Canonical { variables: [], value: UserSubsts { substs: [f64], user_self_ty: None } }) vs u64: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:173:24
[00:22:21]     |
[00:22:21] 173 |         let sign_bit = <$fty>::SIGN_MASK;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 234 |         float_to_int!(f, f64, i32)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:547 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixdfsi[0]) (const Unevaluated(DefId(0/0:197 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGNIFICAND_MASK[0]), [f64]):u64): bad constant user type TypeOf(DefId(0/0:197 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGNIFICAND_MASK[0]), Canonical { variables: [], value: UserSubsts { substs: [f64], user_self_ty: None } }) vs u64: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:185:36
[00:22:21]     |
[00:22:21] 185 |         let significand = (a_abs & <$fty>::SIGNIFICAND_MASK) | <$fty>::IMPLICIT_BIT;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 234 |         float_to_int!(f, f64, i32)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:547 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixdfsi[0]) (const Unevaluated(DefId(0/0:198 ~ compiler_builtins[69d2]::float[0]::Float[0]::IMPLICIT_BIT[0]), [f64]):u64): bad constant user type TypeOf(DefId(0/0:198 ~ compiler_builtins[69d2]::float[0]::Float[0]::IMPLICIT_BIT[0]), Canonical { variables: [], value: UserSubsts { substs: [f64], user_self_ty: None } }) vs u64: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:185:64
[00:22:21]     12m239 |         float_to_int!(f, f64, i64)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:548 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixdfdi[0]) (const Unevaluated(DefId(0/0:197 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGNIFICAND_MASK[0]), [f64]):u64): bad constant user type TypeOf(DefId(0/0:197 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGNIFICAND_MASK[0]), Canonical { variables: [], value: UserSubsts { substs: [f64], user_self_ty: None } }) vs u64: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:185:36
[00:22:21]     |
[00:22:21] 185 |         let significand = (a_abs & <$fty>::SIGNIFICAND_MASK) | <$fty>::IMPLICIT_BIT;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 239 |         float_to_int!(f, f64, i64)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler errorGNIFICAND_MASK) | <$fty>::IMPLICIT_BIT;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 244 |         float_to_int!(f, f64, i128)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:549 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixdfti[0]) (const Unevaluated(DefId(0/0:198 ~ compiler_builtins[69d2]::float[0]::Float[0]::IMPLICIT_BIT[0]), [f64]):u64): bad constant user type TypeOf(DefId(0/0:198 ~ compiler_builtins[69d2]::float[0]::Float[0]::IMPLICIT_BIT[0]), Canonical { variables: [], value: UserSubsts { substs: [f64], user_self_ty: None } }) vs u64: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:185:64
[00:22:21]     |
[00:22:21] 185 |         let significand = (a_abs & <$fty>::SIGNIFICAND_MASK) | <$fty>::IMPLICIT_BIT;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 244 |         float_to_int!(f, ustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:185:64
[00:22:21]     |
[00:22:21] 185 |         let significand = (a_abs & <$fty>::SIGNIFICAND_MASK) | <$fty>::IMPLICIT_BIT;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 249 |         float_to_int!(f, f32, u32)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:551 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixunssfdi[0]) (const Unevaluated(DefId(0/0:196 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGN_MASK[0]), [f32]):u32): bad constant user type TypeOf(DefId(0/0:196 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGN_MASK[0]), Canonical { variables: [], value: UserSubsts { substs: [f32], user_self_ty: None } }) vs u32: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:173:24
[00:22:21]     |
[00:22:21] 173 |         let sign_bit = <$fty>::SIGN_MASK;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 254 |         float_to_int!(f, f32, u64)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:551 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixunssfdi[0]) (const Unevaluated(DefId(0/0:197 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGNIFICAND_MASK[0]), [f32]):u32): bad constant user type TypeOf(DefId(0/0:197 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGNIFICAND_MASK[0]), Canonical { variables: [], value: UserSubsts { substs: [f32], user_self_ty: None } }) vs u32: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:185:36
[00:22:21]     |
[00:22:21] 185 |         let significand = (a_abs & <$fty>::SIGNIFICAND_MASK) | <$fty>::IMPLICIT_BIT;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 254 |         float_to_int!(f, f32, u64)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:551 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixunssfdi[0]) (const Unevaluated(DefId(0/0:198 ~ compiler_builtins[69d2]::float[0]::Float[0]::IMPLICIT_BIT[0]), [f32]):u32): bad constant user type TypeOf(DefId(0/0:198 ~ compiler_builtins[69d2]::float[0]::Float[0]::IMPLICIT_BIT[0]), Canonical { variables: [], value: UserSubsts { substs: [f32], user_self_ty: None } }) vs u32: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:185:64
[00:22:21]     |
[00:22:21] 185 |         let significand = (a_abs & <$fty>::SIGNIFICAND_MASK) | <$fty>::IMPLICIT_BIT;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 254 |         float_to_int!(f, f32, u64)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:552 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixunssfti[0]) (const Unevaluated(DefId(0/0:196 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGN_MASK[0]), [f32]):u32): bad constant user type TypeOf(DefId(0/0:196 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGN_MASK[0]), Canonical { variables: [], value: UserSubsts { substs: [f32], user_self_ty: None } }) vs u32: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:173:24
[00:22:21]     |
[00:22:21] 173 |         let sign_bit = <$fty>::SIGN_MASK;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 259 |         float_to_int!(f, f32, u128)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:552 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixunssfti[0]) (const Unevaluated(DefId(0/0:197 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGNIFICAND_MASK[0]), [f32]):u32): bad constant user type TypeOf(DefId(0/0:197 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGNIFICAND_MASK[0]), Canonical { variables: [], value: UserSubsts { substs: [f32], user_self_ty: None } }) vs u32: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:185:36
[00:22:21]     |
[00:22:21] 185 |         let significand = (a_abs & <$fty>::SIGNIFICAND_MASK) | <$fty>::IMPLICIT_BIT;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 259 |         float_to_int!(f, f32, u128)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:552 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixunssfti[0]) (const Unevaluated(DefId(0/0:198 ~ compiler_builtins[69d2]::float[0]::Float[0]::IMPLICIT_BIT[0]), [f32]):u32): bad constant user type TypeOf(DefId(0/0:198 ~ compiler_builtins[69d2]::float[0]::Float[0]::IMPLICIT_BIT[0]), Canonical { variables: [], value: UserSubsts { substs: [f32], user_self_ty: None } }) vs u32: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:185:64
[00:22:21]     |
[00:22:21] 185 |         let significand = (a_abs & <$fty>::SIGNIFICAND_MASK) | <$fty>::IMPLICIT_BIT;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 259 |         float_to_int!(f, f32, u128)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:553 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixunsdfsi[0]) (const Unevaluated(DefId(0/0:196 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGN_MASK[0]), [f64]):u64): bad constant user type TypeOf(DefId(0/0:196 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGN_MASK[0]), Canonical { variables: [], value: UserSubsts { substs: [f64], user_self_ty: None } }) vs u64: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:173:24
[00:22:21]     |
[00:22:21] 173 |         let sign_bit = <$fty>::SIGN_MASK;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 264 |         float_to_int!(f, f64, u32)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:553 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixunsdfsi[0]) (const Unevaluated(DefId(0/0:197 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGNIFICAND_MASK[0]), [f64]):u64): bad constant user type TypeOf(DefId(0/0:197 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGNIFICAND_MASK[0]), Canonical { variables: [], value: UserSubsts { substs: [f64], user_self_ty: None } }) vs u64: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:185:36
[00:22:21]     |
[00:22:21] 185 |         let significand = (a_abs & <$fty>::SIGNIFICAND_MASK) | <$fty>::IMPLICIT_BIT;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 264 |         float_to_int!(f, f64, u32)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:553 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixunsdfsi[0]) (const Unevaluated(DefId(0/0:198 ~ compiler_builtins[69d2]::float[0]::Float[0]::IMPLICIT_BIT[0]), [f64]):u64): bad constant user type TypeOf(DefId(0/0:198 ~ compiler_builtins[69d2]::float[0]::Float[0]::IMPLICIT_BIT[0]), Canonical { variables: [], value: Use        let sign_bit = <$fty>::SIGN_MASK;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 269 |         float_to_int!(f, f64, u64)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:554 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixunsdfdi[0]) (const Unevaluated(DefId(0/0:197 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGNIFICAND_MASK[0]), [f64]):u64): bad constant user type TypeOf(DefId(0/0:197 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGNIFICAND_MASK[0]), Canonical { variables: [], value: UserSubsts { substs: [f64], user_self_ty: None } }) vs u64: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:185:36
[00:22:21]     |
[00:22:21] 185 |         let significand = (a_abs & <$fty>::SIGNIFICAND_MASK) | <$fty>::IMPLICIT_BIT;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 269 |         float_to_int!(f, f64, u64)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:554 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixunsdfdi[0]) (const Unevaluated(DefId(0/0:198 ~ compiler_builtins[69d2]::float[0]::Float[0]::IMPLICIT_BIT[0]), [f64]):u64): bad constant user type TypeOf(DefId(0/0:198 ~ compiler_builtins[69d2]::float[0]::Float[0]::IMPLICIT_BIT[0]), Canonical { variables: [], value: UserSubsts { substs: [f64], user_self_ty: None } }) vs u64: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:185:64
[00:22:21]     |
[00:22:21] 185 |         let significand = (a_abs & <$fty>::SIGNIFICAND_MASK) | <$fty>::IMPLICIT_BIT;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 269 |         float_to_int!(f, f64, u64)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:555 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixunsdfti[0]) (const Unevaluated(DefId(0/0:196 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGN_MASK[0]), [f64]):u64): bad constant user type TypeOf(DefId(0/0:196 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGN_MASK[0]), Canonical { variables: [], value: UserSubsts { substs: [f64], user_self_ty: None } }) vs u64: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:173:24
[00:22:21]     |
[00:22:21] 173 |         let sign_bit = <$fty>::SIGN_MASK;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 274 |         float_to_int!(f, f64, u128)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:555 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixunsdfti[0]) (const Unevaluated(DefId(0/0:197 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGNIFICAND_MASK[0]), [f64]):u64): bad constant user type TypeOf(DefId(0/0:197 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGNIFICAND_MASK[0]), Canonical { variables: [], value: UserSubsts { substs: [f64], user_self_ty: None } }) vs u64: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:185:36
[00:22:21]     |
[00:22:21] 185 |         let significand = (a_abs & <$fty>::SIGNIFICAND_MASK) | <$fty>::IMPLICIT_BIT;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 274 |         float_to_int!(f, f64, u128)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:555 ~ compiler_builtins[69d2]::float[0]::conv[0]::__fixunsdfti[0]) (const Unevaluated(DefId(0/0:198 ~ compiler_builtins[69d2]::float[0]::Float[0]::IMPLICIT_BIT[0]), [f64]):u64): bad constant user type TypeOf(DefId(0/0:198 ~ compiler_builtins[69d2]::float[0]::Float[0]::IMPLICIT_BIT[0]), Canonical { variables: [], value: UserSubsts { substs: [f64], user_self_ty: None } }) vs u64: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/conv.rs:185:64
[00:22:21]     |
[00:22:21] 185 |         let significand = (a_abs & <$fty>::SIGNIFICAND_MASK) | <$fty>::IMPLICIT_BIT;
[00:22:21] ...
[00:22:21] ...
[00:22:21] 274 |         float_to_int!(f, f64, u128)
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:578 ~ compiler_builtins[69d2]::float[0]::sub[0]::__subsf3[0]) (const Unevaluated(DefId(0/0:196 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGN_MASK[0]), [f32]):u32): bad constant user type TypeOf(DefId(0/0:196 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGN_MASK[0]), Canonical { variables: [], value: UserSubsts { substs: [f32], user_self_ty: None } }) vs u32: NoSolution
[00:22:21]  --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/sub.rs:8:47
[00:22:21]   |
[00:22:21] 8 |         __addsf3(a, f32::from_repr(b.repr() ^ f32::SIGN_MASK))
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:579 ~ compiler_builtins[69d2]::float[0]::sub[0]::__subdf3[0]) (const Unevaluated(DefId(0/0:196 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGN_MASK[0]), [f64]):u64): bad constant user type TypeOf(DefId(0/0:196 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGN_MASK[0]), Canonical { variables: [], value: UserSubsts { substs: [f64], user_self_ty: None } }) vs u64: NoSolution
[00:22:21]   --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/sub.rs:13:47
[00:22:21]    |
[00:22:21] 13 |         __adddf3(a, f64::from_repr(b.repr() ^ f64::SIGN_MASK))
[00:22:21] 
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:595 ~ compiler_builtins[69d2]::float[0]::{{impl}}[0]::EXPONENT_MASK[0]) (const Unevaluated(DefId(0/0:196 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGN_MASK[0]), [f32]):u32): bad constant user type TypeOf(DefId(0/0:196 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGN_MASK[0]), Canonical { variables: [], value: UserSubsts { substs: [f32], user_self_ty: None } }) vs u32: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/mod.rs:101:48
[00:22:21]     |
[00:22:21] 101 |             const EXPONENT_MASK: Self::Int = !(Self::SIGN_MASK | Self::SIGNIFICAND_MASK);
[00:22:21] ...
[00:22:21] ...
[00:22:21] 134 | float_impl!(f32, u32, i32, 32, 23);
[00:22:21]     | ----------------------------------- in this macro invocation
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:595 ~ compiler_builtins[69d2]::float[0]::{{impl}}[0]::EXPONENT_MASK[0]) (const Unevaluated(DefId(0/0:197 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGNIFICAND_MASK[0]), [f32]):u32): bad constant user type TypeOf(DefId(0/0:197 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGNIFICAND_MASK[0]), Canonical { variables: [], value: UserSubsts { substs: [f32], user_self_ty: None } }) vs u32: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/mod.rs:101:66
[00:22:21]     |
[00:22:21] 101 |             const EXPONENT_MASK: Self::Int = !(Self::SIGN_MASK | Self::SIGNIFICAND_MASK);
[00:22:21] ...
[00:22:21] ...
[00:22:21] 134 | float_impl!(f32, u32, i32, 32, 23);
[00:22:21]     | ----------------------------------- onstant user type TypeOf(DefId(0/0:197 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGNIFICAND_MASK[0]), Canonical { variables: [], value: UserSubsts { substs: [f64], user_self_ty: None } }) vs u64: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/mod.rs:101:66
[00:22:21]     |
[00:22:21] 101 |             const EXPONENT_MASK: Self::Int = !(Self::SIGN_MASK | Self::SIGNIFICAND_MASK);
[00:22:21] ...
[00:22:21] ...
[00:22:21] 135 | float_impl!(f64, u64, i64, 64, 52);
[00:22:21]     | ----------------------------------- in this macro invocation
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:599 ~ compiler_builtins[69d2]::float[0]::{{impl}}[0]::from_parts[0]) (const Unevaluated(DefId(0/0:199 ~ compiler_builtins[69d2]::float[0]::Float[0]::EXPONENT_MASK[0]), [f32]):u32): bad constant user type TypeOf(DefId(0/0:199 ~ compiler_builtins[69d2]::float[0]::Float[0]::EXPONENT_MASK[0]), Canonical { variables: [], value: UserSubsts { substs: [f32], user_self_ty: None } }) vs u32: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/mod.rs:122:61
[00:22:21]     |
[00:22:21] 122 |                     ((exponent << Self::SIGNIFICAND_BITS) & Self::EXPONENT_MASK) |
[00:22:21] ...
[00:22:21] ...
[00:22:21] 134 | float_impl!(f32, u32, i32, 32, 23);
[00:22:21]     | ----------------------------------- in this macro invocation
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:599 ~ compiler_builtins[69d2]::float[0]::{{impl}}[0]::from_parts[0]) (const Unevaluated(DefId(0/0:197 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGNIFICAND_MASK[0]), [f32]):u32): bad constant user type TypeOf(DefId(0/0:197 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGNIFICAND_MASK[0]), Canonical { variables: [], value: UserSubsts { substs: [f32], user_self_ty: None } }) vs u32: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/mod.rs:123:36
[00:22:21]     |
[00:22:21] 123 |                     (significand & Self::SIGNIFICAND_MASK))
[00:22:21]     |                                    n this macro invocation
[00:22:21] 
[00:22:21] error: internal compiler error: broken MIR in DefId(0/0:615 ~ compiler_builtins[69d2]::float[0]::{{impl}}[1]::from_parts[0]) (const Unevaluated(DefId(0/0:197 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGNIFICAND_MASK[0]), [f64]):u64): bad constant user type TypeOf(DefId(0/0:197 ~ compiler_builtins[69d2]::float[0]::Float[0]::SIGNIFICAND_MASK[0]), Canonical { variables: [], value: UserSubsts { substs: [f64], user_self_ty: None } }) vs u64: NoSolution
[00:22:21]    --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/float/mod.rs:123:36
[00:22:21]     |
[00:22:21] 123 |                     (significand & Self::SIGNIFICAND_MASK))
[00:22:21] ...
[00:22:21] ...
[00:22:21] 135 | float_impl!(f64, u64, i64, 64, 52);
[00:22:21]     | ----------------------------------- in this macro invocation
[00:22:21] 
[00:22:21] thread 'main' panicked at 'no errors encountered even though `delay_span_bug` issued', librustc_errors/lib.rs:334:17
[00:22:21] 
[00:22:21] error: internal compiler error: unexpected panic
[00:22:21] 
[00:22:21] note: the compiler unexpectedly p

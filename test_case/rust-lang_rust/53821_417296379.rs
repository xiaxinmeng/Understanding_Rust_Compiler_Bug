plain
[00:22:11]    Compiling serialize v0.0.0 (file:///checkout/src/libserialize)
[00:22:11] error[E0080]: erroneous constant used
[00:22:11]    --> librustc_apfloat/ieee.rs:741:21
[00:22:11]     |
[00:22:11] 741 |                     self = Self::NAN;
[00:22:11]     |                     ^^^^^^^^^^^^^^^^ referenced constant has errors
[00:22:11] error[E0080]: erroneous constant used
[00:22:11]    --> librustc_apfloat/ieee.rs:809:53
[00:22:11]     |
[00:22:11]     |
[00:22:11] 809 |             (Category::Infinity, Category::Zero) => Status::INVALID_OP.and(Self::NAN),
[00:22:11]     |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
[00:22:11] error[E0080]: erroneous constant used
[00:22:11]    --> librustc_apfloat/ieee.rs:971:49
[00:22:11]     |
[00:22:11]     |
[00:22:11] 971 |             (Category::Zero, Category::Zero) => Status::INVALID_OP.and(Self::NAN),
[00:22:11]     |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
[00:22:11] error[E0080]: erroneous constant used
[00:22:11]     --> librustc_apfloat/ieee.rs:1021:36
[00:22:11]      |
[00:22:11]      |
[00:22:11] 1021 |             (_, Category::Zero) => Status::INVALID_OP.and(Self::NAN),
[00:22:11]      |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
[00:22:11] error[E0080]: erroneous constant used
[00:22:11]     --> librustc_apfloat/ieee.rs:1100:17
[00:22:11]      |
[00:22:11]      |
[00:22:11] 1100 |                 Status::OK.and(Self::SMALLEST)
[00:22:11]      |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
[00:22:11] error[E0080]: erroneous constant used
[00:22:11]     --> librustc_apfloat/ieee.rs:1093:44
[00:22:11]      |
[00:22:11]      |
[00:22:11] 1093 |                     Status::INVALID_OP.and(Self::NAN.copy_sign(self))
[00:22:11]      |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
[00:22:11] error[E0080]: erroneous constant used
[00:22:11]     --> librustc_apfloat/ieee.rs:1105:43
[00:22:11]      |
[00:22:11]      |
[00:22:11] 1105 |                     return Status::OK.and(-Self::ZERO);
[00:22:11]      |                                           ^^^^^^^^^^^ referenced constant has errors
[00:22:11] error[E0080]: erroneous constant used
[00:22:11]     --> librustc_apfloat/ieee.rs:1110:28
[00:22:11]      |
[00:22:11]      |
[00:22:11] 1110 |                     return Status::OK.and(Self::INFINITY);
[00:22:11]      |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
[00:22:11] error[E0080]: erroneous constant used
[00:22:11]     --> librustc_apfloat/ieee.rs:1203:45
[00:22:11]      |
[00:22:11]      |
[00:22:11] 1203 |             "inf" | "INFINITY" => return Ok(Status::OK.and(Self::INFINITY)),
[00:22:11]      |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
[00:22:11] error[E0080]: erroneous constant used
[00:22:11]     --> librustc_apfloat/ieee.rs:1204:62
[00:22:11]      |
[00:22:11]      |
[00:22:11] 1204 |             "-inf" | "-INFINITY" => return Ok(Status::OK.and(-Self::INFINITY)),
[00:22:11]      |                                                              ^^^^^^^^^^^^^^^ referenced constant has errors
[00:22:11] error[E0080]: erroneous constant used
[00:22:11]     --> librustc_apfloat/ieee.rs:1205:40
[00:22:11]      |
[00:22:11]      |
[00:22:11] 1205 |             "nan" | "NaN" => return Ok(Status::OK.and(Self::NAN)),
[00:22:11]      |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
[00:22:11] error[E0080]: erroneous constant used
[00:22:11]     --> librustc_apfloat/ieee.rs:1206:57
[00:22:11]      |
[00:22:11]      |
[00:22:11] 1206 |             "-nan" | "-NaN" => return Ok(Status::OK.and(-Self::NAN)),
[00:22:11]      |                                                         ^^^^^^^^^^ referenced constant has errors
[00:22:11] error[E0080]: erroneous constant used
[00:22:11]     --> librustc_apfloat/ieee.rs:1554:17
[00:22:11]      |
[00:22:11]      |
[00:22:11] 1554 |                 (Status::OVERFLOW | Status::INEXACT).and(Self::INFINITY)
[00:22:11]      |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
[00:22:11] error[E0080]: erroneous constant used
[00:22:11]     --> librustc_apfloat/ieee.rs:1810:31
[00:22:11]      |
[00:22:11]      |
[00:22:11] 1810 |             None => return Ok(Status::OK.and(Self::ZERO)),
[00:22:11]      |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
[00:22:11] error[E0080]: erroneous constant used
[00:22:11]     --> librustc_apfloat/ieee.rs:1913:31
[00:22:11]      |
[00:22:11]      |
[00:22:11] 1913 |             None => return Ok(Status::OK.and(Self::ZERO)),
[00:22:11]      |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
[00:22:11] error[E0080]: erroneous constant used
[00:22:11]     --> librustc_apfloat/ieee.rs:1953:17
[00:22:11]      |
[00:22:11]      |
[00:22:11] 1953 |                 IeeeFloat::SMALLEST
[00:22:11]      |                 ^^^^^^^^^^^^^^^^^^^ referenced constant has errors
[00:22:11] error[E0080]: erroneous constant used
[00:22:11]     --> librustc_apfloat/ieee.rs:1955:17
[00:22:11]      |
[00:22:11]      |
[00:22:11] 1955 |                 IeeeFloat::ZERO
[00:22:11]      |                 ^^^^^^^^^^^^^^^ referenced constant has errors
[00:22:11] 
[00:22:11] error: any use of this value will cause an error
[00:22:11]    --> librustc_apfloat/ieee.rs:637:5
[00:22:11]     |
[00:22:11] 637 |     const PRECISION: usize = S::PRECISION;
[00:22:11]     |                              |
[00:22:11]     |                              referenced constant has errors
[00:22:11]     |
[00:22:11]     = note: #[deny(const_err)] on by default
[00:22:11]     = note: #[deny(const_err)] on by default
[00:22:11] 
[00:22:11] error[E0080]: erroneous constant used
[00:22:11]   --> librustc_apfloat/ppc.rs:63:30
[00:22:11]    |
[00:22:11] 63 |     const PRECISION: usize = Fallback::<F>::PRECISION;
[00:22:11]    |                              ^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
[00:22:11] 
[00:22:11] error: any use of this value will cause an error
[00:22:11]    --> librustc_apfloat/ieee.rs:637:5
[00:22:11]     |
[00:22:11] 637 |     const PRECISION: usize = S::PRECISION;
[00:22:11]     |                              |
[00:22:11]     |                              referenced constant has errors
[00:22:11] 
[00:22:11] error[E0080]: erroneous constant used
[00:22:11] error[E0080]: erroneous constant used
[00:22:11]    --> librustc_apfloat/ppc.rs:147:30
[00:22:11]     |
[00:22:11] 147 |     const PRECISION: usize = Fallback::<F>::PRECISION;
[00:22:11]     |                              ^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
[00:22:11] 
[00:22:11] error: any use of this value will cause an error
[00:22:11]    --> librustc_apfloat/ieee.rs:638:5
[00:22:11]     |
[00:22:11] 638 |     const MAX_EXP: ExpInt = S::MAX_EXP;
[00:22:11]     |                             |
[00:22:11]     |                             referenced constant has errors
[00:22:11] 
[00:22:11] error[E0080]: erroneous constant used
[00:22:11] error[E0080]: erroneous constant used
[00:22:11]    --> librustc_apfloat/ppc.rs:148:29
[00:22:11]     |
[00:22:11] 148 |     const MAX_EXP: ExpInt = Fallback::<F>::MAX_EXP;
[00:22:11]     |                             ^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
[00:22:11] 
[00:22:11] error: any use of this value will cause an error
[00:22:11]    --> librustc_apfloat/ieee.rs:639:5
[00:22:11]     |
[00:22:11] 639 |     const MIN_EXP: ExpInt = S::MIN_EXP;
[00:22:11]     |                             |
[00:22:11]     |                             referenced constant has errors
[00:22:11] 
[00:22:11] error[E0080]: erroneous constant used
[00:22:11] error[E0080]: erroneous constant used
[00:22:11]    --> librustc_apfloat/ppc.rs:149:29
[00:22:11]     |
[00:22:11] 149 |     const MIN_EXP: ExpInt = Fallback::<F>::MIN_EXP;
[00:22:11]     |                             ^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
[00:22:11] error[E0080]: erroneous constant used
[00:22:11] error[E0080]: erroneous constant used
[00:22:11]    --> librustc_apfloat/ppc.rs:192:44
[00:22:11]     |
[00:22:11] 192 |                     Status::INVALID_OP.and(Self::NAN.copy_sign(self))
[00:22:11]     |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
[00:22:11] error[E0080]: erroneous constant used
[00:22:11] error[E0080]: erroneous constant used
[00:22:11]    --> librustc_apfloat/ppc.rs:301:53
[00:22:11]     |
[00:22:11] 301 |             (Category::Infinity, Category::Zero) => Status::OK.and(Self::NAN),
[00:22:11]     |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
[00:22:11] error: aborting due to 27 previous errors
[00:22:11] 
[00:22:11] For more information about this error, try `rustc --explain E0080`.
[00:22:11] error: Could not compile `rustc_apfloat`.

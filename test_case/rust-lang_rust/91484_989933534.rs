
   Compiling core v0.0.0 (C:\msys64\home\we\rust\library\core)
error[E0425]: cannot find function `simd_select_bitmask` in module `crate::intrinsics`
   --> library\core\src\..\..\portable-simd\crates\core_simd\src\masks\bitmask.rs:108:32
    |
108 |             crate::intrinsics::simd_select_bitmask(
    |                                ^^^^^^^^^^^^^^^^^^^ not found in `crate::intrinsics`
    |
help: consider importing this function
    |
2   | use crate::simd::intrinsics::simd_select_bitmask;
    |

error[E0425]: cannot find function `simd_bitmask` in module `crate::intrinsics`
   --> library\core\src\..\..\portable-simd\crates\core_simd\src\masks\bitmask.rs:119:42
    |
119 |         unsafe { Self(crate::intrinsics::simd_bitmask(value), PhantomData) }
    |                                          ^^^^^^^^^^^^ not found in `crate::intrinsics`
    |
help: consider importing this function
    |
2   | use crate::simd::intrinsics::simd_bitmask;
    |

For more information about this error, try `rustc --explain E0425`.
error: could not compile `core` due to 2 previous errors

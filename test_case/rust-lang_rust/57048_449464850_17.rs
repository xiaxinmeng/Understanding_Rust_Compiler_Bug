\n"},"level":"error","spans":[{00:59:01] + error[E0441]: unrecognized platform-specific intrinsic function: `x86_mm_adds_epi16`
[00:59:01] 3    |
[00:59:01] 3    |
[00:59:01] 4 LL |     fn x86_mm_adds_epi16(x: A, y: A) -> B;
[00:59:01] 5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:01] 6 
[00:59:01] 6 
[00:59:01] - error[E0443]: intrinsic argument 2 has wrong type: found `B`, expected `A` which was used for this vector type previously in this signature
[00:59:01] + error[E0441]: unrecognized platform-specific intrinsic function: `x86_mm_subs_epi16`
[00:59:01] 9    |
[00:59:01] 9    |
[00:59:01] 10 LL |     fn x86_mm_subs_epi16(x: A, y: B) -> A;
[00:59:01] 11    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:01] 12 
[00:59:01] - error: aborting due to 2 previous errors
[00:59:01] - error: aborting due to 2 previous errors
[00:59:01] + error[E0441]: unrecognized platform-specific intrinsic function: `x86_mm_max_epi16`
[00:59:01] +   --> $DIR/simd-intrinsic-single-nominal-type.rs:29:5
[00:59:01] +    |
[00:59:01] + LL |     fn x86_mm_max_epi16(x: B, y: B) -> B;
[00:59:01] 14 
[00:59:01] - For more information about this error, try `rustc --explain E0443`.
[00:59:01] - For more information about this error, try `rustc --explain E0443`.
[00:59:01] + error[E0441]: unrecognized platform-specific intrinsic function: `x86_mm_min_epi16`
[00:59:01] +   --> $DIR/simd-intrinsic-single-nominal-type.rs:30:5
[00:59:01] +    |
[00:59:01] + LL |     fn x86_mm_min_epi16(x: A, y: A) -> A;
[00:59:01] + 
[00:59:01] + 
[00:59:01] + error: aborting due to 4 p"E0441","explanation":"\nAn unknown platform-specific intrinsic function was used. Erroneous\ncode example:\n\n
text
  --> src\ec\suite_b\ops\p384.rs:69:20
   |
69 |     elem_sqr_mont: |r, a| GFp_p384_elem_mul_mont(r, a, a),
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected unsafe fn, found normal fn
   |
   = note: expected type `unsafe extern "C" fn(*mut u64, *const u64)`
              found type `[closure@src\ec\suite_b\ops\p384.rs:69:20: 69:58]`

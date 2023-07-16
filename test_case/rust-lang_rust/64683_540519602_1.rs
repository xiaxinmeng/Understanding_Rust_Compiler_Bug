rust
if_miri(
  || do_fallback(),
  || if have_simd() { do_simd() } else { do_fallback() } // not const-checked
)

rust
#[inline(always)] 
fn foo_impl()  {
  // ...generic implementation here...
  #[cfg(traget_feature = "+avx")] {
     // some avx specific optimizaiton here
  }
  #[cfg(not(target_feature = "+avx"))] {
    // some non-avx generic alternative here
  }
  // ... more generic implementation ...
}

#[target_feature = "+avx"] fn foo_avx() { foo_impl() }
#[target_feature = "+sse3"] fn foo_sse3() { foo_impl() }

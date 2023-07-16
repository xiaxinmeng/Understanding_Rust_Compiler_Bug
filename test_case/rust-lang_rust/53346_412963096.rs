rust
extern "C" {
    #[cfg(target_feature = "avx")]
    fn foo(x: __m256) -> _m256;   // OK
    #[cfg(target_feature = "sse")]
    fn bar(x: __m128) -> __m128; // OK
    
    fn baz(x: __m256); // ERROR IFF cfg!(target_feature="avx") is false
}

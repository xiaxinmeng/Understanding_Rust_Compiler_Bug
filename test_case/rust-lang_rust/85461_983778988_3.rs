rust
// src/memmem/mod.rs
mod genericsimd {
    use crate::memmem::PrefilterFnTy;
    
    #[inline(always)]
    pub(crate) fn find(
        haystack: &(),
        fallback: PrefilterFnTy,
    ) { }
    
    fn find_in_chunk2() { }    
}

mod x86 {
    pub(crate) mod avx {
        use core::arch::x86_64::__m256i;
    
        use crate::memmem::PrefilterFnTy;
        
        const _: PrefilterFnTy = find;
        
        pub(crate) fn find(
            haystack: &(),
        ) {
            super::super::genericsimd::find(
                haystack,
                super::sse::find2,
            )
        }
    }
    
    pub(crate) mod sse {
        use core::arch::x86_64::__m128i;

        pub(crate) fn find2(
            haystack: &(),
        ) {
            super::super::genericsimd::find(
                haystack,
                |_| { },
            )
        }   
    }    
}

#[derive(Debug)]
struct NeedleInfo { }

type PrefilterFnTy = fn(
    haystack: &(),
);

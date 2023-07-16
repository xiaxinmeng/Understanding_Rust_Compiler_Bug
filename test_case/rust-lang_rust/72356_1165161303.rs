rust
    let (prefix, simd, suffix) = unsafe { buffer.align_to::<core::arch::x86_64::__m128i>() };
    unsafe {std::intrinsics::assume(prefix.len() < 16);}

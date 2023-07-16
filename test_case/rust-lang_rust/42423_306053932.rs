rust
// -Copt-level=3 -Ctarget-cpu=native -Cpanic=abort 

#![crate_type="lib"]
#![feature(link_llvm_intrinsics)]

extern {
    #[link_name = "llvm.minnum.f32"]
    pub fn minnum_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.maxnum.f32"]
    pub fn maxnum_f32(a: f32, b: f32) -> f32;
}

pub trait MinMaxTest: Sized {
    fn min_llvm(self, other: Self) -> Self;
    fn min_rust(self, other: Self) -> Self;
    fn max_llvm(self, other: Self) -> Self;
    fn max_rust(self, other: Self) -> Self;
}

impl MinMaxTest for f32 {
    /*
    x86_64:
        _<f32 as 2::MinMaxTest>::min_llvm::hd76590e5233a75ba:
            pushq	%rbp
            movq	%rsp, %rbp
            vminss	%xmm1, %xmm0, %xmm2
            vcmpunordss	%xmm1, %xmm1, %xmm1
            vblendvps	%xmm1, %xmm0, %xmm2, %xmm0
            popq	%rbp
            retq
    
    aarch64:
        _<f32 as 2::MinMaxTest>::min_llvm::hd76590e5233a75ba:
            fminnm	s0, s1, s0
            ret
    */
    fn min_llvm(self, other: Self) -> Self {
        unsafe { minnum_f32(other, self) }     
    }

    /*
    x86_64:
        _<f32 as 2::MinMaxTest>::min_rust::h6a6994ac4726b5ad:
            pushq	%rbp
            movq	%rsp, %rbp
            vminss	%xmm1, %xmm0, %xmm2
            vcmpunordss	%xmm1, %xmm1, %xmm1
            vblendvps	%xmm1, %xmm0, %xmm2, %xmm0
            popq	%rbp
            retq
    
    aarch64:
        _<f32 as 2::MinMaxTest>::min_rust::h6a6994ac4726b5ad:
            fcmp	s1, s0
            fccmp	s1, s1, #1, le
            fcsel	s0, s0, s1, vs
            ret
    */
    fn min_rust(self, other: Self) -> Self {
        if other.is_nan() || other > self { self } else { other }
    }

    /*
    x86_64:
        _<f32 as 2::MinMaxTest>::max_llvm::h8b7b369f1c3662de:
            pushq	%rbp
            movq	%rsp, %rbp
            vmaxss	%xmm1, %xmm0, %xmm2
            vcmpunordss	%xmm1, %xmm1, %xmm1
            vblendvps	%xmm1, %xmm0, %xmm2, %xmm0
            popq	%rbp
            retq

    aarch64:
        _<f32 as 2::MinMaxTest>::max_llvm::h8b7b369f1c3662de:
            fmaxnm	s0, s1, s0
            ret
    */
    fn max_llvm(self, other: Self) -> Self {
        unsafe { maxnum_f32(other, self) }
    }

    /*
    x86_64:
        _<f32 as 2::MinMaxTest>::max_rust::hb29df4916b2f44f3:
            pushq	%rbp
            movq	%rsp, %rbp
            vmaxss	%xmm1, %xmm0, %xmm2
            vcmpunordss	%xmm1, %xmm1, %xmm1
            vblendvps	%xmm1, %xmm0, %xmm2, %xmm0
            popq	%rbp
            retq

    aarch64:
        _<f32 as 2::MinMaxTest>::max_rust::hb29df4916b2f44f3:
            fcmp	s1, s0
            fccmp	s1, s1, #1, pl
            fcsel	s0, s0, s1, vs
            ret
    */
    fn max_rust(self, other: Self) -> Self {
        if other.is_nan() || other < self { self } else { other }
    }

}

/*
x86_64:
    __2::clamp01_rfc1961::hf9aa1066c6c8cd7e:
        pushq	%rbp
        movq	%rsp, %rbp
        vxorps	%xmm1, %xmm1, %xmm1
        vmaxss	%xmm0, %xmm1, %xmm0
        vmovss	LCPI4_0(%rip), %xmm1
        vminss	%xmm0, %xmm1, %xmm0
        popq	%rbp
        retq

aarch64:
    __2::clamp01_rfc1961::hf9aa1066c6c8cd7e:
        movi.2d	v1, #0000000000000000
        fmax	s0, s0, s1
        fmov	s1, #1.00000000
        fmin	s0, s0, s1
        ret
*/
pub fn clamp01_rfc1961(mut x: f32) -> f32 {
    if x < 0.0 { x = 0.0; }
    if x > 1.0 { x = 1.0; }
    x
}

/*
x86_64:
    __2::clamp01_llvm::h30ee56b2e9fb6a19:
        pushq	%rbp
        movq	%rsp, %rbp
        vxorps	%xmm1, %xmm1, %xmm1
        vmaxss	%xmm0, %xmm1, %xmm1
        vcmpunordss	%xmm0, %xmm0, %xmm0
        vandnps	%xmm1, %xmm0, %xmm0
        vmovss	LCPI5_0(%rip), %xmm1
        vminss	%xmm0, %xmm1, %xmm2
        vcmpunordss	%xmm0, %xmm0, %xmm0
        vandnps	%xmm2, %xmm0, %xmm2
        vandps	%xmm1, %xmm0, %xmm0
        vorps	%xmm0, %xmm2, %xmm0
        popq	%rbp
        retq

aarch64:
    __2::clamp01_llvm::h30ee56b2e9fb6a19:
        movi.2d	v1, #0000000000000000
        fmaxnm	s0, s0, s1
        fmov	s1, #1.00000000
        fminnm	s0, s0, s1
        ret
*/
pub fn clamp01_llvm(x: f32) -> f32 {
    x.max_llvm(0.0).min_llvm(1.0)
}

/*
x86_64:
    __2::clamp01_rust::h324077e72816ec64:
        pushq	%rbp
        movq	%rsp, %rbp
        vxorps	%xmm1, %xmm1, %xmm1
        vmaxss	%xmm1, %xmm0, %xmm0
        vminss	LCPI6_0(%rip), %xmm0, %xmm0
        popq	%rbp
        retq

aarch64:
    __2::clamp01_rust::h324077e72816ec64:
        movi.2d	v1, #0000000000000000
        fmaxnm	s0, s0, s1
        fmov	s1, #1.00000000
        fminnm	s0, s0, s1
        ret
*/
pub fn clamp01_rust(x: f32) -> f32 {
    x.max_rust(0.0).min_rust(1.0)
}

/*
x86_64:
    __2::clamp01_std::h5df114f63023c56b:
        pushq	%rbp
        movq	%rsp, %rbp
        vxorps	%xmm1, %xmm1, %xmm1
        callq	_fmaxf
        vmovss	LCPI7_0(%rip), %xmm1
        popq	%rbp
        jmp	_fminf

aarch64:
    __2::clamp01_std::h5df114f63023c56b:
        stp	x29, x30, [sp, #-16]!
        mov	 x29, sp
        movi.2d	v1, #0000000000000000
        bl	_fmaxf
        fmov	s1, #1.00000000
        ldp	x29, x30, [sp], #16
        b	_fminf
*/
pub fn clamp01_std(x: f32) -> f32 {
    x.max(0.0).min(1.0)
}

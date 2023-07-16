rust
#[no_mangle] #[inline(never)] pub fn better(x: f32) -> f32 {
    fmin(fmax(x, 0.), 1.)
}

=>

better:
	.cfi_startproc
	xorps	xmm1, xmm1
	cmpless	xmm1, xmm0
	andps	xmm0, xmm1
	minss	xmm0, dword ptr [rip + .LCPI0_0]
	ret

ptx

.visible .func  (.param .b32 func_retval0) _ZN59_$LT$nvkernel..MyStruct$u20$as$u20$core..cmp..PartialEq$GT$2eq17hebe67210677cfb7eE(
	.param .b64 _ZN59_$LT$nvkernel..MyStruct$u20$as$u20$core..cmp..PartialEq$GT$2eq17hebe67210677cfb7eE_param_0,
	.param .b64 _ZN59_$LT$nvkernel..MyStruct$u20$as$u20$core..cmp..PartialEq$GT$2eq17hebe67210677cfb7eE_param_1
)
{
	.reg .pred 	%p<2>;
	.reg .b32 	%r<4>;
	.reg .b64 	%rd<3>;

	ld.param.u64 	%rd1, [_ZN59_$LT$nvkernel..MyStruct$u20$as$u20$core..cmp..PartialEq$GT$2eq17hebe67210677cfb7eE_param_0];
	ld.u32 	%r1, [%rd1];
	ld.param.u64 	%rd2, [_ZN59_$LT$nvkernel..MyStruct$u20$as$u20$core..cmp..PartialEq$GT$2eq17hebe67210677cfb7eE_param_1];
	ld.u32 	%r2, [%rd2];
	setp.eq.s32 	%p1, %r1, %r2;
	selp.u32 	%r3, 1, 0, %p1;
	st.param.b32 	[func_retval0+0], %r3;
	ret;
}

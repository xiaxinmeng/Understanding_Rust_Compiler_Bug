 ptx
.visible .func  (.param .align 16 .b8 func_retval0[16]) internal(
	.param .align 16 .b8 internal_param_0[16],
	.param .align 16 .b8 internal_param_1[16]
)
{
	.reg .b64 	%rd<11>;

	ld.param.u64 	%rd1, [internal_param_0];
	ld.param.u64 	%rd2, [internal_param_0+8];
	ld.param.u64 	%rd3, [internal_param_1+8];
	ld.param.u64 	%rd4, [internal_param_1];
	mul.lo.s64 	%rd5, %rd4, %rd2;
	mul.hi.u64 	%rd6, %rd4, %rd1;
	add.s64 	%rd7, %rd6, %rd5;
	mul.lo.s64 	%rd8, %rd3, %rd1;
	add.s64 	%rd9, %rd7, %rd8;
	mul.lo.s64 	%rd10, %rd4, %rd1;
	st.param.v2.b64	[func_retval0+0], {%rd10, %rd9};
	ret;
}

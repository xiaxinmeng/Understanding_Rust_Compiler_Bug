
.visible .func  (.param .b64 func_retval0) foo(
        .param .b64 foo_param_0
)
{

        ld.param.u64    %rd2, [foo_param_0];
        mov.u64 %rd1, %rd2
        st.param.b64    [func_retval0+0], %rd1;
        ret;

}

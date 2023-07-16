
pub unsafe extern "C" fn callee(arg: Franta) {
    tst_use(arg.a);
    tst_use(arg.b);
    tst_use(arg.c);
    tst_use(arg.d);
}

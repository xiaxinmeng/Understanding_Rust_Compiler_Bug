
// assembly-output: emit-asm
// needs-llvm-components: sparc
// compile-flags: --target=sparcv9-sun-solaris
#![crate_type = "lib"]

#[repr(C)]
pub struct Franta {
        a:f32,
        b:f32,
        c:f32,
        d:f32,
}

// CHECK-LABEL: callee
#[no_mangle]
pub unsafe extern "C" fn callee(arg: Franta) {
    // CHECK: st %f3, [%fp+2043]
    // CHECK: st %f2, [%fp+2039]
    // CHECK: st %f1, [%fp+2035]
    // CHECK: fmovs %f0, %f1
    tst_use(arg.a);
    tst_use(arg.b);
    tst_use(arg.c);
    tst_use(arg.d);
}

extern "C" {
    fn opaque_callee(arg: Franta, intarg: i32);
    fn tst_use(arg: f32);
}

// CHECK-LABEL: caller
// CHECK: ld [%i1], %f0
// CHECK: ld [%i2], %f1
// CHECK: ld [%i3], %f2
// CHECK: ld [%i0], %f3
// CHECK: call {{.*}}
// CHECK: mov     3, %o2
pub unsafe extern "C" fn caller() {
    opaque_callee(Franta { a: 1.0, b: 2.0, c: 3.0, d: 4.0 }, 3);
}

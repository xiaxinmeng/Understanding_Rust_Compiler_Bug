rust
// compiler-flags: -Copt-level=3 --target=sparc...

#[repr(C)]
pub struct Franta {
        a:f32,
        b:f32,
        c:f32,
        d:f32,
}

// CHECK-LABEL: callee
#[no_mangle]
pub extern "C" fn callee(arg: Franta) {
    // CHECK: fmovs   %f?, %f0
    // CHECK: call use
    use(arg.a); 
    // CHECK: fmovs   %f?, %f0
    // CHECK: call use
    use(arg.b); 
    // CHECK: fmovs   %f?, %f0
    // CHECK: call use
    use(arg.c);
    // CHECK: fmovs   %f?, %f0
    // CHECK: call use
    use(arg.d);
}

extern "C" {
    fn opaque_callee(arg: Franta, intarg: i32);
    fn use(arg: f32);
}

// CHECK-LABEL: caller
// CHECK: mov       0x3, %o2
// CHECK: fmovs     %{{.*}}, %f0
// CHECK: fmovs     %{{.*}}, %f1
// CHECK: fmovs     %{{.*}}, %f2
// CHECK: fmovs     %{{.*}}, %f3
// CHECK: call {{.*}}
pub unsafe extern "C" fn caller() {
    opaque_callee(Franta { a: 1.0, b: 2.0, c: 3.0, d: 4.0 }, 3);
}

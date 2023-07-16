
    // CHECK: @f1(i32 inreg %arg0, i32 inreg %arg1, i32 %arg2)
    #[no_mangle]
    pub extern "fastcall" fn f1(_: i32, _: i32, _: i32) {}

    // CHECK: @f2(i32* inreg %arg0, i32* inreg %arg1, i32* %arg2)
    #[no_mangle]
    pub extern "fastcall" fn f2(_: *const i32, _: *const i32, _: *const i32) {}

    // CHECK: @f3(float %arg0, i32 inreg %arg1, i32 inreg %arg2, i32 %arg3)
    #[no_mangle]
    pub extern "fastcall" fn f3(_: f32, _: i32, _: i32, _: i32) {}

    // CHECK: @f4(i32 inreg %arg0, float %arg1, i32 inreg %arg2, i32 %arg3)
    #[no_mangle]
    pub extern "fastcall" fn f4(_: i32, _: f32, _: i32, _: i32) {}

    // CHECK: @f5(i64 %arg0, i32 %arg1)
    #[no_mangle]
    pub extern "fastcall" fn f5(_: i64, _: i32) {}

    // CHECK: @f6(i1 inreg zeroext %arg0, i32 inreg %arg1, i32 %arg2)
    #[no_mangle]
    pub extern "fastcall" fn f6(_: bool, _: i32, _: i32) {}

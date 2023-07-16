
unsafe fn ret_val() -> SomeStruct {
    let mut val: SomeStruct = uninitialized();
    SomeExternFun(&mut val);
    val
}

pub unsafe fn call_ret_val() {
     let val = ret_val();
     SomeExternFun2(&val);
}

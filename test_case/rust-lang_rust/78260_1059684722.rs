
#[no_mangle]
unsafe extern "avr-interrupt" fn __vector_2() {
    llvm_asm!("":::"r18" "r19" "r20" "r21" "r22" "r23" "r24" "r25" "r26" "r27" "r30" "r31");
    function_call();
}

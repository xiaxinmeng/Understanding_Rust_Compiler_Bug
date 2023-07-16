
420015ec <esp_riscv_rt::_trap_rust_abi_shim1>:
extern "C" fn _trap_rust_abi_shim1<A, R>(arg: A, f: fn(A) -> R) -> R {
420015ec:       1101                    addi    sp,sp,-32
420015ee:       ce06                    sw      ra,28(sp)
420015f0:       4110                    lw      a2,0(a0)
420015f2:       4154                    lw      a3,4(a0)
420015f4:       4518                    lw      a4,8(a0)
420015f6:       4548                    lw      a0,12(a0)
    f(arg)
420015f8:       ca2a                    sw      a0,20(sp)
420015fa:       c83a                    sw      a4,16(sp)
420015fc:       c636                    sw      a3,12(sp)
420015fe:       0028                    addi    a0,sp,8
42001600:       c432                    sw      a2,8(sp)
42001602:       9582                    jalr    a1
}
42001604:       40f2                    lw      ra,28(sp)
42001606:       6105                    addi    sp,sp,32
42001608:       8082                    ret

4200160a <esp_riscv_rt::_trap_rust_abi_shim1>:
    f(arg)
4200160a:       8582                    jr      a1

rust
extern "C" fn add(x: i32, y: i32) -> i32 {
    x + y
}

// AArch64
let result: i32;
asm!(
    "call {}",
    sym add,
    in("x0") 1,
    in("x1") 2,
    lateout("x0") result,
    clobber_abi("C"),
);
assert_eq!(result, 3);

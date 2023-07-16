
// compile-flags: -O -C no-prepopulate-passes

#[repr(u16)]
pub enum E2 {
    A = 13,
    B = 42,
}

// For optimized code we keep an unreachable target so LLVM knows the possible values

// CHECK-LABEL: @exhaustive_match_2
#[no_mangle]
pub fn exhaustive_match_2(e: E2) -> u8 {
    // CHECK: switch i16 %{{.+}}, label %[[UNREACH:.+]] [
    // CHECK-NEXT: i16 13,
    // CHECK-NEXT: i16 42,
    // CHECK-NEXT: ]
    // CHECK: [[UNREACH]]:
    // CHECK-NEXT: unreachable
    match e {
        E2::A => 0,
        E2::B => 1,
    }
}

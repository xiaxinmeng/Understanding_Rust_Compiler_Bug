rust
#![feature(link_llvm_intrinsics)]
extern {
    #[link_name="llvm.sadd.sat.i32"]
    fn add_sat_i32(x: i32, y: i32) -> i32;
    #[link_name="llvm.uadd.sat.i32"]
    fn add_sat_u32(x: u32, y: u32) -> u32;
}
pub unsafe fn test_sfold(x: i32) -> i32 {
    add_sat_i32(add_sat_i32(x, 10), 20)
}
pub unsafe fn test_ufold(x: u32) -> u32 {
    add_sat_u32(add_sat_u32(x, 10), 20)
}

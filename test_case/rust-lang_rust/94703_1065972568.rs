Rust
// CHECK: .visible .entry f_u32_u16_tuple_arg(
// CHECK: .param .u32 f_u32_u16_tuple_arg_param_0
// CHECK: .param .u16 f_u32_u16_tuple_arg_param_1
#[no_mangle]
pub unsafe extern "ptx-kernel" fn f_u32_u16_tuple_arg(a: (u32, u16)) {}

// CHECK: .visible .entry f_u8_u8_u32_tuple_arg(
// CHECK: .param .align 4 .b8 f_u8_u8_u32_tuple_arg_param_0[8]
#[no_mangle]
pub unsafe extern "ptx-kernel" fn f_u8_u8_u32_tuple_arg(a: (u8, u8, u32)) {}

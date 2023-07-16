
enum basic_opcode { ... }
fn basic_opcode(v: uint) -> basic_opcode {
    unsafe { unsafe::reinterpret_cast(v) }
}

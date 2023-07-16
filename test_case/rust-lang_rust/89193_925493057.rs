rust
    // FIXME: use:
    //  https://github.com/llvm-mirror/llvm/blob/master/include/llvm/IR/Function.h#L182
    //  https://github.com/llvm-mirror/llvm/blob/master/include/llvm/IR/Intrinsics.h#L81
    fn llvm_vector_str(elem_ty: Ty<'_>, vec_len: u64, no_pointers: usize) -> String {
        let p0s: String = "p0".repeat(no_pointers);
        match *elem_ty.kind() {
            ty::Int(v) => format!("v{}{}i{}", vec_len, p0s, v.bit_width().unwrap()),
            ty::Uint(v) => format!("v{}{}i{}", vec_len, p0s, v.bit_width().unwrap()),
            ty::Float(v) => format!("v{}{}f{}", vec_len, p0s, v.bit_width()),
            _ => unreachable!(),
        }
    }
 
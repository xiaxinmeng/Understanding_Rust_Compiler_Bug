 rust
do member_name.as_c_str |member_name| { unsafe {
    llvm::LLVMDIBuilderCreateMemberType(
        DIB(cx),
        file_metadata,
        ...,
        0,
        member_type_metadata[i])
}}

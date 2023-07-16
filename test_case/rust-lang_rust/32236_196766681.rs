 Rust

pub static INTRINSICS: &'static [Intrinsic] = &[
        Intrinsic {
            name: "movemask_ps",
            inputs: &[&::F32x4],
            output: &::I32,
            definition: Named("llvm.x86.sse.movmsk.ps")
        },
        Intrinsic {
            name: "storeu_ps",
            inputs: &[&Type::Pointer(&::F32, Some(&::I8), false), &::F32x4],
            output: &::VOID,
            definition: Named("llvm.x86.sse.storeu.ps")
        },
];

plain
   Compiling rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
   Compiling rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
   Compiling rustc_transmute v0.1.0 (/checkout/compiler/rustc_transmute)
error: usage of `ty::TyKind::<kind>`
   --> compiler/rustc_transmute/src/nfa.rs:133:13
    |
133 |             TyKind::Bool => Ok(Self::bool()),
    |             ^^^^^^ help: try using `ty::<kind>` directly: `ty`
    |
    = note: `-D rustc::usage-of-ty-tykind` implied by `-D warnings`

error: usage of `ty::TyKind::<kind>`
   --> compiler/rustc_transmute/src/nfa.rs:135:13
    |
135 |             TyKind::Int(I8) | TyKind::Uint(U8) => Ok(Self::u8()),
    |             ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   --> compiler/rustc_transmute/src/nfa.rs:135:31
    |
135 |             TyKind::Int(I8) | TyKind::Uint(U8) => Ok(Self::u8()),
    |                               ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   --> compiler/rustc_transmute/src/nfa.rs:136:13
    |
136 |             TyKind::Int(I16) | TyKind::Uint(U16) => Ok(Self::number(2)),
    |             ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   --> compiler/rustc_transmute/src/nfa.rs:136:32
    |
136 |             TyKind::Int(I16) | TyKind::Uint(U16) => Ok(Self::number(2)),
    |                                ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   --> compiler/rustc_transmute/src/nfa.rs:137:13
    |
137 |             TyKind::Int(I32) | TyKind::Uint(U32) | TyKind::Float(F32) => Ok(Self::number(4)),
    |             ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   --> compiler/rustc_transmute/src/nfa.rs:137:32
    |
137 |             TyKind::Int(I32) | TyKind::Uint(U32) | TyKind::Float(F32) => Ok(Self::number(4)),
    |                                ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   --> compiler/rustc_transmute/src/nfa.rs:137:52
    |
137 |             TyKind::Int(I32) | TyKind::Uint(U32) | TyKind::Float(F32) => Ok(Self::number(4)),
    |                                                    ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   --> compiler/rustc_transmute/src/nfa.rs:138:13
    |
138 |             TyKind::Int(I64) | TyKind::Uint(U64) | TyKind::Float(F64) => Ok(Self::number(8)),
    |             ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   --> compiler/rustc_transmute/src/nfa.rs:138:32
    |
138 |             TyKind::Int(I64) | TyKind::Uint(U64) | TyKind::Float(F64) => Ok(Self::number(8)),
    |                                ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   --> compiler/rustc_transmute/src/nfa.rs:138:52
    |
138 |             TyKind::Int(I64) | TyKind::Uint(U64) | TyKind::Float(F64) => Ok(Self::number(8)),
    |                                                    ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   --> compiler/rustc_transmute/src/nfa.rs:139:13
    |
139 |             TyKind::Int(I128) | TyKind::Uint(U128) => Ok(Self::number(16)),
    |             ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   --> compiler/rustc_transmute/src/nfa.rs:139:33
    |
139 |             TyKind::Int(I128) | TyKind::Uint(U128) => Ok(Self::number(16)),
    |                                 ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   --> compiler/rustc_transmute/src/nfa.rs:140:13
    |
140 |             TyKind::Int(Isize) | TyKind::Uint(Usize) => {
    |             ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   --> compiler/rustc_transmute/src/nfa.rs:140:34
    |
140 |             TyKind::Int(Isize) | TyKind::Uint(Usize) => {
    |                                  ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   --> compiler/rustc_transmute/src/nfa.rs:144:13
    |
144 |             TyKind::Adt(adt_def, substs_ref) => {
    |             ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   --> compiler/rustc_transmute/src/nfa.rs:314:29
    |
314 |         let parent = if let TyKind::Adt(adt_def, ..) = scope.kind() {
    |                             ^^^^^^ help: try using `ty::<kind>` directly: `ty`
error: could not compile `rustc_transmute` due to 17 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:07:51

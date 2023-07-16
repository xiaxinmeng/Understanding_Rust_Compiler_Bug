
error: internal compiler error: compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:856:33: unexpected path: def=aes_gcm::aead::generic_array::GenericArray substs=[_, _] path=Path { span: src/services/encryption.rs:53:25: 53:30 (#0), res: Def(TyAlias, DefId(180:97 ~ aes_gcm[7ab9]::Nonce)), segments: [PathSegment { ident: Nonce#0, hir_id: Some(HirId { owner: DefId(0:223 ~ easylink[c894]::services::encryption::encode), local_id: 61 }), res: Some(Err), args: None, infer_args: true }] }

/* backtrace */

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (50b00252a 2022-06-06) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C embed-bitcode=no -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `services::encryption::encode`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack

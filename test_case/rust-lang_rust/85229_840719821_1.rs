
error: internal compiler error: compiler/rustc_mir/src/const_eval/mod.rs:160:14: cannot destructure constant Const { ty: &[usize], val: Value(ByRef { alloc: Allocation { bytes: [0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [(Size { raw: 0 }, ((), alloc2))] }), init_mask: InitMask { blocks: [65535], len: Size { raw: 16 } }, size: Size { raw: 16 }, align: Align { pow2: 3 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }

thread 'rustc' panicked at 'Box<Any>', /rustc/21e92b97309e15b16bc6b8dd4509d5e3ad4c430d/library/std/src/panic.rs:59:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (21e92b973 2021-05-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [destructure_const] destructure constant
#1 [type_op_ascribe_user_type] evaluating `type_op_ascribe_user_type` `Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: UserFacing }, value: AscribeUserType { mir_ty: fn() {foo::<{&[1, 2, 3, 4]}>}, def_id: DefId(0:3 ~ playground[b426]::foo), user_substs: UserSubsts { substs: [Const { ty: &'static [usize], val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:6 ~ playground[b426]::main::{constant#0}), const_param_did: Some(DefId(0:4 ~ playground[b426]::foo::X)) }, substs: [], promoted: None }) }], user_self_ty: None } } } }`
end of query stack


thread 'rustc' panicked at 'index out of bounds: the len is 2 but the index is 2', /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.75.0/src/lib.rs:2748:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-nightly (475b00aa4 2021-12-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z chalk -C debuginfo=1 -C linker=/opt/compiler-explorer/gcc-11.1.0/bin/gcc --crate-type bin

query stack during panic:
#0 [evaluate_goal] evaluating trait selection obligation `environment: [Binder(TypeWellFormedFromEnv(impl for<'a> Fn(&Borrowed<'_>) -> bool), []), Binder(ProjectionPredicate(ProjectionTy { substs: [impl for<'a> Fn(&Borrowed<'_>) -> bool, (&Borrowed<'_>,)], item_def_id: DefId(2:3268 ~ core[5910]::ops::function::FnOnce::Output) }, bool), [Region(BrNamed(DefId(0:16 ~ example[a33f]::borrowed::{opaque#0}::'a), 'a)), Region(BrAnon(0)), Region(BrAnon(1))]), Binder(TraitPredicate(<impl for<'a> Fn(&Borrowed<'_>) -> bool as core::ops::function::Fn<(&Borrowed<'_>,)>>, polarity:Positive), [Region(BrNamed(DefId(0:16 ~ example[a33f]::borrowed::{opaque#0}::'a), 'a)), Region(BrAnon(0)), Region(BrAnon(1))]), Binder(TraitPredicate(<impl for<'a> Fn(&Borrowed<'_>) -> bool as core::ops::function::FnMut<(&Borrowed<'_>,)>>, polarity:Positive), [Region(BrNamed(DefId(0:16 ~ example[a33f]::borrowed::{opaque#0}::'a), 'a)), Region(BrAnon(0)), Region(BrAnon(1))]), Binder(TraitPredicate(<impl for<'a> Fn(&Borrowed<'_>) -> bool as core::ops::function::FnOnce<(&Borrowed<'_>,)>>, polarity:Positive), [Region(BrNamed(DefId(0:16 ~ example[a33f]::borrowed::{opaque#0}::'a), 'a)), Region(BrAnon(0)), Region(BrAnon(1))]), Binder(TraitPredicate(<impl for<'a> Fn(&Borrowed<'_>) -> bool as core::marker::Sized>, polarity:Positive), [])], goal: impl for<'a> Fn(&Borrowed<'_>) -> bool: core::marker::Sized`
#1 [check_item_well_formed] checking that `borrowed` is well-formed
end of query stack
thread 'rustc' panicked at 'explicit panic', compiler/rustc_traits/src/chalk/lowering.rs:906:30

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-nightly (475b00aa4 2021-12-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z chalk -C debuginfo=1 -C linker=/opt/compiler-explorer/gcc-11.1.0/bin/gcc --crate-type bin

query stack during panic:
#0 [evaluate_goal] evaluating trait selection obligation `environment: [], goal: impl Fn(&Borrowed<'a>)-> bool well-formed`
#1 [check_item_well_formed] checking that `is_borrowed_cool` is well-formed
end of query stack
Compiler returned: 101

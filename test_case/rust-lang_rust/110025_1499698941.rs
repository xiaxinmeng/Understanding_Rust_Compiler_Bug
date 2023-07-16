plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0308]: mismatched types
   --> compiler/rustc_trait_selection/src/solve/trait_goals/structural_traits.rs:100:34
    |
100 |                 kind: ty::BrAnon(counter, None),
    |                       ---------- ^^^^^^^ expected `BoundVar`, found `u32`
    |                       arguments to this enum variant are incorrect
    |
note: tuple variant defined here
   --> /checkout/compiler/rustc_middle/src/ty/sty.rs:63:5
   --> /checkout/compiler/rustc_middle/src/ty/sty.rs:63:5
    |
63  |     BrAnon(BoundVar, Option<Span>),
    |     ^^^^^^
help: call `Into::into` on this expression to convert `u32` into `BoundVar`
    |
100 |                 kind: ty::BrAnon(counter.into(), None),

error[E0308]: mismatched types
   --> compiler/rustc_trait_selection/src/solve/trait_goals/structural_traits.rs:108:71
    |
    |
108 |         (0..counter).map(|i| ty::BoundVariableKind::Region(ty::BrAnon(i, None))),
    |                                                            ---------- ^ expected `BoundVar`, found `u32`
    |                                                            arguments to this enum variant are incorrect
    |
note: tuple variant defined here
   --> /checkout/compiler/rustc_middle/src/ty/sty.rs:63:5
   --> /checkout/compiler/rustc_middle/src/ty/sty.rs:63:5
    |
63  |     BrAnon(BoundVar, Option<Span>),
    |     ^^^^^^
help: call `Into::into` on this expression to convert `u32` into `BoundVar`
    |
108 |         (0..counter).map(|i| ty::BoundVariableKind::Region(ty::BrAnon(i.into(), None))),

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_trait_selection` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...

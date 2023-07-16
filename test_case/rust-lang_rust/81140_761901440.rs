plain
    Checking url v2.1.1
    Checking toml v0.5.7
    Checking cargo_metadata v0.12.0
    Checking clippy_lints v0.1.51 (/checkout/src/tools/clippy/clippy_lints)
error[E0769]: tuple variant `TerminatorKind::SwitchInt` written as struct variant
   --> src/tools/clippy/clippy_lints/src/utils/qualify_min_const_fn.rs:282:9
282 | /         TerminatorKind::SwitchInt {
283 | |             discr,
284 | |             switch_ty: _,
285 | |             targets: _,
285 | |             targets: _,
286 | |         } => check_operand(tcx, discr, span, body),
    | |_________^ help: use the tuple variant pattern syntax instead: `TerminatorKind::SwitchInt(_)`

error[E0769]: tuple variant `TerminatorKind::Call` written as struct variant
   --> src/tools/clippy/clippy_lints/src/utils/qualify_min_const_fn.rs:293:9
293 | /         TerminatorKind::Call {
294 | |             func,
295 | |             args,
295 | |             args,
296 | |             from_hir_call: _,
...   |
299 | |             fn_span: _,
300 | |         } => {
    | |_________^ help: use the tuple variant pattern syntax instead: `TerminatorKind::Call(_)`

error[E0769]: tuple variant `TerminatorKind::Assert` written as struct variant
   --> src/tools/clippy/clippy_lints/src/utils/qualify_min_const_fn.rs:337:9
337 | /         TerminatorKind::Assert {
338 | |             cond,
339 | |             expected: _,
340 | |             msg: _,
340 | |             msg: _,
341 | |             target: _,
342 | |             cleanup: _,
343 | |         } => check_operand(tcx, cond, span, body),
    | |_________^ help: use the tuple variant pattern syntax instead: `TerminatorKind::Assert(_)`

error[E0026]: variant `rustc_middle::mir::TerminatorKind::Call` does not have fields named `func`, `args`, `destination`
    |
    |
288 |         if let mir::TerminatorKind::Call { func, args, destination, .. } = kind;
    |                                            ^^^^  ^^^^  ^^^^^^^^^^^ variant `rustc_middle::mir::TerminatorKind::Call` does not have these fields

error[E0614]: type `rustc_middle::mir::Local` cannot be dereferenced
    |
    |
295 |             Some((def_id, *local, inner_ty, destination.as_ref().map(|(dest, _)| dest)?.as_local()?))


error[E0026]: variant `rustc_middle::mir::TerminatorKind::Call` does not have fields named `args`, `destination`
    |
542 |             args,
    |             ^^^^
    |             ^^^^
543 |             destination: Some((mir::Place { local: dest, .. }, _)),
    |             ^^^^^^^^^^^ variant `rustc_middle::mir::TerminatorKind::Call` does not have these fields
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0026, E0614, E0769.
For more information about an error, try `rustc --explain E0026`.

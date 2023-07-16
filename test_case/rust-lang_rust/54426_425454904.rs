plain
[00:17:58]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:18:04]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:19:22]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:19:31]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:19:54] error[E0277]: the trait bound `&[mir::ProjectionElem<'_, mir::Local, &ty::TyS<'_>>]: std::iter::Iterator` is not satisfied
[00:19:54]    --> librustc/mir/tcx.rs:287:28
[00:19:54]     |
[00:19:54] 287 |                 elems: tcx.mk_place_elems(&self.elems[..elem_index]),
[00:19:54]     |                            ^^^^^^^^^^^^^^ `&[mir::ProjectionElem<'_, mir::Local, &ty::TyS<'_>>]` is not an iterator; maybe try calling `.iter()` or a similar method
[00:19:54]     |
[00:19:54]     = help: the trait `std::iter::Iterator` is not implemented for `&[mir::ProjectionElem<'_, mir::Local, &ty::TyS<'_>>]`
[00:19:54]     = note: required because of the requirements on the impl of `ty::context::InternAs<[mir::ProjectionElem<'_, mir::Local, &ty::TyS<'_>>], &ty::List<mir::ProjectionElem<'_, mir::Local, &ty::TyS<'_>>>>` for `&[mir::ProjectionElem<'_, mir::Local, &ty::TyS<'_>>]`
travis_time:end:2641244a:start=1538143933558164958,finish=1538145135013876462,duration=1201455711504

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:135a95e7

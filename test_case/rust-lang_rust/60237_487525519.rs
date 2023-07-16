
error[E0277]: the trait bound `std::result::Result<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, ty::layout::LayoutError<'_>>: ty::layout::MaybeResult<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, !>` is not satisfied
    --> src/librustc/ty/layout.rs:1245:30
     |
1245 |                 match layout.field(self, i) {
     |                              ^^^^^ the trait `ty::layout::MaybeResult<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, !>` is not implemented for `std::result::Result<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, ty::layout::LayoutError<'_>>`
     |
     = help: the following implementations were found:
               <std::result::Result<T, E> as ty::layout::MaybeResult<T, E>>
     = note: required because of the requirements on the impl of `rustc_target::abi::TyLayoutMethods<'_, ty::layout::LayoutCx<'_, ty::context::TyCtxt<'_, 'tcx, 'tcx>>>` for `&ty::TyS<'_>`

error[E0277]: the trait bound `std::result::Result<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, ty::layout::LayoutError<'_>>: ty::layout::MaybeResult<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, !>` is not satisfied
    --> src/librustc/ty/layout.rs:1312:51
     |
1312 |                                            layout.for_variant(self, i))
     |                                                   ^^^^^^^^^^^ the trait `ty::layout::MaybeResult<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, !>` is not implemented for `std::result::Result<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, ty::layout::LayoutError<'_>>`
     |
     = help: the following implementations were found:
               <std::result::Result<T, E> as ty::layout::MaybeResult<T, E>>
     = note: required because of the requirements on the impl of `rustc_target::abi::TyLayoutMethods<'_, ty::layout::LayoutCx<'_, ty::context::TyCtxt<'_, 'tcx, 'tcx>>>` for `&ty::TyS<'_>`

error[E0277]: the trait bound `std::result::Result<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, ty::layout::LayoutError<'_>>: ty::layout::MaybeResult<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, !>` is not satisfied
    --> src/librustc/ty/layout.rs:1998:47
     |
1998 |                 return self.find_niche(layout.field(self, 0)?);
     |                                               ^^^^^ the trait `ty::layout::MaybeResult<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, !>` is not implemented for `std::result::Result<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, ty::layout::LayoutError<'_>>`
     |
     = help: the following implementations were found:
               <std::result::Result<T, E> as ty::layout::MaybeResult<T, E>>
     = note: required because of the requirements on the impl of `rustc_target::abi::TyLayoutMethods<'_, ty::layout::LayoutCx<'_, ty::context::TyCtxt<'_, 'tcx, 'tcx>>>` for `&ty::TyS<'_>`

error[E0277]: the trait bound `std::result::Result<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, ty::layout::LayoutError<'_>>: ty::layout::MaybeResult<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, !>` is not satisfied
    --> src/librustc/ty/layout.rs:2006:57
     |
2006 |             if let Some(mut c) = self.find_niche(layout.field(self, i)?)? {
     |                                                         ^^^^^ the trait `ty::layout::MaybeResult<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, !>` is not implemented for `std::result::Result<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, ty::layout::LayoutError<'_>>`
     |
     = help: the following implementations were found:
               <std::result::Result<T, E> as ty::layout::MaybeResult<T, E>>
     = note: required because of the requirements on the impl of `rustc_target::abi::TyLayoutMethods<'_, ty::layout::LayoutCx<'_, ty::context::TyCtxt<'_, 'tcx, 'tcx>>>` for `&ty::TyS<'_>`


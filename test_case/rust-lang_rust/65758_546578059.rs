
2019-10-25T20:33:57.8333743Z error[E0308]: mismatched types
2019-10-25T20:33:57.8335013Z --> src\tools\miri\src\helpers.rs:78:82
2019-10-25T20:33:57.8335166Z |
2019-10-25T20:33:57.8336103Z 78 | let place = mir::Place { base: mir::PlaceBase::Local(local), projection: Box::new([]) };
2019-10-25T20:33:57.8336416Z | ^^^^^^^^^^^^ expected reference, found struct `std::boxed::Box`
2019-10-25T20:33:57.8336985Z |
2019-10-25T20:33:57.8337152Z = note: expected type `&rustc::ty::List<rustc::mir::ProjectionElem<rustc::mir::Local, &rustc::ty::TyS<'_>>>`
2019-10-25T20:33:57.8337702Z found type `std::boxed::Box<[_; 0]>`

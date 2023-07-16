
src/librustc_metadata/encoder.rs:1416:13: 1416:44 error: unresolved name `encode_bounds_and_type_for_item` [E0425]

src/librustc_metadata/encoder.rs:1416             encode_bounds_and_type_for_item(rbml_w, self.ecx, self.index, ty.id);

                                                  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

src/librustc_metadata/encoder.rs:1416:13: 1416:44 help: run `rustc --explain E0425` to see a detailed explanation

src/librustc_metadata/encoder.rs:1407:9: 1407:28 error: the trait bound `index_builder::ItemContentBuilder<'a, 'tcx, 'encoder>: rustc::hir::intravisit::Visitor<'_>` is not satisfied [E0277]

src/librustc_metadata/encoder.rs:1407         intravisit::walk_ty(self, ty);

                                              ^~~~~~~~~~~~~~~~~~~

src/librustc_metadata/encoder.rs:1407:9: 1407:28 help: run `rustc --explain E0277` to see a detailed explanation

src/librustc_metadata/encoder.rs:1407:9: 1407:28 note: required by `rustc::hir::intravisit::walk_ty`

src/librustc_metadata/encoder.rs:1410:32: 1410:58 error: attempted access of field `rbml_w_for_visit_item` on type `&mut index_builder::ItemContentBuilder<'a, 'tcx, 'encoder>`, but no field with that name was found

src/librustc_metadata/encoder.rs:1410             let rbml_w = &mut *self.rbml_w_for_visit_item;

                                                                     ^~~~~~~~~~~~~~~~~~~~~~~~~~

src/librustc_metadata/encoder.rs:1412:25: 1412:35 error: attempted access of field `index` on type `&mut index_builder::ItemContentBuilder<'a, 'tcx, 'encoder>`, but no field with that name was found

src/librustc_metadata/encoder.rs:1412             let _task = self.index.record(def_id, rbml_w);

                                                              ^~~~~~~~~~

src/librustc_metadata/encoder.rs:1416:63: 1416:73 error: attempted access of field `index` on type `&mut index_builder::ItemContentBuilder<'a, 'tcx, 'encoder>`, but no field with that name was found

src/librustc_metadata/encoder.rs:1416             encode_bounds_and_type_for_item(rbml_w, self.ecx, self.index, ty.id);

                                                                                                    ^~~~~~~~~~

error: aborting due to 4 previous errors

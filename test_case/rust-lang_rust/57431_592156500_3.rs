
361843 [DEBUG rustc_mir::borrow_check::diagnostics::mutability_errors] report_mutability_error(access_place=(*_2), span=/home/nell/rust/src/test/ui/borrowck/issue-57431-coerced-mut-reference.rs:11:5:        11:13, the_place_err=PlaceRef { local: _2, projection: [Deref] }, error_access=MutableBorrow, location=bb0[12],)
361844 [DEBUG rustc_mir::borrow_check::diagnostics::mutability_errors] report_mutability_error: access_place_desc=Some("*ref_term")
361845 [DEBUG rustc_mir::borrow_check::diagnostics] borrowed_content_source: mpi=mp2
361846 [DEBUG rustc_mir::borrow_check::diagnostics] borrowed_content_source: init=mp2@Statement(bb0[6]) (Deep)
361847 [DEBUG rustc_mir::borrow_check::diagnostics] borrowed_content_source: loc=bb0[6] is_terminator=false
361848 [DEBUG rustc_mir::borrow_check::diagnostics::mutability_errors] report_mutability_error: item_msg="`*ref_term`", reason=", as it is behind a `&` reference"
361849 [DEBUG rustc_span::source_map] byte pos BytePos(218) is on the line at byte pos BytePos(214)
361850 [DEBUG rustc_span::source_map] char pos CharPos(218) is on the line at char pos CharPos(214)
361851 [DEBUG rustc_span::source_map] byte is on line: 11
361852 [DEBUG rustc_span::source_map] byte pos BytePos(226) is on the line at byte pos BytePos(214)
361853 [DEBUG rustc_span::source_map] char pos CharPos(226) is on the line at char pos CharPos(214)
361854 [DEBUG rustc_span::source_map] byte is on line: 11
361855 [DEBUG rustc_mir::borrow_check::diagnostics] borrow_spans: use_span=/home/nell/rust/src/test/ui/borrowck/issue-57431-coerced-mut-reference.rs:11:5: 11:13 location=bb0[12]
361856 [DEBUG rustc_errors::diagnostic_builder] Created new diagnostic
361857 [DEBUG rustc_mir::borrow_check::diagnostics::mutability_errors] report_mutability_error: act="borrow as mutable", acted_on="borrowed as mutable"

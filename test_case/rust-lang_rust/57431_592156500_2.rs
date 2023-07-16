
====================

Corresponds to

ref_term.mutate()

361821 [DEBUG rustc_mir::borrow_check] =========
361822 [DEBUG rustc_mir::borrow_check] place (*_2)
361823 [DEBUG rustc_span::source_map] byte pos BytePos(218) is on the line at byte pos BytePos(214)
361824 [DEBUG rustc_span::source_map] char pos CharPos(218) is on the line at char pos CharPos(214)
361825 [DEBUG rustc_span::source_map] byte is on line: 11
361826 [DEBUG rustc_span::source_map] byte pos BytePos(226) is on the line at byte pos BytePos(214)
361827 [DEBUG rustc_span::source_map] char pos CharPos(226) is on the line at char pos CharPos(214)
361828 [DEBUG rustc_span::source_map] byte is on line: 11
361829 [DEBUG rustc_mir::borrow_check] span /home/nell/rust/src/test/ui/borrowck/issue-57431-coerced-mut-reference.rs:11:5: 11:13
361830 [DEBUG rustc_mir::borrow_check] kind Reservation(MutableBorrow(Mut { allow_two_phase_borrow: true }))
361831 [DEBUG rustc_mir::borrow_check] is_local_mutation_allowedNo
361832 [DEBUG rustc_mir::borrow_check] flow_stateBorrowckAnalyses { borrows: [bw0], uninits: [mp0, mp3, mp4, mp5], ever_inits: [in0, in2] }
361833 [DEBUG rustc_mir::borrow_check] location bb0[12]
361834 [DEBUG rustc_mir::borrow_check] =========
361835 [DEBUG rustc_mir::borrow_check] =========

=======

Corresponds to

ref_term.mutate()

361903 [DEBUG rustc_mir::borrow_check] =========
361904 [DEBUG rustc_mir::borrow_check] place _5
361905 [DEBUG rustc_span::source_map] byte pos BytePos(218) is on the line at byte pos BytePos(214)
361906 [DEBUG rustc_span::source_map] char pos CharPos(218) is on the line at char pos CharPos(214)
361907 [DEBUG rustc_span::source_map] byte is on line: 11
361908 [DEBUG rustc_span::source_map] byte pos BytePos(226) is on the line at byte pos BytePos(214)
361909 [DEBUG rustc_span::source_map] char pos CharPos(226) is on the line at char pos CharPos(214)
361910 [DEBUG rustc_span::source_map] byte is on line: 11
361911 [DEBUG rustc_mir::borrow_check] span /home/nell/rust/src/test/ui/borrowck/issue-57431-coerced-mut-reference.rs:11:5: 11:13
361912 [DEBUG rustc_mir::borrow_check] kind Write(Mutate)
361913 [DEBUG rustc_mir::borrow_check] is_local_mutation_allowedNo
361914 [DEBUG rustc_mir::borrow_check] flow_stateBorrowckAnalyses { borrows: [bw0], uninits: [mp0, mp3, mp4, mp5], ever_inits: [in0, in2] }
361915 [DEBUG rustc_mir::borrow_check] location bb0[12]
361916 [DEBUG rustc_mir::borrow_check] =========
361917 [DEBUG rustc_mir::borrow_check] =========

========

Corresponds to

ref_term.mutate()

362059 [DEBUG rustc_mir::borrow_check] =========
362060 [DEBUG rustc_mir::borrow_check] place _5
362061 [DEBUG rustc_span::source_map] byte pos BytePos(218) is on the line at byte pos BytePos(214)
362062 [DEBUG rustc_span::source_map] char pos CharPos(218) is on the line at char pos CharPos(214)
362063 [DEBUG rustc_span::source_map] byte is on line: 11
362064 [DEBUG rustc_span::source_map] byte pos BytePos(235) is on the line at byte pos BytePos(214)
362065 [DEBUG rustc_span::source_map] char pos CharPos(235) is on the line at char pos CharPos(214)
362066 [DEBUG rustc_span::source_map] byte is on line: 11
362067 [DEBUG rustc_mir::borrow_check] span /home/nell/rust/src/test/ui/borrowck/issue-57431-coerced-mut-reference.rs:11:5: 11:22
362068 [DEBUG rustc_mir::borrow_check] kind Write(Move)
362069 [DEBUG rustc_mir::borrow_check] is_local_mutation_allowedYes
362070 [DEBUG rustc_mir::borrow_check] flow_stateBorrowckAnalyses { borrows: [bw0], uninits: [mp0, mp3, mp4], ever_inits: [in0, in2, in3] }
362071 [DEBUG rustc_mir::borrow_check] location bb0[13]
362072 [DEBUG rustc_mir::borrow_check] =========
362073 [DEBUG rustc_mir::borrow_check] =========
=================

Corresponds to

ref_term.mutate()

362236 [DEBUG rustc_mir::borrow_check] =========
362237 [DEBUG rustc_mir::borrow_check] place _5
362238 [DEBUG rustc_span::source_map] byte pos BytePos(234) is on the line at byte pos BytePos(214)
362239 [DEBUG rustc_span::source_map] char pos CharPos(234) is on the line at char pos CharPos(214)
362240 [DEBUG rustc_span::source_map] byte is on line: 11
362241 [DEBUG rustc_span::source_map] byte pos BytePos(235) is on the line at byte pos BytePos(214)
362242 [DEBUG rustc_span::source_map] char pos CharPos(235) is on the line at char pos CharPos(214)
362243 [DEBUG rustc_span::source_map] byte is on line: 11
362244 [DEBUG rustc_mir::borrow_check] span /home/nell/rust/src/test/ui/borrowck/issue-57431-coerced-mut-reference.rs:11:21: 11:22
362245 [DEBUG rustc_mir::borrow_check] kind Write(StorageDeadOrDrop)
362246 [DEBUG rustc_mir::borrow_check] is_local_mutation_allowedYes
362247 [DEBUG rustc_mir::borrow_check] flow_stateBorrowckAnalyses { borrows: [], uninits: [mp0, mp3, mp5], ever_inits: [in0, in2, in3, in4] }
362248 [DEBUG rustc_mir::borrow_check] location bb2[0]
362249 [DEBUG rustc_mir::borrow_check] =========
362250 [DEBUG rustc_mir::borrow_check] =========
========

Corresponds to ref_term.mutate()

362281 [DEBUG rustc_mir::borrow_check] =========
362282 [DEBUG rustc_mir::borrow_check] place _4
362283 [DEBUG rustc_span::source_map] byte pos BytePos(235) is on the line at byte pos BytePos(214)
362284 [DEBUG rustc_span::source_map] char pos CharPos(235) is on the line at char pos CharPos(214)
362285 [DEBUG rustc_span::source_map] byte is on line: 11
362286 [DEBUG rustc_span::source_map] byte pos BytePos(236) is on the line at byte pos BytePos(214)
362287 [DEBUG rustc_span::source_map] char pos CharPos(236) is on the line at char pos CharPos(214)
362288 [DEBUG rustc_span::source_map] byte is on line: 11
362289 [DEBUG rustc_mir::borrow_check] span /home/nell/rust/src/test/ui/borrowck/issue-57431-coerced-mut-reference.rs:11:22: 11:23
362290 [DEBUG rustc_mir::borrow_check] kind Write(StorageDeadOrDrop)
362291 [DEBUG rustc_mir::borrow_check] is_local_mutation_allowedYes
362292 [DEBUG rustc_mir::borrow_check] flow_stateBorrowckAnalyses { borrows: [], uninits: [mp0, mp3, mp5], ever_inits: [in0, in2, in4] }
362293 [DEBUG rustc_mir::borrow_check] location bb2[1]
362294 [DEBUG rustc_mir::borrow_check] =========
362295 [DEBUG rustc_mir::borrow_check] =========
=======


DEBUG:<unknown>: access_place: logging error place_span=`(_1, .\issue-48070.rs:15:5: 19:6)` kind=`(Deep, Activation(MutableBorrow(Mut { allow_two_phase_borrow: false }), bw0))`
DEBUG:<unknown>: check_access_permissions((*_4), Reservation(MutableBorrow(Mut { allow_two_phase_borrow: true })), No)
DEBUG:<unknown>: places_conflict(_1,(*_4),Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_4, (*_4)]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_4)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,(*_4),Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_4, (*_4)]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_4)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,(*_4),Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_4, (*_4)]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_4)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,(*_4),Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_4, (*_4)]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_4)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,(*_4),Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_4, (*_4)]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_4)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: check_if_path_is_moved part1 place: (*_4)
DEBUG:<unknown>: check_if_path_is_moved part2 place: (*_4)
DEBUG:<unknown>: check_access_permissions(_3, Write(Mutate), ExceptUpvars)
DEBUG:<unknown>: places_conflict(_1,_3,Shallow(None))
DEBUG:<unknown>: places_conflict: components [_1] / [_3]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_3)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,_3,Shallow(None))
DEBUG:<unknown>: places_conflict: components [_1] / [_3]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_3)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,_3,Shallow(None))
DEBUG:<unknown>: places_conflict: components [_1] / [_3]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_3)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,_3,Shallow(None))
DEBUG:<unknown>: places_conflict: components [_1] / [_3]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_3)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,_3,Shallow(None))
DEBUG:<unknown>: places_conflict: components [_1] / [_3]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_3)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: check_if_reassignment_to_immutable_state(_3)
DEBUG:<unknown>: MirBorrowckCtxt::process_terminator(bb9[1], Terminator { source_info: SourceInfo { span: .\issue-48070.rs:15:5: 19:13, scope: scope[1] }, kind: _2 = const Foo::emit(move _3) -> [return: bb12, unwind: bb1] }): borrows in effect: [&mut _1, &mut _1@active, &mut _1, &mut _1@active, &mut _1, &mut _1@active, &mut (*_4)] borrows generated: [&mut (*_4)] inits: [_1, _3, _4, _5] uninits: [_0, _2, _6, _7, _8, _9] move_out: [mp7@bb3[3], mp9@bb4[3]] ever_init: [mp1@.\issue-48070.rs:14:19: 14:31 (Deep), mp5@.\issue-48070.rs:15:11: 15:13 (Deep), mp4@.\issue-48070.rs:16:15: 16:23 (Deep), mp3@.\issue-48070.rs:15:5: 19:6 (Deep), mp4@.\issue-48070.rs:17:15: 17:28 (Deep), mp4@.\issue-48070.rs:18:14: 18:27 (Deep)]
DEBUG:<unknown>: check_access_permissions((*_4), Activation(MutableBorrow(Mut { allow_two_phase_borrow: true }), bw3), No)
DEBUG:<unknown>: places_conflict(_1,(*_4),Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_4, (*_4)]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_4)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,(*_4),Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_4, (*_4)]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_4)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,(*_4),Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_4, (*_4)]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_4)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,(*_4),Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_4, (*_4)]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_4)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,(*_4),Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_4, (*_4)]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_4)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,(*_4),Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_4, (*_4)]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_4)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict((*_4),(*_4),Deep)
DEBUG:<unknown>: places_conflict: components [_4, (*_4)] / [_4, (*_4)]
DEBUG:<unknown>: places_conflict: Some(_4) vs. Some(_4)
DEBUG:<unknown>: place_element_conflict: DISJOINT-OR-EQ-LOCAL
DEBUG:<unknown>: places_conflict: Some((*_4)) vs. Some((*_4))
DEBUG:<unknown>: place_element_conflict: DISJOINT-OR-EQ-DEREF
DEBUG:<unknown>: places_conflict: None vs. None
DEBUG:<unknown>: places_conflict: full borrow, CONFLICT
DEBUG:<unknown>: each_borrow_involving_path: ra6 @ BorrowData { location: bb9[0], kind: Mut { allow_two_phase_borrow: true }, region: '_#5r, borrowed_place: (*_4), assigned_place: _3 } vs. (*_4)/Deep
DEBUG:<unknown>: check_access_for_conflict place_span: ((*_4), .\issue-48070.rs:15:5: 19:13) sd: Deep rw: Activation(MutableBorrow(Mut { allow_two_phase_borrow: true }), bw3) skipping (ra6, BorrowData { location: bb9[0], kind: Mut { allow_two_phase_borrow: true }, region: '_#5r, borrowed_place: (*_4), assigned_place: _3 }) b/c activation of same borrow_index: bw3
DEBUG:<unknown>: check_access_permissions(_3, Write(Move), Yes)
DEBUG:<unknown>: places_conflict(_1,_3,Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_3]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_3)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,_3,Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_3]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_3)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,_3,Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_3]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_3)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,_3,Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_3]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_3)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,_3,Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_3]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_3)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,_3,Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_3]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_3)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict((*_4),_3,Deep)
DEBUG:<unknown>: places_conflict: components [_4, (*_4)] / [_3]
DEBUG:<unknown>: places_conflict: Some(_4) vs. Some(_3)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: check_if_path_is_moved part1 place: _3
DEBUG:<unknown>: check_if_path_is_moved part2 place: _3
DEBUG:<unknown>: check_access_permissions(_2, Write(Mutate), ExceptUpvars)
DEBUG:<unknown>: places_conflict(_1,_2,Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_2]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_2)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,_2,Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_2]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_2)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,_2,Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_2]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_2)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,_2,Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_2]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_2)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,_2,Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_2]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_2)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,_2,Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_2]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_2)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict((*_4),_2,Deep)
DEBUG:<unknown>: places_conflict: components [_4, (*_4)] / [_2]
DEBUG:<unknown>: places_conflict: Some(_4) vs. Some(_2)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: check_if_reassignment_to_immutable_state(_2)
DEBUG:<unknown>: MirBorrowckCtxt::process_statement(bb10[0], _4 = &mut (*_6)): borrows in effect: [&mut _1, &mut _1@active] borrows generated: [&mut (*_6)] inits: [_1, _5, _6] uninits: [_0, _2, _3, _4, _7, _8, _9] move_out: [mp7@bb3[3]] ever_init: [mp1@.\issue-48070.rs:14:19: 14:31 (Deep), mp5@.\issue-48070.rs:15:11: 15:13 (Deep), mp7@.\issue-48070.rs:17:15: 17:18 (Deep), mp6@.\issue-48070.rs:17:15: 17:28 (NonPanicPathOnly)]
DEBUG:<unknown>: check_access_permissions((*_6), Write(MutableBorrow(Mut { allow_two_phase_borrow: false })), No)
DEBUG:<unknown>: places_conflict(_1,(*_6),Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_6, (*_6)]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_6)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,(*_6),Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_6, (*_6)]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_6)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: check_if_path_is_moved part1 place: (*_6)
DEBUG:<unknown>: check_if_path_is_moved part2 place: (*_6)
DEBUG:<unknown>: check_access_permissions(_4, Write(Mutate), ExceptUpvars)
DEBUG:<unknown>: places_conflict(_1,_4,Shallow(None))
DEBUG:<unknown>: places_conflict: components [_1] / [_4]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_4)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,_4,Shallow(None))
DEBUG:<unknown>: places_conflict: components [_1] / [_4]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_4)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: check_if_reassignment_to_immutable_state(_4)
DEBUG:<unknown>: MirBorrowckCtxt::process_statement(bb10[1], StorageDead(_6)): borrows in effect: [&mut _1, &mut _1@active, &mut (*_6)] borrows generated: [] inits: [_1, _4, _5, _6] uninits: [_0, _2, _3, _7, _8, _9] move_out: [mp7@bb3[3]] ever_init: [mp1@.\issue-48070.rs:14:19: 14:31 (Deep), mp5@.\issue-48070.rs:15:11: 15:13 (Deep), mp7@.\issue-48070.rs:17:15: 17:18 (Deep), mp6@.\issue-48070.rs:17:15: 17:28 (NonPanicPathOnly), mp4@.\issue-48070.rs:17:15: 17:28 (Deep)]
DEBUG:<unknown>: check_access_permissions(_6, Write(StorageDeadOrDrop), Yes)
DEBUG:<unknown>: places_conflict(_1,_6,Shallow(None))
DEBUG:<unknown>: places_conflict: components [_1] / [_6]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_6)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,_6,Shallow(None))
DEBUG:<unknown>: places_conflict: components [_1] / [_6]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_6)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict((*_6),_6,Shallow(None))
DEBUG:<unknown>: places_conflict: components [_6, (*_6)] / [_6]
DEBUG:<unknown>: places_conflict: Some(_6) vs. Some(_6)
DEBUG:<unknown>: place_element_conflict: DISJOINT-OR-EQ-LOCAL
DEBUG:<unknown>: places_conflict: Some((*_6)) vs. None
DEBUG:<unknown>: places_conflict: shallow access behind ptr
DEBUG:<unknown>: MirBorrowckCtxt::process_statement(bb10[2], StorageDead(_7)): borrows in effect: [&mut _1, &mut _1@active] borrows generated: [] inits: [_1, _4, _5] uninits: [_0, _2, _3, _6, _7, _8, _9] move_out: [mp7@bb3[3]] ever_init: [mp1@.\issue-48070.rs:14:19: 14:31 (Deep), mp5@.\issue-48070.rs:15:11: 15:13 (Deep), mp7@.\issue-48070.rs:17:15: 17:18 (Deep), mp4@.\issue-48070.rs:17:15: 17:28 (Deep)]
DEBUG:<unknown>: check_access_permissions(_7, Write(StorageDeadOrDrop), Yes)
DEBUG:<unknown>: places_conflict(_1,_7,Shallow(None))
DEBUG:<unknown>: places_conflict: components [_1] / [_7]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_7)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,_7,Shallow(None))
DEBUG:<unknown>: places_conflict: components [_1] / [_7]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_7)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: MirBorrowckCtxt::process_terminator(bb10[3], Terminator { source_info: SourceInfo { span: .\issue-48070.rs:15:5: 19:6, scope: scope[1] }, kind: goto -> bb9 }): borrows in effect: [&mut _1, &mut _1@active] borrows generated: [] inits: [_1, _4, _5] uninits: [_0, _2, _3, _6, _7, _8, _9] move_out: [mp7@bb3[3]] ever_init: [mp1@.\issue-48070.rs:14:19: 14:31 (Deep), mp5@.\issue-48070.rs:15:11: 15:13 (Deep), mp4@.\issue-48070.rs:17:15: 17:28 (Deep)]
DEBUG:<unknown>: MirBorrowckCtxt::process_statement(bb11[0], _4 = &mut (*_8)): borrows in effect: [&mut _1, &mut _1@active] borrows generated: [&mut (*_8)] inits: [_1, _5, _8] uninits: [_0, _2, _3, _4, _6, _7, _9] move_out: [mp9@bb4[3]] ever_init: [mp1@.\issue-48070.rs:14:19: 14:31 (Deep), mp5@.\issue-48070.rs:15:11: 15:13 (Deep), mp9@.\issue-48070.rs:18:14: 18:17 (Deep), mp8@.\issue-48070.rs:18:14: 18:27 (NonPanicPathOnly)]
DEBUG:<unknown>: check_access_permissions((*_8), Write(MutableBorrow(Mut { allow_two_phase_borrow: false })), No)
DEBUG:<unknown>: places_conflict(_1,(*_8),Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_8, (*_8)]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_8)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,(*_8),Deep)
DEBUG:<unknown>: places_conflict: components [_1] / [_8, (*_8)]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_8)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: check_if_path_is_moved part1 place: (*_8)
DEBUG:<unknown>: check_if_path_is_moved part2 place: (*_8)
DEBUG:<unknown>: check_access_permissions(_4, Write(Mutate), ExceptUpvars)
DEBUG:<unknown>: places_conflict(_1,_4,Shallow(None))
DEBUG:<unknown>: places_conflict: components [_1] / [_4]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_4)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,_4,Shallow(None))
DEBUG:<unknown>: places_conflict: components [_1] / [_4]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_4)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: check_if_reassignment_to_immutable_state(_4)
DEBUG:<unknown>: MirBorrowckCtxt::process_statement(bb11[1], StorageDead(_8)): borrows in effect: [&mut _1, &mut _1@active, &mut (*_8)] borrows generated: [] inits: [_1, _4, _5, _8] uninits: [_0, _2, _3, _6, _7, _9] move_out: [mp9@bb4[3]] ever_init: [mp1@.\issue-48070.rs:14:19: 14:31 (Deep), mp5@.\issue-48070.rs:15:11: 15:13 (Deep), mp9@.\issue-48070.rs:18:14: 18:17 (Deep), mp8@.\issue-48070.rs:18:14: 18:27 (NonPanicPathOnly), mp4@.\issue-48070.rs:18:14: 18:27 (Deep)]
DEBUG:<unknown>: check_access_permissions(_8, Write(StorageDeadOrDrop), Yes)
DEBUG:<unknown>: places_conflict(_1,_8,Shallow(None))
DEBUG:<unknown>: places_conflict: components [_1] / [_8]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_8)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,_8,Shallow(None))
DEBUG:<unknown>: places_conflict: components [_1] / [_8]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_8)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict((*_8),_8,Shallow(None))
DEBUG:<unknown>: places_conflict: components [_8, (*_8)] / [_8]
DEBUG:<unknown>: places_conflict: Some(_8) vs. Some(_8)
DEBUG:<unknown>: place_element_conflict: DISJOINT-OR-EQ-LOCAL
DEBUG:<unknown>: places_conflict: Some((*_8)) vs. None
DEBUG:<unknown>: places_conflict: shallow access behind ptr
DEBUG:<unknown>: MirBorrowckCtxt::process_statement(bb11[2], StorageDead(_9)): borrows in effect: [&mut _1, &mut _1@active] borrows generated: [] inits: [_1, _4, _5] uninits: [_0, _2, _3, _6, _7, _8, _9] move_out: [mp9@bb4[3]] ever_init: [mp1@.\issue-48070.rs:14:19: 14:31 (Deep), mp5@.\issue-48070.rs:15:11: 15:13 (Deep), mp9@.\issue-48070.rs:18:14: 18:17 (Deep), mp4@.\issue-48070.rs:18:14: 18:27 (Deep)]
DEBUG:<unknown>: check_access_permissions(_9, Write(StorageDeadOrDrop), Yes)
DEBUG:<unknown>: places_conflict(_1,_9,Shallow(None))
DEBUG:<unknown>: places_conflict: components [_1] / [_9]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_9)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: places_conflict(_1,_9,Shallow(None))
DEBUG:<unknown>: places_conflict: components [_1] / [_9]
DEBUG:<unknown>: places_conflict: Some(_1) vs. Some(_9)
DEBUG:<unknown>: place_element_conflict: DISJOINT-LOCAL
DEBUG:<unknown>: places_conflict: disjoint
DEBUG:<unknown>: MirBorrowckCtxt::process_terminator(bb11[3], Terminator { source_info: SourceInfo { span: .\issue-48070.rs:15:5: 19:6, scope: scope[1] }, kind: goto -> bb9 }): borrows in effect: [&mut _1, &mut _1@active] borrows generated: [] inits: [_1, _4, _5] uninits: [_0, _2, _3, _6, _7, _8, _9] move_out: [mp9@bb4[3]] ever_init: [mp1@.\issue-48070.rs:14:19: 14:31 (Deep), mp5@.\issue-48070.rs:15:11: 15:13 (Deep), mp4@.\issue-48070.rs:18:14: 18:27 (Deep)]
DEBUG:<unknown>: MirBorrowckCtxt::process_statement(bb12[0], nop): borrows in effect: [] borrows generated: [] inits: [_1, _2, _4, _5] uninits: [_0, _3, _6, _7, _8, _9] move_out: [mp7@bb3[3], mp9@bb4[3], mp3@bb9[1]] ever_init: [mp1@.\issue-48070.rs:14:19: 14:31 (Deep), mp5@.\issue-48070.rs:15:11: 15:13 (Deep), mp4@.\issue-48070.rs:16:15: 16:23 (Deep), mp3@.\issue-48070.rs:15:5: 19:6 (Deep), mp2@.\issue-48070.rs:15:5: 19:13 (NonPanicPathOnly), mp4@.\issue-48070.rs:17:15: 17:28 (Deep), mp4@.\issue-48070.rs:18:14: 18:27 (Deep)]
DEBUG:<unknown>: MirBorrowckCtxt::process_statement(bb12[1], StorageDead(_3)): borrows in effect: [] borrows generated: [] inits: [_1, _2, _4, _5] uninits: [_0, _3, _6, _7, _8, _9] move_out: [mp7@bb3[3], mp9@bb4[3], mp3@bb9[1]] ever_init: [mp1@.\issue-48070.rs:14:19: 14:31 (Deep), mp5@.\issue-48070.rs:15:11: 15:13 (Deep), mp4@.\issue-48070.rs:16:15: 16:23 (Deep), mp3@.\issue-48070.rs:15:5: 19:6 (Deep), mp2@.\issue-48070.rs:15:5: 19:13 (NonPanicPathOnly), mp4@.\issue-48070.rs:17:15: 17:28 (Deep), mp4@.\issue-48070.rs:18:14: 18:27 (Deep)]
DEBUG:<unknown>: check_access_permissions(_3, Write(StorageDeadOrDrop), Yes)
DEBUG:<unknown>: MirBorrowckCtxt::process_statement(bb12[2], StorageDead(_4)): borrows in effect: [] borrows generated: [] inits: [_1, _2, _4, _5] uninits: [_0, _3, _6, _7, _8, _9] move_out: [mp7@bb3[3], mp9@bb4[3], mp3@bb9[1]] ever_init: [mp1@.\issue-48070.rs:14:19: 14:31 (Deep), mp5@.\issue-48070.rs:15:11: 15:13 (Deep), mp4@.\issue-48070.rs:16:15: 16:23 (Deep), mp2@.\issue-48070.rs:15:5: 19:13 (NonPanicPathOnly), mp4@.\issue-48070.rs:17:15: 17:28 (Deep), mp4@.\issue-48070.rs:18:14: 18:27 (Deep)]
DEBUG:<unknown>: check_access_permissions(_4, Write(StorageDeadOrDrop), Yes)
DEBUG:<unknown>: MirBorrowckCtxt::process_statement(bb12[3], StorageDead(_5)): borrows in effect: [] borrows generated: [] inits: [_1, _2, _5] uninits: [_0, _3, _4, _6, _7, _8, _9] move_out: [mp7@bb3[3], mp9@bb4[3], mp3@bb9[1]] ever_init: [mp1@.\issue-48070.rs:14:19: 14:31 (Deep), mp5@.\issue-48070.rs:15:11: 15:13 (Deep), mp2@.\issue-48070.rs:15:5: 19:13 (NonPanicPathOnly)]
DEBUG:<unknown>: check_access_permissions(_5, Write(StorageDeadOrDrop), Yes)
DEBUG:<unknown>: MirBorrowckCtxt::process_statement(bb12[4], _0 = ()): borrows in effect: [] borrows generated: [] inits: [_1, _2] uninits: [_0, _3, _4, _5, _6, _7, _8, _9] move_out: [mp7@bb3[3], mp9@bb4[3], mp3@bb9[1]] ever_init: [mp1@.\issue-48070.rs:14:19: 14:31 (Deep), mp2@.\issue-48070.rs:15:5: 19:13 (NonPanicPathOnly)]
DEBUG:<unknown>: check_access_permissions(_0, Write(Mutate), ExceptUpvars)
DEBUG:<unknown>: check_if_reassignment_to_immutable_state(_0)
DEBUG:<unknown>: MirBorrowckCtxt::process_statement(bb12[5], StorageDead(_1)): borrows in effect: [] borrows generated: [] inits: [_0, _1, _2] uninits: [_3, _4, _5, _6, _7, _8, _9] move_out: [mp7@bb3[3], mp9@bb4[3], mp3@bb9[1]] ever_init: [mp1@.\issue-48070.rs:14:19: 14:31 (Deep), mp2@.\issue-48070.rs:15:5: 19:13 (NonPanicPathOnly), mp0@.\issue-48070.rs:13:11: 20:2 (Deep)]
DEBUG:<unknown>: check_access_permissions(_1, Write(StorageDeadOrDrop), Yes)
DEBUG:<unknown>: MirBorrowckCtxt::process_terminator(bb12[6], Terminator { source_info: SourceInfo { span: .\issue-48070.rs:20:2: 20:2, scope: scope[0] }, kind: return }): borrows in effect: [] borrows generated: [] inits: [_0, _2] uninits: [_1, _3, _4, _5, _6, _7, _8, _9] move_out: [mp7@bb3[3], mp9@bb4[3], mp3@bb9[1]] ever_init: [mp2@.\issue-48070.rs:15:5: 19:13 (NonPanicPathOnly), mp0@.\issue-48070.rs:13:11: 20:2 (Deep)]
DEBUG:<unknown>: mir_borrowck done

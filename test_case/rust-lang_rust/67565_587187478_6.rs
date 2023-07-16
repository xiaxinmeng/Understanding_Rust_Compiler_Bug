
[DEBUG rustc_mir::borrow_check] MirBorrowckCtxt::process_statement(bb0[0], StorageLive(_3)): BorrowckAnalyses { borrows: [], uninits: [mp0, mp1, mp2, mp3, mp4], ever_inits: [] }
[DEBUG rustc_mir::borrow_check] MirBorrowckCtxt::process_statement(bb0[1], StorageLive(_4)): BorrowckAnalyses { borrows: [], uninits: [mp0, mp1, mp2, mp3, mp4], ever_inits: [] }
[DEBUG rustc_mir::borrow_check] MirBorrowckCtxt::process_statement(bb0[2], _4 = const 1i32): BorrowckAnalyses { borrows: [], uninits: [mp0, mp1, mp2, mp3, mp4], ever_inits: [] }
[DEBUG rustc_mir::borrow_check] check_if_assigned_path_is_moved place: _4                                                       
[DEBUG rustc_mir::borrow_check] check_access_permissions(_4, Write(Mutate), is_local_mutation_allowed: No)
[DEBUG rustc_mir::borrow_check] check_access_for_conflict(location=bb0[2], place_span=(_4, foo.rs:6:36: 6:37), sd=Shallow(None), rw=Write(Mutate))
[DEBUG rustc_mir::borrow_check] MirBorrowckCtxt::process_statement(bb0[3], _3 = &mut _4): BorrowckAnalyses { borrows: [], uninits: [mp0, mp1, mp2, mp3], ever_inits: [in0] }
[DEBUG rustc_mir::borrow_check] check_access_permissions(_4, Write(MutableBorrow(Mut { allow_two_phase_borrow: false })), is_local_mutation_allowed: No)
[DEBUG rustc_mir::borrow_check] check_access_for_conflict(location=bb0[3], place_span=(_4, foo.rs:6:31: 6:37), sd=Deep, rw=Write(MutableBorrow(Mut { allow_two_phase_borrow: false })))
[DEBUG rustc_mir::borrow_check] check_if_full_path_is_moved place: PlaceRef { local: _4, projection: [] }
[DEBUG rustc_mir::borrow_check] check_if_path_or_subpath_is_moved place: PlaceRef { local: _4, projection: [] }
[DEBUG rustc_mir::borrow_check] check_if_assigned_path_is_moved place: _3
[DEBUG rustc_mir::borrow_check] check_access_permissions(_3, Write(Mutate), is_local_mutation_allowed: No)
[DEBUG rustc_mir::borrow_check] check_access_for_conflict(location=bb0[3], place_span=(_3, foo.rs:6:31: 6:37), sd=Shallow(None), rw=Write(Mutate))
[DEBUG rustc_mir::borrow_check] MirBorrowckCtxt::process_statement(bb0[4], StorageLive(_1)): BorrowckAnalyses { borrows: [bw0], uninits: [mp0, mp1, mp2], ever_inits: [in0, in1] }
[DEBUG rustc_mir::borrow_check] MirBorrowckCtxt::process_statement(bb0[5], _1 = &mut _3): BorrowckAnalyses { borrows: [bw0], uninits: [mp0, mp1, mp2], ever_inits: [in0, in1] }
[DEBUG rustc_mir::borrow_check] check_access_permissions(_3, Write(MutableBorrow(Mut { allow_two_phase_borrow: false })), is_local_mutation_allowed: No)
[DEBUG rustc_mir::borrow_check] check_access_for_conflict(location=bb0[5], place_span=(_3, foo.rs:6:9: 6:28), sd=Deep, rw=Write(MutableBorrow(Mut { allow_two_phase_borrow: false })))
[DEBUG rustc_mir::borrow_check::places_conflict] borrow_conflicts_with_place(_4, PlaceRef { local: _3, projection: [] }, Deep, Overlap)
[DEBUG rustc_mir::borrow_check] check_if_full_path_is_moved place: PlaceRef { local: _3, projection: [] }
[DEBUG rustc_mir::borrow_check] check_if_path_or_subpath_is_moved place: PlaceRef { local: _3, projection: [] }
[DEBUG rustc_mir::borrow_check] check_if_assigned_path_is_moved place: _1
[DEBUG rustc_mir::borrow_check] check_if_reassignment_to_immutable_state(_1)
[DEBUG rustc_mir::borrow_check] MirBorrowckCtxt::process_statement(bb0[6], StorageLive(_2)): BorrowckAnalyses { borrows: [bw0, bw1], uninits: [mp0, mp2], ever_inits: [in0, in1, in2] }
[DEBUG rustc_mir::borrow_check] MirBorrowckCtxt::process_statement(bb0[7], _2 = &_3): BorrowckAnalyses { borrows: [bw0, bw1], uninits: [mp0, mp2], ever_inits: [in0, in1, in2] }
[DEBUG rustc_mir::borrow_check] check_access_permissions(_3, Read(Borrow(Shared)), is_local_mutation_allowed: No)
[DEBUG rustc_mir::borrow_check] check_access_for_conflict(location=bb0[7], place_span=(_3, foo.rs:6:22: 6:28), sd=Deep, rw=Read(Borrow(Shared)))
[DEBUG rustc_mir::borrow_check::places_conflict] borrow_conflicts_with_place(_4, PlaceRef { local: _3, projection: [] }, Deep, Overlap)
[DEBUG rustc_mir::borrow_check::places_conflict] borrow_conflicts_with_place(_3, PlaceRef { local: _3, projection: [] }, Deep, Overlap)
[DEBUG rustc_mir::borrow_check::path_utils] each_borrow_involving_path: bw1 @ BorrowData { reserve_location: bb0[5], activation_location: NotTwoPhase, kind: Mut { allow_two_phase_borrow: false }, region: '_#3r, borrowed_place: _3, 
assigned_place: _1 } vs. _3/Deep
[DEBUG rustc_mir::borrow_check::path_utils] is_active(borrow_data=BorrowData { reserve_location: bb0[5], activation_location: NotTwoPhase, kind: Mut { allow_two_phase_borrow: false }, region: '_#3r, borrowed_place: _3, assigned_place: _1 }, location=bb0[7])
[DEBUG rustc_mir::borrow_check::diagnostics] borrow_spans: use_span=foo.rs:6:9: 6:28 location=bb0[5]
[DEBUG rustc_mir::borrow_check::diagnostics] borrow_spans: use_span=foo.rs:6:22: 6:28 location=bb0[7]
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] explain_why_borrow_contains_point(location=bb0[7], borrow=BorrowData { reserve_location: bb0[5], activation_location: NotTwoPhase, kind: Mut { allow_two_phase_borrow: false }, region: '_#3r, borrowed_place: _3, assigned_place: _1 }, kind_place=None)
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] explain_why_borrow_contains_point: borrow_region_vid='_#3r
[DEBUG rustc_mir::borrow_check::region_infer] find_sub_region_live_at(fr1='_#3r, elem=bb0[7])
[DEBUG rustc_mir::borrow_check::region_infer] find_constraint_paths_between_regions: from_region='_#3r r='_#3r value={bb0[5..=8]}
[DEBUG rustc_mir::borrow_check::region_infer] find_sub_region_live_at: liveness_constraints for '_#3r are "{bb0[5]}"
[DEBUG rustc_mir::borrow_check::region_infer] find_constraint_paths_between_regions: from_region='_#3r r='_#5r value={bb0[6..=8]}
[DEBUG rustc_mir::borrow_check::region_infer] find_sub_region_live_at: liveness_constraints for '_#5r are "{bb0[6..=8]}"
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] explain_why_borrow_contains_point: region_sub='_#5r
[DEBUG rustc_mir::borrow_check::diagnostics] move_spans: moved_place=PlaceRef { local: _1, projection: [] } location=bb0[8] stmt=(*(*_1)) = const 0i32
[DEBUG rustc_mir::borrow_check::diagnostics] borrow_spans: use_span=foo.rs:7:5: 7:13 location=bb0[8]
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] reach_through_backedge: from=bb0[8] to=bb0[8]
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] reach_through_backedge: location=bb0[8] outmost_back_edge=None
                       pending_locations=[] visited_locations={bb0[8]}
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] reach_through_backedge: location=bb0[9] outmost_back_edge=None
                       pending_locations=[] visited_locations={bb0[8], bb0[9]}
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] reach_through_backedge: location=bb0[10] outmost_back_edge=None
                       pending_locations=[] visited_locations={bb0[8], bb0[9], bb0[10]}
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] reach_through_backedge: location=bb0[11] outmost_back_edge=None
                       pending_locations=[] visited_locations={bb0[8], bb0[10], bb0[9], bb0[11]}
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] reach_through_backedge: location=bb0[12] outmost_back_edge=None
                       pending_locations=[] visited_locations={bb0[8], bb0[10], bb0[12], bb0[9], bb0[11]}
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] reach_through_backedge: location=bb0[13] outmost_back_edge=None
                       pending_locations=[] visited_locations={bb0[8], bb0[13], bb0[10], bb0[12], bb0[9], bb0[11]}
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] reach_through_backedge: location=bb0[14] outmost_back_edge=None
                       pending_locations=[] visited_locations={bb0[8], bb0[13], bb0[10], bb0[12], bb0[9], bb0[14], bb0[11]}
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait_object: location=bb0[5] stmt=Some(_1 = &mut _3)
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait: target=_1 queue=[bb0[5]]
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait: target=_1
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait_object: stmt=_1 = &mut _3
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait: queue=[bb0[6]]
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait: target=_1
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait_object: stmt=StorageLive(_2)
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait: queue=[bb0[7]]
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait: target=_1
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait_object: stmt=_2 = &_3
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait: queue=[bb0[8]]
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait: target=_1
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait_object: stmt=(*(*_1)) = const 0i32
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait: target=_1
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait_object: stmt=_0 = ()
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait: queue=[bb0[10]]
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait: target=_1
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait_object: stmt=StorageDead(_2)
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait: queue=[bb0[11]]
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait: target=_1
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait_object: stmt=StorageDead(_1)
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait: queue=[bb0[12]]
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait: target=_1
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait_object: stmt=StorageDead(_4)
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait: queue=[bb0[13]]
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait: target=_1
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait_object: stmt=StorageDead(_3)
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait: queue=[bb0[14]]
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait: target=_1
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait_object: terminator=Terminator { source_info: SourceInfo { span: foo.rs:8:2: 8:2, scope: scope[0] }, kind: return }
[DEBUG rustc_mir::borrow_check::diagnostics::explain_borrow] was_captured_by_trait: queue=[]
[DEBUG rustc_mir::borrow_check] access_place: logging error place_span=`(_3, foo.rs:6:22: 6:28)` kind=`(Deep, Read(Borrow(Shared)))`
[DEBUG rustc_mir::borrow_check] check_if_full_path_is_moved place: PlaceRef { local: _3, projection: [] }
[DEBUG rustc_mir::borrow_check] check_if_path_or_subpath_is_moved place: PlaceRef { local: _3, projection: [] }
[DEBUG rustc_mir::borrow_check] check_if_assigned_path_is_moved place: _2
[DEBUG rustc_mir::borrow_check] check_if_reassignment_to_immutable_state(_2)
[DEBUG rustc_mir::borrow_check] MirBorrowckCtxt::process_statement(bb0[8], (*(*_1)) = const 0i32): BorrowckAnalyses { borrows: [bw0, bw1], uninits: [mp0], ever_inits: [in0, in1, in2, in3] }
[DEBUG rustc_mir::borrow_check] check_if_assigned_path_is_moved place: (*(*_1))
[DEBUG rustc_mir::borrow_check] check_if_full_path_is_moved place: PlaceRef { local: _1, projection: [Deref] }
[DEBUG rustc_mir::borrow_check] check_access_permissions((*(*_1)), Write(Mutate), is_local_mutation_allowed: No)
[DEBUG rustc_mir::borrow_check] check_access_for_conflict(location=bb0[8], place_span=((*(*_1)), foo.rs:7:5: 7:13), sd=Shallow(None), rw=Write(Mutate))
[DEBUG rustc_mir::borrow_check::places_conflict] borrow_conflicts_with_place(_4, PlaceRef { local: _1, projection: [Deref, Deref] }, Shallow(None), Overlap)
[DEBUG rustc_mir::borrow_check::places_conflict] place_element_conflict: DISJOINT-LOCAL
[DEBUG rustc_mir::borrow_check::places_conflict] borrow_conflicts_with_place: disjoint
[DEBUG rustc_mir::borrow_check::places_conflict] borrow_conflicts_with_place(_3, PlaceRef { local: _1, projection: [Deref, Deref] }, Shallow(None), Overlap)
[DEBUG rustc_mir::borrow_check::places_conflict] place_element_conflict: DISJOINT-LOCAL
[DEBUG rustc_mir::borrow_check::places_conflict] borrow_conflicts_with_place: disjoint
[DEBUG rustc_mir::borrow_check] MirBorrowckCtxt::process_statement(bb0[9], _0 = ()): BorrowckAnalyses { borrows: [], uninits: [mp0], ever_inits: [in0, in1, in2, in3] }
[DEBUG rustc_mir::borrow_check] check_if_assigned_path_is_moved place: _0
[DEBUG rustc_mir::borrow_check] check_access_permissions(_0, Write(Mutate), is_local_mutation_allowed: No)
[DEBUG rustc_mir::borrow_check] check_access_for_conflict(location=bb0[9], place_span=(_0, foo.rs:5:12: 8:2), sd=Shallow(None), rw=Write(Mutate))
[DEBUG rustc_mir::borrow_check] MirBorrowckCtxt::process_statement(bb0[10], StorageDead(_2)): BorrowckAnalyses { borrows: [], uninits: [], ever_inits: [in0, in1, in2, in3, in4] }
[DEBUG rustc_mir::borrow_check] check_access_permissions(_2, Write(StorageDeadOrDrop), is_local_mutation_allowed: Yes)
[DEBUG rustc_mir::borrow_check] check_access_for_conflict(location=bb0[10], place_span=(_2, foo.rs:8:1: 8:2), sd=Shallow(None), rw=Write(StorageDeadOrDrop))
[DEBUG rustc_mir::borrow_check] MirBorrowckCtxt::process_statement(bb0[11], StorageDead(_1)): BorrowckAnalyses { borrows: [], uninits: [mp2], ever_inits: [in0, in1, in2, in4] }
[DEBUG rustc_mir::borrow_check] check_access_permissions(_1, Write(StorageDeadOrDrop), is_local_mutation_allowed: Yes)
[DEBUG rustc_mir::borrow_check] check_access_for_conflict(location=bb0[11], place_span=(_1, foo.rs:8:1: 8:2), sd=Shallow(None), rw=Write(StorageDeadOrDrop))
[DEBUG rustc_mir::borrow_check] MirBorrowckCtxt::process_statement(bb0[12], StorageDead(_4)): BorrowckAnalyses { borrows: [], uninits: [mp1, mp2], ever_inits: [in0, in1, in4] }
[DEBUG rustc_mir::borrow_check] check_access_permissions(_4, Write(StorageDeadOrDrop), is_local_mutation_allowed: Yes)
[DEBUG rustc_mir::borrow_check] check_access_for_conflict(location=bb0[12], place_span=(_4, foo.rs:8:1: 8:2), sd=Shallow(None), rw=Write(StorageDeadOrDrop))
[DEBUG rustc_mir::borrow_check] MirBorrowckCtxt::process_statement(bb0[13], StorageDead(_3)): BorrowckAnalyses { borrows: [], uninits: [mp1, mp2, mp4], ever_inits: [in1, in4] }
[DEBUG rustc_mir::borrow_check] check_access_permissions(_3, Write(StorageDeadOrDrop), is_local_mutation_allowed: Yes)
[DEBUG rustc_mir::borrow_check] check_access_for_conflict(location=bb0[13], place_span=(_3, foo.rs:8:1: 8:2), sd=Shallow(None), rw=Write(StorageDeadOrDrop))
[DEBUG rustc_mir::borrow_check] MirBorrowckCtxt::process_terminator(bb0[14], Terminator { source_info: SourceInfo { span: foo.rs:8:2: 8:2, scope: scope[0] }, kind: return }): BorrowckAnalyses { borrows: [], uninits: [mp1, mp2, mp3, mp4], ever_inits: [in4] }
[DEBUG rustc_mir::borrow_check::used_muts] visit_statement: statement=_4 = const 1i32 local=_4 never_initialized_mut_locals={}
[DEBUG rustc_mir::borrow_check::used_muts] visit_statement: statement=_3 = &mut _4 local=_3 never_initialized_mut_locals={}
[DEBUG rustc_mir::borrow_check::used_muts] visit_statement: statement=_1 = &mut _3 local=_1 never_initialized_mut_locals={}
[DEBUG rustc_mir::borrow_check::used_muts] visit_statement: statement=_2 = &_3 local=_2 never_initialized_mut_locals={}
[DEBUG rustc_mir::borrow_check::used_muts] visit_statement: statement=(*(*_1)) = const 0i32 local=_1 never_initialized_mut_locals={}
[DEBUG rustc_mir::borrow_check::used_muts] visit_statement: statement=_0 = () local=_0 never_initialized_mut_locals={}
[DEBUG rustc_mir::borrow_check::used_muts] visit_terminator_kind: kind=return
[DEBUG rustc_mir::borrow_check::used_muts] gather_used_muts: never_initialized_mut_locals={}
[DEBUG rustc_mir::borrow_check] mbcx.used_mut: {_4, _3}

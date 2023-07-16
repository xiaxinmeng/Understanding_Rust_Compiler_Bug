
src/librustc_data_structures/graph_algorithms/dominators/test.rs:25:17: 25:58 error: attempted access of field `vec` on type `&indexed_vec::IndexVec<usize, core::option::Option<usize>>`, but no field with that name was found

src/librustc_data_structures/graph_algorithms/dominators/test.rs:25     assert_eq!(&dominators.all_immediate_dominators().vec[..],

                                                                                    ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

src/librustc_data_structures/graph_algorithms/dominators/test.rs:25:5: 29:28 note: in this expansion of assert_eq! (defined in <std macros>)

src/librustc_data_structures/graph_algorithms/dominators/test.rs:48:17: 48:58 error: attempted access of field `vec` on type `&indexed_vec::IndexVec<usize, core::option::Option<usize>>`, but no field with that name was found

src/librustc_data_structures/graph_algorithms/dominators/test.rs:48     assert_eq!(&dominators.all_immediate_dominators().vec[..],

                                                                                    ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

src/librustc_data_structures/graph_algorithms/dominators/test.rs:48:5: 51:46 note: in this expansion of assert_eq! (defined in <std macros>)

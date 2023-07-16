
Breakpoint 1, rust_panic () at libstd/panicking.rs:525
525             __rust_start_panic(obj as usize)
Missing separate debuginfos, use: dnf debuginfo-install libstdc++-8.1.1-1.fc28.x86_64
(gdb) up
#1  0x00007ffff763037c in std::panicking::update_count_then_panic () at libstd/panicking.rs:516
516         rust_panic(&mut RewrapBox(msg))
(gdb)
#2  0x00007ffff7630996 in std::panic::resume_unwind () at libstd/panic.rs:425
425         panicking::update_count_then_panic(payload)
(gdb)
#3  0x00007ffff0bf1032 in rustc_errors::FatalError::raise () at librustc_errors/lib.rs:238
238             panic::resume_unwind(Box::new(FatalErrorMarker))
(gdb)
#4  0x00007ffff5b8ad64 in <(dyn rustc_typeck::astconv::AstConv<'gcx, 'tcx> + 'o)>::trait_def_id () at librustc_typeck/astconv.rs:697
697                     FatalError.raise();
(gdb)
#5  <(dyn rustc_typeck::astconv::AstConv<'gcx, 'tcx> + 'o)>::instantiate_poly_trait_ref_inner () at librustc_typeck/astconv.rs:711
711             let trait_def_id = self.trait_def_id(trait_ref);
(gdb)
#6  0x00007ffff5b8adf8 in <(dyn rustc_typeck::astconv::AstConv<'gcx, 'tcx> + 'o)>::instantiate_poly_trait_ref ()
    at librustc_typeck/astconv.rs:744
744             self.instantiate_poly_trait_ref_inner(&poly_trait_ref.trait_ref, self_ty,
(gdb)
#7  0x00007ffff5bc4918 in rustc_typeck::collect::explicit_predicates_of () at librustc_typeck/collect.rs:1808
1808                                let trait_ref = AstConv::instantiate_poly_trait_ref(
(gdb)
#8  0x00007ffff2b1ef18 in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::explicit_predicates_of<'tcx>>::compute::{{closure}} () at librustc/ty/query/plumbing.rs:822
822                         provider(tcx.global_tcx(), key)
(gdb)
#9  rustc::ty::query::__query_compute::explicit_predicates_of () at librustc/ty/query/plumbing.rs:790
790                     f()
(gdb)

#10 0x00007ffff2cc91a5 in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::explicit_predicates_of<'tcx>>::compute () at librustc/ty/query/plumbing.rs:820
820                     __query_compute::$name(move || {
(gdb)
#11 0x00007ffff308af86 in rustc::dep_graph::graph::DepGraph::with_task_impl () at librustc/dep_graph/graph.rs:340
340                     (task(cx, arg), DepNodeIndex::INVALID)
(gdb) #12 0x00007ffff3046f1c in rustc::dep_graph::graph::DepGraph::with_task () at librustc/dep_graph/graph.rs:206
206             self.with_task_impl(key, cx, arg, false, task,

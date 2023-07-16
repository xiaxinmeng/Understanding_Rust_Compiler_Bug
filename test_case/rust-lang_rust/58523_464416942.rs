
#0  core::iter::range::<impl core::iter::traits::iterator::Iterator for core::ops::range::Range<A>>::next (self=<optimized out>)
    at /home/mark/Build/rust/src/libcore/iter/range.rs:212
#1  <rustc_mir::interpret::memory::Memory<'a, 'mir, 'tcx, M>>::copy_undef_mask (self=<optimized out>, src=..., dest=..., size=...,
    repeat=<optimized out>) at src/librustc_mir/interpret/memory.rs:790
#2  <rustc_mir::interpret::memory::Memory<'a, 'mir, 'tcx, M>>::copy_repeatedly (self=0x7f045e41c1f8, src=<optimized out>, src_align=...,
    dest=<optimized out>, dest_align=..., size=..., length=1, nonoverlapping=<optimized out>) at src/librustc_mir/interpret/memory.rs:766
#3  0x00007f0462835528 in <rustc_mir::interpret::memory::Memory<'a, 'mir, 'tcx, M>>::copy (self=0x7f045e41c1f8, src_align=..., dest_align=...,
    size=..., nonoverlapping=255, src=<optimized out>, dest=<optimized out>) at src/librustc_mir/interpret/memory.rs:673
#4  rustc_mir::interpret::place::<impl rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>>::copy_op_no_validate (
    self=<optimized out>, src=..., dest=...) at src/librustc_mir/interpret/place.rs:838
#5  0x00007f0462846287 in rustc_mir::interpret::place::<impl rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>>::copy_op (
    self=<optimized out>, src=..., dest=...) at src/librustc_mir/interpret/place.rs:796
#6  rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>>::eval_rvalue_into_place (
    self=0x7f045e41c190, rvalue=<optimized out>, place=<optimized out>) at src/librustc_mir/interpret/step.rs:147
#7  0x00007f0462844e39 in rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>>::statement (
    self=<optimized out>, stmt=<optimized out>) at src/librustc_mir/interpret/step.rs:85
#8  rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>>::step (self=0x7f045e41c190)
    at src/librustc_mir/interpret/step.rs:61
#9  rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>>::run (self=0x7f045e41c190)
    at src/librustc_mir/interpret/step.rs:40
#10 0x00007f0462a5b50e in rustc_mir::const_eval::eval_body_using_ecx (ecx=0x7f045e41c190, cid=..., mir=<optimized out>, param_env=...)
    at src/librustc_mir/const_eval.rs:157
#11 0x00007f0462a5ea59 in rustc_mir::const_eval::eval_body_and_ecx (tcx=..., cid=...,
    mir=<unknown type in /home/mark/.rustup/toolchains/stage1/lib/librustc_mir-cd23d8cc09622ccc.so, CU 0x17247a7, DIE 0x1901b01>, param_env=...)
    at src/librustc_mir/const_eval.rs:120
#12 rustc_mir::const_eval::const_eval_raw_provider (tcx=..., key=...) at src/librustc_mir/const_eval.rs:636
#13 0x00007f04627392cf in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::const_eval_raw<'tcx>>::compute::{{closure}} () at /home/mark/Build/rust/src/librustc/ty/query/plumbing.rs:963
#14 rustc::ty::query::__query_compute::const_eval_raw (f=...) at /home/mark/Build/rust/src/librustc/ty/query/plumbing.rs:922
#15 0x00007f0462413cfc in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::const_eval_raw<'tcx>>::compute (tcx=..., key=<error reading variable: Cannot access memory at address 0x32d7007>)
    at /home/mark/Build/rust/src/librustc/ty/query/plumbing.rs:955
#16 0x00007f046277817e in rustc::dep_graph::graph::DepGraph::with_task_impl (self=<optimized out>, key=..., cx=..., arg=..., no_tcx=false,
    task=<optimized out>, create_task=0x7f046272a400 <core::ops::function::FnOnce::call_once>,
    finish_task_and_alloc_depnode=0x7f046272a200 <core::ops::function::FnOnce::call_once>, hash_result=<optimized out>)
    at /home/mark/Build/rust/src/librustc/dep_graph/graph.rs:334
#17 0x00007f04625b994c in rustc::dep_graph::graph::DepGraph::with_task (self=0x1, cx=..., task=0x7f03314df010, key=..., arg=...,
    hash_result=<optimized out>) at /home/mark/Build/rust/src/librustc/dep_graph/graph.rs:202
#18 rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job::{{closure}}::{{closure}} (tcx=...)
    at /home/mark/Build/rust/src/librustc/ty/query/plumbing.rs:567
#19 rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::start_query::{{closure}}::{{closure}} ()
    at /home/mark/Build/rust/src/librustc/ty/query/plumbing.rs:280
#20 rustc::ty::context::tls::enter_context::{{closure}} () at /home/mark/Build/rust/src/librustc/ty/context.rs:1955
#21 rustc::ty::context::tls::set_tlv (f=..., value=<optimized out>) at /home/mark/Build/rust/src/librustc/ty/context.rs:1888
#22 rustc::ty::context::tls::enter_context (f=..., context=<optimized out>) at /home/mark/Build/rust/src/librustc/ty/context.rs:1954
#23 rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::start_query::{{closure}} (current_icx=<optimized out>)
    at /home/mark/Build/rust/src/librustc/ty/query/plumbing.rs:279

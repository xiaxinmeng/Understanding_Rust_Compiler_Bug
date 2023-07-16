
rustc_mir::transform::promote_consts::Promoter::promote_temp
<rustc_mir::transform::promote_consts::Promoter<'a, 'tcx> as rustc::mir::visit::MutVisitor<'tcx>>::visit_lvalue
[...] (lots and lots of these two functions calling each other)
<rustc_mir::transform::qualify_consts::QualifyAndPromoteConstants as rustc::mir::transform::MirMapPass<'tcx>>::run_pass
rustc::mir::transform::Passes::run_passes
rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::{{closure}}
[...] (I hope from here on its the same for each rustc invocation)

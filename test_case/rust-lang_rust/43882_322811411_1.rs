
DEBUG:rustc::infer::error_reporting: report_region_errors: error = ConcreteFailure(Reborrow(src/test/ui/lifetime-errors/ex3-both-anon-regions.rs:12:12: 12:13), ReFree(DefId { krate: CrateNum(0), node: DefIndex(3) => ex3_both_anon_regions/8cd878b::foo[0] }, BrNamed(CrateNum(0):DefIndex(2147483657), 'a(86))), ReFree(DefId { krate: CrateNum(0), node: DefIndex(3) => ex3_both_anon_regions/8cd878b::foo[0] }, BrNamed(CrateNum(0):DefIndex(2147483658), 'b(88))))
DEBUG:rustc::infer::error_reporting::named_anon_conflict: try_report_named_anon_conflict
DEBUG:rustc::infer::error_reporting::named_anon_conflict: 1.1
DEBUG:rustc::infer::error_reporting::util: arg=Arg { pat: pat(16: y), id: NodeId(15) }
DEBUG:rustc::infer::error_reporting::anon_anon_conflict: try_report_anon_anon_conflict
DEBUG:rustc::infer::error_reporting::anon_anon_conflict: LateBound depth = 1 self.infcx.tcx.hir.local_def_id(id) =DefId { krate: CrateNum(0), node: DefIndex(2147483658) => ex3_both_anon_regions/8cd878b::foo[0]::'b[0] }
                        def_id=DefId { krate: CrateNum(0), node: DefIndex(2147483658) => ex3_both_anon_regions/8cd878b::foo[0]::'b[0] }
DEBUG:rustc::infer::error_reporting::anon_anon_conflict: LateBound depth = 1 self.infcx.tcx.hir.local_def_id(id) =DefId { krate: CrateNum(0), node: DefIndex(2147483658) => ex3_both_anon_regions/8cd878b::foo[0]::'b[0] }
                        def_id=DefId { krate: CrateNum(0), node: DefIndex(2147483657) => ex3_both_anon_regions/8cd878b::foo[0]::'a[0] }


[DEBUG rustc_query_system::query::plumbing] ty::query::get_query<type_of>(key=DefId(0:5 ~ fk[317d]::Recurse[0]::0[0]), span=src/test/ui/__check/fk.rs:1:1: 1:1 (#0))
[DEBUG rustc_hir::definitions] DefPath::make: krate=crate0 index=Some(DefIndex(3))
[DEBUG rustc_hir::definitions] DefPath::make: key=DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("Recurse"), disambiguator: 0 } }
[DEBUG rustc_hir::definitions] DefPath::make: krate=crate0 index=Some(DefIndex(0))
[DEBUG rustc_hir::definitions] DefPath::make: key=DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } }
[DEBUG rustc_middle::ty::print::pretty] try_print_visible_def_path: def_id=DefId(0:3 ~ fk[317d]::Recurse[0])
[DEBUG rustc_hir::definitions] DefPath::make: krate=crate0 index=Some(DefIndex(3))
[DEBUG rustc_hir::definitions] DefPath::make: key=DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("Recurse"), disambiguator: 0 } }
[DEBUG rustc_hir::definitions] DefPath::make: krate=crate0 index=Some(DefIndex(0))
[DEBUG rustc_hir::definitions] DefPath::make: key=DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } }
[DEBUG rustc_middle::ty::print] default_print_def_path: def_id=DefId(0:3 ~ fk[317d]::Recurse[0]), substs=[]
[DEBUG rustc_middle::ty::print] default_print_def_path: key=DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("Recurse"), disambiguator: 0 } }
[DEBUG rustc_hir::definitions] DefPath::make: krate=crate0 index=Some(DefIndex(0))
[DEBUG rustc_hir::definitions] DefPath::make: key=DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } }
[DEBUG rustc_middle::ty::print::pretty] try_print_visible_def_path: def_id=DefId(0:0 ~ fk[317d])
[DEBUG rustc_middle::ty::fold] HasTypeFlagsVisitor: t=Recurse t.flags=(empty) self.flags=HAS_TY_PARAM | HAS_RE_PARAM | HAS_CT_PARAM | NEEDS_SUBST
[DEBUG rustc_hir::definitions] DefPath::make: krate=crate0 index=Some(DefIndex(5))
[DEBUG rustc_hir::definitions] DefPath::make: key=DefKey { parent: Some(DefIndex(3)), disambiguated_data: DisambiguatedDefPathData { data: ValueNs("0"), disambiguator: 0 } }
[DEBUG rustc_hir::definitions] DefPath::make: krate=crate0 index=Some(DefIndex(3))
[DEBUG rustc_hir::definitions] DefPath::make: key=DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("Recurse"), disambiguator: 0 } }
[DEBUG rustc_hir::definitions] DefPath::make: krate=crate0 index=Some(DefIndex(0))
[DEBUG rustc_hir::definitions] DefPath::make: key=DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } }
[DEBUG rustc_span::source_map] byte pos BytePos(0) is on the line at byte pos BytePos(0)
[DEBUG rustc_span::source_map] char pos CharPos(0) is on the line at char pos CharPos(0)
[DEBUG rustc_span::source_map] byte is on line: 1
[DEBUG rustc_span::source_map] byte pos BytePos(0) is on the line at byte pos BytePos(0)
[DEBUG rustc_span::source_map] char pos CharPos(0) is on the line at char pos CharPos(0)
[DEBUG rustc_span::source_map] byte is on line: 1

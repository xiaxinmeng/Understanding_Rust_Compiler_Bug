
DEBUG 2019-07-06T16:31:24Z: rustc::ty::print::pretty: try_print_visible_def_path: cur_def_key=DefKey { parent: Some(DefIndex(1432)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("Sized"), disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc_metadata::decoder: def_path(cnum=crate2, id=DefIndex(1432))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate2 index=Some(DefIndex(1432))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("marker"), disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate2 index=Some(DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::ty::print::pretty: try_print_visible_def_path: def_id=DefId(2:1432 ~ core[f7fb]::marker[0])
DEBUG 2019-07-06T16:31:24Z: rustc::ty::query::plumbing: ty::query::get_query<visible_parent_map>(key=crate0, span=file6.rs:1:1: 1:1)
DEBUG 2019-07-06T16:31:24Z: rustc::ty::print::pretty: try_print_visible_def_path: cur_def_key=DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("marker"), disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc_metadata::decoder: def_path(cnum=crate1, id=DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate1 index=Some(DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::ty::print::pretty: try_print_visible_def_path: def_id=DefId(1:0 ~ std[e8ce])
DEBUG 2019-07-06T16:31:24Z: rustc_metadata::decoder: def_path(cnum=crate1, id=DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate1 index=Some(DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::ty::query::plumbing: ty::query::get_query<extern_crate>(key=DefId(1:0 ~ std[e8ce]), span=file6.rs:1:1: 1:1)
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate0 index=Some(DefIndex(11))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("std"), disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate0 index=Some(DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::ty::print::pretty: try_print_visible_def_path: def_id=DefId(0:11 ~ file6[317d]::std[0])
DEBUG 2019-07-06T16:31:24Z: rustc::ty::query::plumbing: ty::query::get_query<crate_name>(key=crate1, span=file6.rs:1:1: 1:1)
DEBUG 2019-07-06T16:31:24Z: rustc_metadata::decoder: def_path(cnum=crate1, id=DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate1 index=Some(DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc_metadata::decoder: def_path(cnum=crate2, id=DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate2 index=Some(DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::ty::print::pretty: try_print_visible_def_path: visible_parent=DefId(1:0 ~ std[e8ce]) actual_parent=Some(DefId(2:0 ~ core[f7fb]))
DEBUG 2019-07-06T16:31:24Z: rustc_metadata::decoder: def_path(cnum=crate1, id=DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate1 index=Some(DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc_metadata::decoder: def_path(cnum=crate2, id=DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate2 index=Some(DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::ty::print::pretty: try_print_visible_def_path: data=TypeNs("marker") visible_parent=DefId(1:0 ~ std[e8ce]) actual_parent=Some(DefId(2:0 ~ core[f7fb]))
DEBUG 2019-07-06T16:31:24Z: rustc_metadata::decoder: def_path(cnum=crate1, id=DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate1 index=Some(DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::ty::query::plumbing: ty::query::get_query<item_children>(key=DefId(1:0 ~ std[e8ce]), span=file6.rs:1:1: 1:1)
DEBUG 2019-07-06T16:31:24Z: rustc::ty::print::pretty: try_print_visible_def_path: data=TypeNs("marker")
DEBUG 2019-07-06T16:31:24Z: rustc_metadata::decoder: def_path(cnum=crate2, id=DefIndex(1432))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate2 index=Some(DefIndex(1432))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("marker"), disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate2 index=Some(DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc_metadata::decoder: def_path(cnum=crate2, id=DefIndex(1432))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate2 index=Some(DefIndex(1432))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("marker"), disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate2 index=Some(DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::ty::print::pretty: try_print_visible_def_path: visible_parent=DefId(2:1432 ~ core[f7fb]::marker[0]) actual_parent=Some(DefId(2:1432 ~ core[f7fb]::marker[0]))
DEBUG 2019-07-06T16:31:24Z: rustc_metadata::decoder: def_path(cnum=crate2, id=DefIndex(1432))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate2 index=Some(DefIndex(1432))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("marker"), disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate2 index=Some(DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc_metadata::decoder: def_path(cnum=crate2, id=DefIndex(1432))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate2 index=Some(DefIndex(1432))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("marker"), disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate2 index=Some(DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::ty::print::pretty: try_print_visible_def_path: data=TypeNs("Sized") visible_parent=DefId(2:1432 ~ core[f7fb]::marker[0]) actual_parent=Some(DefId(2:1432 ~ core[f7fb]::marker[0]))
DEBUG 2019-07-06T16:31:24Z: rustc::ty::print::pretty: try_print_visible_def_path: data=TypeNs("Sized")
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate0 index=Some(DefIndex(23))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: Some(DefIndex(22)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("Input"), disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate0 index=Some(DefIndex(22))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("MyTrait"), disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate0 index=Some(DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::ty::print: default_print_def_path: def_id=DefId(0:23 ~ file6[317d]::MyTrait[0]::Input[0]), substs=[Self]
DEBUG 2019-07-06T16:31:24Z: rustc::ty::print: default_print_def_path: key=DefKey { parent: Some(DefIndex(22)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("Input"), disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate0 index=Some(DefIndex(23))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: Some(DefIndex(22)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("Input"), disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate0 index=Some(DefIndex(22))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("MyTrait"), disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate0 index=Some(DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::ty::query::plumbing: ty::query::get_query<generics_of>(key=DefId(0:23 ~ file6[317d]::MyTrait[0]::Input[0]), span=file6.rs:1:1: 1:1)
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate0 index=Some(DefIndex(22))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("MyTrait"), disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate0 index=Some(DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::ty::query::plumbing: ty::query::get_query<generics_of>(key=DefId(0:22 ~ file6[317d]::MyTrait[0]), span=file6.rs:1:1: 1:1)
DEBUG 2019-07-06T16:31:24Z: rustc::ty::fold: HasTypeFlagsVisitor: t=Self t.flags=HAS_SELF | HAS_FREE_LOCAL_NAMES self.flags=KEEP_IN_LOCAL_TCX
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate0 index=Some(DefIndex(22))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("MyTrait"), disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate0 index=Some(DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::ty::print: default_print_def_path: def_id=DefId(0:22 ~ file6[317d]::MyTrait[0]), substs=[Self]
DEBUG 2019-07-06T16:31:24Z: rustc::ty::print: default_print_def_path: key=DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("MyTrait"), disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate0 index=Some(DefIndex(22))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("MyTrait"), disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate0 index=Some(DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::ty::query::plumbing: ty::query::get_query<generics_of>(key=DefId(0:22 ~ file6[317d]::MyTrait[0]), span=file6.rs:1:1: 1:1)
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate0 index=Some(DefIndex(22))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("MyTrait"), disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate0 index=Some(DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::ty::print::pretty: try_print_visible_def_path: def_id=DefId(0:22 ~ file6[317d]::MyTrait[0])
DEBUG 2019-07-06T16:31:24Z: rus^CDEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("MyTrait"), disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate0 index=Some(DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::ty::print: default_print_def_path: def_id=DefId(0:22 ~ file6[317d]::MyTrait[0]), substs=[]
DEBUG 2019-07-06T16:31:24Z: rustc::ty::print: default_print_def_path: key=DefKey { parent: Some(DefIndex(0)), disambiguated_data: DisambiguatedDefPathData { data: TypeNs("MyTrait"), disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: krate=crate0 index=Some(DefIndex(0))
DEBUG 2019-07-06T16:31:24Z: rustc::hir::map::definitions: DefPath::make: key=DefKey { parent: None, disambiguated_data: DisambiguatedDefPathData { data: CrateRoot, disambiguator: 0 } }
DEBUG 2019-07-06T16:31:24Z: rustc::ty::print::pretty: try_print_visible_def_path: def_id=DefId(0:0 ~ file6[317d])

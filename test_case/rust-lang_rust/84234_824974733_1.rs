
compiler/rustc_codegen_ssa/src/back/linker.rs
1253:    let formats = tcx.dependency_formats(LOCAL_CRATE);
1254-    let deps = formats.iter().find_map(|(t, list)| (*t == crate_type).then_some(list)).unwrap();

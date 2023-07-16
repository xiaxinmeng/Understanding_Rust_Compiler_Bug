Rust
    if is_extern && !std_internal {
        let target = &tcx.sess.target.target.llvm_target;
        // WebAssembly cannot export data symbols, so reduce their export level
        if target.contains("wasm32") || target.contains("emscripten") {
            if let Some(Node::Item(&hir::Item { kind: hir::ItemKind::Static(..), .. })) =
                tcx.hir().get_if_local(sym_def_id)
            {
                return SymbolExportLevel::Rust;
            }
        }

        SymbolExportLevel::C
    } else {
        SymbolExportLevel::Rust
    }

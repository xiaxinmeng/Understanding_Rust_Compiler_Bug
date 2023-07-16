rust
let export_level = if special_runtime_crate {
                        // We can probably do better here by just ensuring that
                        // it has hidden visibility rather than public
                        // visibility, as this is primarily here to ensure it's
                        // not stripped during LTO.
                        //
                        // In general though we won't link right if these
                        // symbols are stripped, and LTO currently strips them.
                        if &*name == "rust_eh_personality" ||
                           &*name == "rust_eh_register_frames" ||
                           &*name == "rust_eh_unregister_frames" {
                            SymbolExportLevel::C
                        } else {
                            SymbolExportLevel::Rust
                        }
                    } else if (&*name).starts_with("__rdl_") ||
                              (&*name).starts_with("__rust_") { 
                        SymbolExportLevel::Rust
                    } else {
                        export_level(scx, def_id)
                    };

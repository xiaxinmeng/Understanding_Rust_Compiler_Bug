
  if (CodeGenOpts.DwarfVersion)
    // We actually want the latest version when there are conflicts.
    // We can change from Warning to Latest if such mode is supported.
    getModule().addModuleFlag(llvm::Module::Warning, "Dwarf Version",
                              CodeGenOpts.DwarfVersion);
  if (DebugInfo)
    // We support a single version in the linked module: error out when
    // modules do not have the same version. We are going to implement dropping
    // debug info when the version number is not up-to-date. Once that is
    // done, the bitcode linker is not going to see modules with different
    // version numbers.
    getModule().addModuleFlag(llvm::Module::Error, "Debug Info Version",
                              llvm::DEBUG_METADATA_VERSION);

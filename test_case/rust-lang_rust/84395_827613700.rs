c++
FunctionImporter Importer(CombinedIndex, ModuleLoader);
  if (Error Err = Importer.importFunctions(Mod, ImportList).takeError())
    return Err;

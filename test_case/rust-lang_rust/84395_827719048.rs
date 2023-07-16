
static void codegen(const Config &Conf, TargetMachine *TM,                                                                                                                                                          
                    AddStreamFn AddStream, unsigned Task, Module &Mod,                                                                                                                                              
                    const ModuleSummaryIndex &CombinedIndex) {                                                                                                                                                      
  errs() << "FFFFFFFFFFFFFFFFFFFFFFFF\n";                                                                                                                                                                           
  if (Conf.PreCodeGenModuleHook && !Conf.PreCodeGenModuleHook(Task, Mod))                                                                                                                                           
    return;                                                                                                                                                                                                         
                                                                                                                                                                                                                    
  errs() << "YYYYYYYYYYYYYYYYYYYY\n";                                                                                                                                                                               
  if (EmbedBitcode == LTOBitcodeEmbedding::EmbedOptimized) {                                                                                                                                                        
    errs() << "XXXXXXXXXXXXXXXXXXXX\n";                                                                                                                                                                             
    llvm::EmbedBitcodeInModule(Mod, llvm::MemoryBufferRef(),                                                                                                                                                        
                               /*EmbedBitcode*/ true,                                                                                                                                                               
                               /*EmbedCmdline*/ false,                                                                                                                                                              
                               /*CmdArgs*/ std::vector<uint8_t>());                                                                                                                                                 
  }                                                                                                                                                                                                                 
  abort(); 

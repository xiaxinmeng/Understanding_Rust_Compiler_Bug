cpp
    auto *mainTy = llvm::FunctionType::get(llvm::Type::getInt32Ty(theContext), { llvm::Type::getInt32Ty(theContext), llvm::Type::getInt8PtrTy(theContext)->getPointerTo() }, false);

    auto *mainFn = llvm::Function::Create(mainTy, llvm::GlobalValue::LinkageTypes::ExternalLinkage, "main", module);
    auto *entry = llvm::BasicBlock::Create(theContext, "entry", mainFn);

    llvm::IRBuilder<> builder(theContext);
    builder.SetInsertPoint(entry);
    auto *inst = builder.CreateRet(llvm::ConstantInt::get(llvm::Type::getInt32Ty(theContext), 0));

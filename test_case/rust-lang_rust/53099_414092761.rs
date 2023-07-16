
    } else if (retTy->isAggregateType() || retTy->isVectorTy() || retTy->isIntegerTy(128)) {
      auto &DL = CS.getCalledFunction()->getParent()->getDataLayout();
      O << ".param .align " << retAlignment << " .b8 _["
        << DL.getTypeAllocSize(retTy) << "]";
    } else {

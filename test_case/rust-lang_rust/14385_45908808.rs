 cpp
return wrap(Builder->createStructType(
        unwrapDI<DIDescriptor>(Scope),
        Name,
        unwrapDI<DIFile>(File),
        LineNumber,
        SizeInBits,
        AlignInBits,
        Flags,
        unwrapDI<DIType>(DerivedFrom),
        unwrapDI<DIArray>(Elements),
        RunTimeLang,
        unwrapDI<DIType>(VTableHolder)
#if LLVM_VERSION_MINOR >= 5
        ,UniqueId
#endif
        ));

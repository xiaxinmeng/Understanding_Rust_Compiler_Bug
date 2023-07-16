cxx
    AMemSet = Builder.CreateMemSet(StartPtr, ByteVal, Range.End - Range.Start,
                                   MaybeAlign(Range.Alignment));


define internal fastcc { i8*, i64 } @_ZN3lib5Slice10into_value17hd068c2a4c36b2989E(i8* nonnull align 1 %0, i64 %1) unnamed_addr #6 {
  %3 = insertvalue { i8*, i64 } undef, i8* %0, 0
  %4 = insertvalue { i8*, i64 } %3, i64 %1, 1

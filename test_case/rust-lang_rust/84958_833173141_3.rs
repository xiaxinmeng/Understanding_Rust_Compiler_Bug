
  %4 = call fastcc { i8*, i64 } @_ZN4main22read_single_byte_value17h6a8946d030728332E(%Reader* nonnull align 8 dereferenceable(24) %input) #7
  %value.0.i = extractvalue { i8*, i64 } %4, 0
  %value.1.i = extractvalue { i8*, i64 } %4, 1
  %5 = tail call fastcc { i8*, i64 } @_ZN3lib5Slice10into_value17hd068c2a4c36b2989E(i8* noalias nonnull readonly align 1 %value.0.i, i64 %value.1.i) #7, !noalias !9

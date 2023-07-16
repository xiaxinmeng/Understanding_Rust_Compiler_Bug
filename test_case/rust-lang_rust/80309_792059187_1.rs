llvm
  %wrapping_add = getelementptr i8, i8* null, i64 %b_ptr
  %deref = load i8, i8* %wrapping_add, align 1

; becomes=>

  %deref = i8 undef ; because we're dereferencing a pointer with a provenance of a null object.

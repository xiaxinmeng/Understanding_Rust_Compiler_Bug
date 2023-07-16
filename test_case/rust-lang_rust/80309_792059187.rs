llvm
; code corresponds to `x.wrapping_sub(x as usize)`
define i8* @initial(i8* %b) {
  %b_ptr = ptrtoint i8* %b to i64
  %sub = sub i64 0, %b_ptr
  %gep = getelementptr i8, i8* %b, i64 %sub
  ret i8* %gep
}

define i8* @step1(i8* %b) {        ; no provenances remain in this function, ptrtoint discards that information
  %b_ptr = ptrtoint i8* %b to i64
  %1 = ptrtoint i8* %b to i64
  %2 = sub i64 %1, %b_ptr          ; constant 0
  %gep = inttoptr i64 %2 to i8*    ; becomes a constant `null` pointer, with a provenance of null object
  ret i8* %gep
}

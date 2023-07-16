llvm
; ModuleID = 'foo.ll'
source_filename = "foo.ll"
target triple = "wasm32-unknown-unknown"

@wut = private constant i128 8000000000000000000

define i128 @get() {
start:
  %0 = tail call { i64, i128 } @decode(i8* bitcast (i128* @wut to i8*))
  %1 = extractvalue { i64, i128 } %0, 1
  ret i128 %1
}

declare hidden { i64, i128 } @decode(i8* nocapture readonly)

 llvm
target datalayout = "e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: uwtable
define internal void @_ZN4main20h1c12588f54a45a2feaaE() unnamed_addr #0 {
entry-block:
  ret void
}

define i64 @main(i64, i8**) unnamed_addr #1 {
top:
  %2 = call i64 @_ZN2rt10lang_start20h09857311e158dc649GyE(i8* bitcast (void ()* @_ZN4main20h1c12588f54a45a2feaaE to i8*), i64 %0, i8** %1)
  ret i64 %2
}

declare i64 @_ZN2rt10lang_start20h09857311e158dc649GyE(i8*, i64, i8**) unnamed_addr #1

define internal void @_ZN4main3foo20h2bc621e3a5534caalaaE() unnamed_addr #1 {
"the block":
  call void @_ZN4main3foo10__rust_abiE()
  ret void
}

; Function Attrs: uwtable
define internal void @_ZN4main3foo10__rust_abiE() unnamed_addr #0 {
entry-block:
  ret void
}

attributes #0 = { uwtable "split-stack" }
attributes #1 = { "split-stack" }

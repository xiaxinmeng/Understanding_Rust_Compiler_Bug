llvm
; ModuleID = 'bugpoint-reduced-simplified.bc'
source_filename = "bugpoint-output-7eb0c3b.bc"
target triple = "x86_64-unknown-linux-gnu"

%foo = type { %bar*, %baz* }
%bar = type { %bil*, i8*, %baz* }
%bil = type { i32, [0 x i32], [1 x i32] }
%baz = type { i8*, i64 }

define fastcc void @fail_sroa() {
entry-block:
  %arg5 = alloca %foo, align 8

  %0 = bitcast %foo* %arg5 to i64*
  store i64 undef, i64* %0, align 8

  %1 = getelementptr inbounds %foo, %foo* %arg5, i64 0, i32 0
  %2 = load %bar*, %bar** %1, align 8, !nonnull !0

  unreachable
}

!0 = !{}

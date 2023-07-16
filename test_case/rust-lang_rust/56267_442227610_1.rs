
%"Foo<(i32, i32)>" = type { [0 x i64], i64, [0 x i32], { i32, i32 }, [0 x i32] }

; file2::test
; Function Attrs: nonlazybind uwtable
define void @_ZN5file24test17h241867b5d90e9ca9E(%"Foo<(i32, i32)>"* noalias nocapture sret dereferenceable(16), i32 %x.0, i32 %x.1) unnamed_addr #0 {
start:
  %1 = bitcast %"Foo<(i32, i32)>"* %0 to i64*
  store i64 0, i64* %1, align 8
  %2 = getelementptr inbounds %"Foo<(i32, i32)>", %"Foo<(i32, i32)>"* %0, i32 0, i32 3
  %3 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %2, i32 0, i32 0
  store i32 %x.0, i32* %3, align 8
  %4 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %2, i32 0, i32 1
  store i32 %x.1, i32* %4, align 8
  ret void
}

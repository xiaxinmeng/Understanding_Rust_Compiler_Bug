
declare { i8*, i64 } @get()

declare void @use(i8*, i64)
      
define void @test(i64* %p) {
  %struct = call { i8*, i64 } @get()
  %struct.1 = extractvalue { i8*, i64 } %struct, 1
  store i64 %struct.1, i64* %p, align 8
  %struct.2 = extractvalue { i8*, i64 } %struct, 1
  call void @use(i8* undef, i64 %struct.2)
  ret void
}   

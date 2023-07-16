
%L = type { [0 x i64], %SV, [0 x i64], %SV, [0 x i64] }
%SV = type { [0 x i64], i64, [0 x i64], i64, [0 x i64], [40 x i64], [0 x i64] }

define void @test(i64* nocapture %a) unnamed_addr #0 {
start:
  %_4 = alloca %L, align 8
  %_4.0.sroa_cast = bitcast %L* %_4 to i8*
  call void @llvm.lifetime.start.p0i8(i64 672, i8* nonnull %_4.0.sroa_cast)
  %_41213 = bitcast %L* %_4 to i8*
  call void @llvm.memset.p0i8.i64(i8* nonnull align 8 dereferenceable(16) %_41213, i8 0, i64 16, i1 false)
  %0 = getelementptr inbounds %L, %L* %_4, i64 0, i32 3
  %1 = bitcast %SV* %0 to i8*
  call void @llvm.memset.p0i8.i64(i8* nonnull align 8 dereferenceable(16) %1, i8 0, i64 16, i1 false)
  %2 = bitcast i64* %a to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 8 dereferenceable(672) %2, i8* nonnull align 8 dereferenceable(672) %_4.0.sroa_cast, i64 672, i1 false) #3
  call void @llvm.lifetime.end.p0i8(i64 672, i8* nonnull %_4.0.sroa_cast)
  ret void
}

declare void @llvm.lifetime.start.p0i8(i64 immarg, i8* nocapture) #1

declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #1

declare void @llvm.lifetime.end.p0i8(i64 immarg, i8* nocapture) #1

declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #2

llvm
; std::net::addr::SocketAddr::new
; Function Attrs: nounwind uwtable
define void @_ZN3std3net4addr10SocketAddr3new17hcf120d42abe9cbefE(%"net::addr::SocketAddr"* noalias nocapture sret dereferenceable(32), %"net::ip::IpAddr"* noalias nocapture readonly dereferenceable(20) %ip, i16 %port) unnamed_addr #2 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality !dbg !5534 {
start:
  %_10.sroa.0 = alloca [16 x i8], align 4
  %a1.sroa.0 = alloca [16 x i8], align 4
  %1 = bitcast %"net::ip::IpAddr"* %ip to i32*, !dbg !5535
  %2 = load i32, i32* %1, align 4, !dbg !5535, !range !5538
  %switch = icmp eq i32 %2, 1, !dbg !5535
  br i1 %switch, label %bb3, label %bb2, !dbg !5535

bb2:                                              ; preds = %start
  %a.sroa.0.0..sroa_idx2 = getelementptr inbounds %"net::ip::IpAddr", %"net::ip::IpAddr"* %ip, i64 0, i32 0, i64 4, !dbg !5535
  %a.sroa.0.0..sroa_cast = bitcast i8* %a.sroa.0.0..sroa_idx2 to i32*, !dbg !5535
  %a.sroa.0.0.copyload = load i32, i32* %a.sroa.0.0..sroa_cast, align 4, !dbg !5535
  %3 = tail call i16 @llvm.bswap.i16(i16 %port) #12, !dbg !5539
  %4 = bitcast %"net::addr::SocketAddr"* %0 to i32*, !dbg !5546
  store i32 0, i32* %4, align 4, !dbg !5546
  %_6.sroa.0.0..sroa_idx = getelementptr inbounds %"net::addr::SocketAddr", %"net::addr::SocketAddr"* %0, i64 0, i32 0, i64 4, !dbg !5546
  %_6.sroa.0.0..sroa_cast = bitcast i8* %_6.sroa.0.0..sroa_idx to i16*, !dbg !5546
  store i16 2, i16* %_6.sroa.0.0..sroa_cast, align 4, !dbg !5546
  %_6.sroa.4.0..sroa_idx = getelementptr inbounds %"net::addr::SocketAddr", %"net::addr::SocketAddr"* %0, i64 0, i32 0, i64 6, !dbg !5546
  %_6.sroa.4.0..sroa_cast = bitcast i8* %_6.sroa.4.0..sroa_idx to i16*, !dbg !5546
  store i16 %3, i16* %_6.sroa.4.0..sroa_cast, align 2, !dbg !5546
  %_6.sroa.5.0..sroa_idx = getelementptr inbounds %"net::addr::SocketAddr", %"net::addr::SocketAddr"* %0, i64 0, i32 0, i64 8, !dbg !5546
  %_6.sroa.5.0..sroa_cast = bitcast i8* %_6.sroa.5.0..sroa_idx to i32*, !dbg !5546
  store i32 %a.sroa.0.0.copyload, i32* %_6.sroa.5.0..sroa_cast, align 4, !dbg !5546
  %_6.sroa.6.0..sroa_idx = getelementptr inbounds %"net::addr::SocketAddr", %"net::addr::SocketAddr"* %0, i64 0, i32 0, i64 12, !dbg !5546
  %_6.sroa.6.0..sroa_cast = bitcast i8* %_6.sroa.6.0..sroa_idx to i64*, !dbg !5546
  store i64 0, i64* %_6.sroa.6.0..sroa_cast, align 4, !dbg !5546
  %.pre = getelementptr inbounds [16 x i8], [16 x i8]* %a1.sroa.0, i64 0, i64 0, !dbg !5547
  br label %bb4, !dbg !5548

bb3:                                              ; preds = %start
  %a1.sroa.0.0.sroa_idx10 = getelementptr inbounds [16 x i8], [16 x i8]* %a1.sroa.0, i64 0, i64 0, !dbg !5549
  call void @llvm.lifetime.start(i64 16, i8* nonnull %a1.sroa.0.0.sroa_idx10), !dbg !5549
  %5 = getelementptr inbounds %"net::ip::IpAddr", %"net::ip::IpAddr"* %ip, i64 0, i32 0, i64 4, !dbg !5549
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull %a1.sroa.0.0.sroa_idx10, i8* %5, i64 16, i32 4, i1 false), !dbg !5549
  %_10.sroa.0.0.sroa_idx4 = getelementptr inbounds [16 x i8], [16 x i8]* %_10.sroa.0, i64 0, i64 0, !dbg !5549
  ; @@@@ WHY ARE WE MAKING THIS CALL TO LIFETIME.START/LIFETIME.END??? @@@@
  call void @llvm.lifetime.start(i64 16, i8* nonnull %_10.sroa.0.0.sroa_idx4), !dbg !5549
  %6 = tail call i16 @llvm.bswap.i16(i16 %port) #12, !dbg !5550
  call void @llvm.lifetime.end(i64 16, i8* nonnull %_10.sroa.0.0.sroa_idx4), !dbg !5549
  %7 = bitcast %"net::addr::SocketAddr"* %0 to i32*, !dbg !5549
  store i32 1, i32* %7, align 4, !dbg !5549
  %_9.sroa.0.0..sroa_idx = getelementptr inbounds %"net::addr::SocketAddr", %"net::addr::SocketAddr"* %0, i64 0, i32 0, i64 4, !dbg !5549
  %_9.sroa.0.0..sroa_cast = bitcast i8* %_9.sroa.0.0..sroa_idx to i16*, !dbg !5549
  store i16 10, i16* %_9.sroa.0.0..sroa_cast, align 4, !dbg !5549
  %_9.sroa.4.0..sroa_idx = getelementptr inbounds %"net::addr::SocketAddr", %"net::addr::SocketAddr"* %0, i64 0, i32 0, i64 6, !dbg !5549
  %_9.sroa.4.0..sroa_cast = bitcast i8* %_9.sroa.4.0..sroa_idx to i16*, !dbg !5549
  store i16 %6, i16* %_9.sroa.4.0..sroa_cast, align 2, !dbg !5549
  %_9.sroa.5.0..sroa_idx = getelementptr inbounds %"net::addr::SocketAddr", %"net::addr::SocketAddr"* %0, i64 0, i32 0, i64 8, !dbg !5549
  %_9.sroa.5.0..sroa_cast = bitcast i8* %_9.sroa.5.0..sroa_idx to i32*, !dbg !5549
  store i32 0, i32* %_9.sroa.5.0..sroa_cast, align 4, !dbg !5549
  %_10.sroa.0.0._9.sroa.6.0..sroa_idx.sroa_idx = getelementptr inbounds %"net::addr::SocketAddr", %"net::addr::SocketAddr"* %0, i64 0, i32 0, i64 12, !dbg !5549
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %_10.sroa.0.0._9.sroa.6.0..sroa_idx.sroa_idx, i8* nonnull %_10.sroa.0.0.sroa_idx4, i64 16, i32 4, i1 false), !dbg !5549
  %_9.sroa.7.0..sroa_idx = getelementptr inbounds %"net::addr::SocketAddr", %"net::addr::SocketAddr"* %0, i64 0, i32 0, i64 28, !dbg !5549
  %_9.sroa.7.0..sroa_cast = bitcast i8* %_9.sroa.7.0..sroa_idx to i32*, !dbg !5549
  store i32 0, i32* %_9.sroa.7.0..sroa_cast, align 4, !dbg !5549
  br label %bb4, !dbg !5548

bb4:                                              ; preds = %bb3, %bb2
  %a1.sroa.0.0.sroa_idx13.pre-phi = phi i8* [ %.pre, %bb2 ], [ %a1.sroa.0.0.sroa_idx10, %bb3 ], !dbg !5547
  call void @llvm.lifetime.end(i64 16, i8* nonnull %a1.sroa.0.0.sroa_idx13.pre-phi), !dbg !5547
  ret void, !dbg !5557
}


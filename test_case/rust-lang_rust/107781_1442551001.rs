
The unwind destination does not have an exception handling instruction!
  invoke void @_ZN3std9panicking11begin_panic17h06d72bd16276e249E(ptr noalias noundef nonnull readonly align 1 @anon.de822ab2b20f7769a577e3136c8d4650.29, i64 53, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) @anon.de822ab2b20f7769a577e3136c8d4650.108) #33
          to label %175 unwind label %176
CleanupPadInst not the first non-PHI instruction in the block.
  %180 = cleanuppad within none []
CleanupReturnInst must unwind to an EH block which is not a landingpad.
  cleanupret from %190 unwind label %176
The unwind destination does not have an exception handling instruction!
  invoke void @_ZN3log17__private_api_log17h7b3a7ccf9969b414E(ptr noalias nocapture noundef nonnull readonly dereferenceable(48) %14, i64 noundef 2, ptr noalias noundef nonnull readonly align 8 dereferenceable(56) @anon.de822ab2b20f7769a577e3136c8d4650.120, ptr noalias noundef readonly align 8 null, i64 undef)
          to label %450 unwind label %176
The unwind destination does not have an exception handling instruction!
  %456 = invoke fastcc noundef nonnull ptr @_ZN6anyhow9__private10format_err17h1aedc5f595b3a414E(ptr noalias nocapture noundef nonnull readonly dereferenceable(48) %13)
          to label %457 unwind label %176
The unwind destination does not have an exception handling instruction!
  %469 = invoke i64 @_ZN4core4hash11BuildHasher8hash_one17h81d4add725a37804E(ptr noalias noundef nonnull readonly align 8 dereferenceable(16) %468, ptr noalias noundef nonnull readonly align 1 dereferenceable(16) %462)
          to label %470 unwind label %176
The unwind destination does not have an exception handling instruction!
  invoke void @"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$6remove17h7bbd3646bce49305E"(ptr noalias nocapture noundef nonnull sret(%307) dereferenceable(64) %38, ptr noalias noundef nonnull align 8 dereferenceable(48) %511, ptr noalias noundef nonnull readonly align 1 dereferenceable(16) %39)
          to label %512 unwind label %176
CleanupReturnInst must unwind to an EH block which is not a landingpad.
  cleanupret from %526 unwind label %176
The unwind destination does not have an exception handling instruction!
  invoke fastcc void @"_ZN4core3ptr65drop_in_place$LT$notify..windows..ReadDirectoryChangesWatcher$GT$17h9bd052b32e0afedcE"(ptr nonnull %521)
          to label %551 unwind label %176
The unwind destination does not have an exception handling instruction!
  %558 = invoke noundef nonnull align 8 ptr @"_ZN87_$LT$tokio..loom..std..atomic_usize..AtomicUsize$u20$as$u20$core..ops..deref..Deref$GT$5deref17hc0da01bd21d56c60E"(ptr noundef nonnull align 8 %557)
          to label %559 unwind label %176
The unwind destination does not have an exception handling instruction!
  invoke void @_ZN17app_name4scan15project_watcher14ProjectWatcher3new17h439fbb49bbbd4106E(ptr noalias nocapture noundef nonnull sret(%308) dereferenceable(64) %45, ptr noalias noundef nonnull readonly align 8 dereferenceable(8) %565, ptr noundef nonnull %556, ptr noalias nocapture noundef nonnull readonly dereferenceable(16) %44)
          to label %566 unwind label %176
The unwind destination does not have an exception handling instruction!
  invoke void @"_ZN9hashbrown3map28HashMap$LT$K$C$V$C$S$C$A$GT$6insert17hc40dd8c50b7da086E"(ptr noalias nocapture noundef nonnull sret(%307) dereferenceable(64) %43, ptr noalias noundef nonnull align 8 dereferenceable(48) %463, ptr noalias nocapture noundef nonnull dereferenceable(16) %10, ptr noalias nocapture noundef nonnull dereferenceable(64) %9)
          to label %654 unwind label %176
The unwind destination does not have an exception handling instruction!
  invoke fastcc void @"_ZN4core3ptr65drop_in_place$LT$notify..windows..ReadDirectoryChangesWatcher$GT$17h9bd052b32e0afedcE"(ptr nonnull %667)
          to label %668 unwind label %176
The unwind destination does not have an exception handling instruction!
  invoke void @_ZN3log17__private_api_log17h7b3a7ccf9969b414E(ptr noalias nocapture noundef nonnull readonly dereferenceable(48) %41, i64 noundef 4, ptr noalias noundef nonnull readonly align 8 dereferenceable(56) @anon.de822ab2b20f7769a577e3136c8d4650.129, ptr noalias noundef readonly align 8 null, i64 undef)
          to label %679 unwind label %176
The unwind destination does not have an exception handling instruction!
  invoke void @_ZN3log17__private_api_log17h7b3a7ccf9969b414E(ptr noalias nocapture noundef nonnull readonly dereferenceable(48) %32, i64 noundef 3, ptr noalias noundef nonnull readonly align 8 dereferenceable(56) @anon.de822ab2b20f7769a577e3136c8d4650.132, ptr noalias noundef readonly align 8 null, i64 undef)
          to label %688 unwind label %176
in function _ZN17app_name4scan7scanner17BackgroundScanner3new28_$u7b$$u7b$closure$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17hf77b037c79bcab15E.llvm.3981804495839767495
LLVM ERROR: Broken function found, compilation aborted!

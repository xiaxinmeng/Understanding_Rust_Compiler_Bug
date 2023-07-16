
(lldb) up
frame #2: 0x000007fec210fa00 rustc_driver-b05da10288c4445f.dll`public: class llvm::Error __cdecl llvm::sys::fs::TempFile::keep(Name=<unavailable>, Name=0x000000000269b260) __ptr64 at Path.cpp:1213
   1210 #ifdef _WIN32
   1211   // If we can't cancel the delete don't rename.
   1212   auto H = reinterpret_cast<HANDLE>(_get_osfhandle(FD));
-> 1213   std::error_code RenameEC = setDeleteDisposition(H, false);
   1214   if (!RenameEC) {
   1215     RenameEC = rename_fd(FD, Name);
   1216     // If rename failed because it's cross-device, copy instead
(lldb) vo
(const llvm::Twine &) Name = <register r8 is not available>

(const llvm::Twine &) Name = 0x000000000269b260
(std::error_code) RenameEC =
(void *) H = <register rax is not available>

(void *) H = 0x00000000000000a8
(std::error_code) EC = <extracting data from value failed>

(std::error_code) EC =
(std::error_code) EC =
(lldb) up
frame #3: 0x000007fec1e1eb0c rustc_driver-b05da10288c4445f.dll`class llvm::Error __cdecl llvm::writeArchive(ArcName=(Data = "p\aP", Length = 1), ArcName=<unavailable>, ArcName=<unavailable>, ArcName=<unavailable>, ArcName=<unavailable>, ArcName=<unavailable>, ArcName=<unavailable>, ArcName=<unavailable>, ArcName=<unavailable>, NewMembers=<unavailable>, NewMembers=<unavailable>, NewMembers=<unavailable>, NewMembers=<unavailable>, NewMembers=<unavailable>, WriteSymtab=true, WriteSymtab=<unavailable>, WriteSymtab=<unavailable>, WriteSymtab=<unavailable>, WriteSymtab=<unavailable>, WriteSymtab=<unavailable>, WriteSymtab=<unavailable>, WriteSymtab=<unavailable>, WriteSymtab=<unavailable>, WriteSymtab=<unavailable>,WriteSymtab=<unavailable>, Kind=K_GNU, Kind=<unavailable>, Kind=<unavailable>, Kind=<unavailable>, Kind=<unavailable>, Kind=<unavailable>, Kind=<unavailable>, Kind=<unavailable>, Kind=<unavailable>, Kind=<unavailable>, Kind=<unavailable>, Kind=K_GNU64, Kind=K_GNU, Deterministic=true, Deterministic=<unavailable>, Deterministic=<unavailable>, Deterministic=<unavailable>, Deterministic=<unavailable>, Deterministic=<unavailable>, Deterministic=<unavailable>, Deterministic=<unavailable>, Deterministic=<unavailable>, Deterministic=<unavailable>, Deterministic=true, Deterministic=true, Thin=<unavailable>, Thin=<unavailable>, Thin=<unavailable>, Thin=<unavailable>, Thin=<unavailable>, Thin=<unavailable>, Thin=<unavailable>, Thin=<unavailable>, Thin=<unavailable>, Thin=true, Thin=false, OldArchiveBuf=0x000000000269b570, OldArchiveBuf=0x000000000269b570, OldArchiveBuf=0x000000000269b570, OldArchiveBuf=0x000000000269b570, OldArchiveBuf=0x0000002000000101, OldArchiveBuf=0x000000000269b570) at ArchiveWriter.cpp:642
   639    // closed before we attempt to rename.
   640    OldArchiveBuf.reset();
   641
-> 642    return Temp->keep(ArcName);
   643  }
   644
   645  } // namespace llvm
(lldb) vo
(llvm::StringRef) ArcName =
(llvm::StringRef) ArcName = <register rdx is not available>

(llvm::StringRef) ArcName = <extracting data from value failed>

(llvm::StringRef) ArcName = <extracting data from value failed>

(llvm::StringRef) ArcName = <extracting data from value failed>

(llvm::StringRef) ArcName = <register rdx is not available>

(llvm::StringRef) ArcName = <extracting data from value failed>

(llvm::StringRef) ArcName = <extracting data from value failed>

(llvm::StringRef) ArcName = <extracting data from value failed>

(llvm::ArrayRef<llvm::NewArchiveMember>) NewMembers = <extracting data from value failed>

(llvm::ArrayRef<llvm::NewArchiveMember>) NewMembers = <extracting data from value failed>

(llvm::ArrayRef<llvm::NewArchiveMember>) NewMembers = <extracting data from value failed>

(llvm::ArrayRef<llvm::NewArchiveMember>) NewMembers = <extracting data from value failed>

(llvm::ArrayRef<llvm::NewArchiveMember>) NewMembers = <register r8 is not available>

(bool) WriteSymtab = true
(bool) WriteSymtab = <Unable to convert register kind=4 reg_num=69 to a native register number.>

(bool) WriteSymtab = <Unable to convert register kind=4 reg_num=36 to a native register number.>

(bool) WriteSymtab = <Unable to convert register kind=4 reg_num=36 to a native register number.>

(bool) WriteSymtab = <Unable to convert register kind=4 reg_num=36 to a native register number.>

(bool) WriteSymtab = <Unable to convert register kind=4 reg_num=36 to a native register number.>

(bool) WriteSymtab = <Unable to convert register kind=4 reg_num=36 to a native register number.>

(bool) WriteSymtab = <Unable to convert register kind=4 reg_num=36 to a native register number.>

(bool) WriteSymtab = <Unable to convert register kind=4 reg_num=36 to a native register number.>

(bool) WriteSymtab = <Unable to convert register kind=4 reg_num=36 to a native register number.>

(bool) WriteSymtab = <Unable to convert register kind=4 reg_num=36 to a native register number.>

(llvm::object::Archive::Kind) Kind = K_GNU
(llvm::object::Archive::Kind) Kind = <Unable to convert register kind=4 reg_num=38 to a native register number.>

(llvm::object::Archive::Kind) Kind = <Unable to convert register kind=4 reg_num=38 to a native register number.>

(llvm::object::Archive::Kind) Kind = <Unable to convert register kind=4 reg_num=38 to a native register number.>

(llvm::object::Archive::Kind) Kind = <Unable to convert register kind=4 reg_num=38 to a native register number.>

(llvm::object::Archive::Kind) Kind = <Unable to convert register kind=4 reg_num=38 to a native register number.>

(llvm::object::Archive::Kind) Kind = <Unable to convert register kind=4 reg_num=38 to a native register number.>

(llvm::object::Archive::Kind) Kind = <Unable to convert register kind=4 reg_num=38 to a native register number.>

(llvm::object::Archive::Kind) Kind = <Unable to convert register kind=4 reg_num=38 to a native register number.>

(llvm::object::Archive::Kind) Kind = <Unable to convert register kind=4 reg_num=38 to a native register number.>

(llvm::object::Archive::Kind) Kind = <Unable to convert register kind=4 reg_num=38 to a native register number.>

(llvm::object::Archive::Kind) Kind = K_GNU64
(llvm::object::Archive::Kind) Kind = K_GNU
(bool) Deterministic = true
(bool) Deterministic = <Unable to convert register kind=4 reg_num=29 to a native register number.>

(bool) Deterministic = <Unable to convert register kind=4 reg_num=29 to a native register number.>

(bool) Deterministic = <Unable to convert register kind=4 reg_num=29 to a native register number.>

(bool) Deterministic = <Unable to convert register kind=4 reg_num=29 to a native register number.>

(bool) Deterministic = <Unable to convert register kind=4 reg_num=29 to a native register number.>

(bool) Deterministic = <Unable to convert register kind=4 reg_num=29 to a native register number.>

(bool) Deterministic = <Unable to convert register kind=4 reg_num=29 to a native register number.>

(bool) Deterministic = <Unable to convert register kind=4 reg_num=29 to a native register number.>

(bool) Deterministic = <Unable to convert register kind=4 reg_num=29 to a native register number.>

(bool) Deterministic = true
(bool) Deterministic = true
(bool) Thin = <Unable to convert register kind=4 reg_num=39 to a native register number.>

(bool) Thin = <Unable to convert register kind=4 reg_num=39 to a native register number.>

(bool) Thin = <Unable to convert register kind=4 reg_num=39 to a native register number.>

(bool) Thin = <Unable to convert register kind=4 reg_num=39 to a native register number.>

(bool) Thin = <Unable to convert register kind=4 reg_num=39 to a native register number.>

(bool) Thin = <Unable to convert register kind=4 reg_num=39 to a native register number.>

(bool) Thin = <Unable to convert register kind=4 reg_num=39 to a native register number.>

(bool) Thin = <Unable to convert register kind=4 reg_num=39 to a native register number.>

(bool) Thin = <Unable to convert register kind=4 reg_num=39 to a native register number.>

(bool) Thin = true
(bool) Thin = false
(std::unique_ptr<llvm::MemoryBuffer,std::default_delete<llvm::MemoryBuffer> > *) OldArchiveBuf = 0x000000000269b570
(std::unique_ptr<llvm::MemoryBuffer,std::default_delete<llvm::MemoryBuffer> > *) OldArchiveBuf = 0x000000000269b570
(std::unique_ptr<llvm::MemoryBuffer,std::default_delete<llvm::MemoryBuffer> > *) OldArchiveBuf = 0x000000000269b570
(std::unique_ptr<llvm::MemoryBuffer,std::default_delete<llvm::MemoryBuffer> > *) OldArchiveBuf = 0x000000000269b570
(std::unique_ptr<llvm::MemoryBuffer,std::default_delete<llvm::MemoryBuffer> > *) OldArchiveBuf = 0x0000002000000101
(std::unique_ptr<llvm::MemoryBuffer,std::default_delete<llvm::MemoryBuffer> > *) OldArchiveBuf = 0x000000000269b570
(llvm::SmallString<0>) SymNamesBuf = <register rcx is not available>

(llvm::SmallString<0>) SymNamesBuf = <register rcx is not available>

(llvm::SmallString<0>) SymNamesBuf = <register rcx is not available>

(llvm::SmallString<0>) SymNamesBuf = <register rcx is not available>

(llvm::SmallString<0>) SymNamesBuf =
(llvm::SmallString<0>) StringTableBuf = <register rcx is not available>

(llvm::SmallString<0>) StringTableBuf = <register rcx is not available>

(llvm::SmallString<0>) StringTableBuf = <register rcx is not available>

(llvm::SmallString<0>) StringTableBuf = <register rcx is not available>

(llvm::SmallString<0>) StringTableBuf = <register rcx is not available>

(llvm::SmallString<0>) StringTableBuf = <register rcx is not available>

(llvm::SmallString<0>) StringTableBuf = <register rcx is not available>

(llvm::SmallString<0>) StringTableBuf = <register rcx is not available>

(llvm::SmallString<0>) StringTableBuf =
(llvm::raw_fd_ostream) Out = <register rcx is not available>

(llvm::raw_fd_ostream) Out = <register rcx is not available>

(llvm::raw_fd_ostream) Out = <register rcx is not available>

(llvm::raw_fd_ostream) Out =
(llvm::raw_svector_ostream) SymNames =
(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <register rcx is not available>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <register r10 is not available>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <register r10 is not available>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <Unable to convert register kind=4 reg_num=60 to a native register number.>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <Unable to convert register kind=4 reg_num=60 to a native register number.>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <Unable to convert register kind=4 reg_num=24 to a native register number.>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <Unable to convert register kind=4 reg_num=24 to a native register number.>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <Unable to convert register kind=4 reg_num=24 to a native register number.>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <Unable to convert register kind=4 reg_num=24 to a native register number.>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <Unable to convert register kind=4 reg_num=24 to a native register number.>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <Unable to convert register kind=4 reg_num=69 to a native register number.>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <Unable to convert register kind=4 reg_num=71 to a native register number.>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <Unable to convert register kind=4 reg_num=71 to a native register number.>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <Unable to convert register kind=4 reg_num=71 to a native register number.>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <Unable to convert register kind=4 reg_num=71 to a native register number.>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <Unable to convert register kind=4 reg_num=71 to a native register number.>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <Unable to convert register kind=4 reg_num=33 to a native register number.>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <register rcx is not available>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <register rcx is not available>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <register r10 is not available>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <register r10 is not available>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <register r10 is not available>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <register r10 is not available>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <register r10 is not available>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <register r10 is not available>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <register r10 is not available>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <register r10 is not available>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <Unable to convert register kind=4 reg_num=60 to a native register number.>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <Unable to convert register kind=4 reg_num=60 to a native register number.>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <Unable to convert register kind=4 reg_num=24 to a native register number.>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <Unable to convert register kind=4 reg_num=24 to a native register number.>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <Unable to convert register kind=4 reg_num=71 to a native register number.>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <Unable to convert register kind=4 reg_num=71 to a native register number.>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <Unable to convert register kind=4 reg_num=71 to a native register number.>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp = <Unable to convert register kind=4 reg_num=71 to a native register number.>

(llvm::Expected<llvm::sys::fs::TempFile>) Temp =
(llvm::raw_svector_ostream) StringTable =
(llvm::Expected<std::vector<`anonymous namespace'::MemberData,std::allocator<`anonymous namespace'::MemberData> > >) DataOrErr = <register rcx is not available>

(llvm::Expected<std::vector<`anonymous namespace'::MemberData,std::allocator<`anonymous namespace'::MemberData> > >) DataOrErr = <register rcx is not available>

(llvm::Expected<std::vector<`anonymous namespace'::MemberData,std::allocator<`anonymous namespace'::MemberData> > >) DataOrErr = <register rcx is not available>

(llvm::Expected<std::vector<`anonymous namespace'::MemberData,std::allocator<`anonymous namespace'::MemberData> > >) DataOrErr = <register rcx is not available>

(llvm::Expected<std::vector<`anonymous namespace'::MemberData,std::allocator<`anonymous namespace'::MemberData> > >) DataOrErr = <register rcx is not available>

(llvm::Expected<std::vector<`anonymous namespace'::MemberData,std::allocator<`anonymous namespace'::MemberData> > >) DataOrErr = <register rcx is not available>

(llvm::Expected<std::vector<`anonymous namespace'::MemberData,std::allocator<`anonymous namespace'::MemberData> > >) DataOrErr = <extracting data from value failed>

(llvm::Expected<std::vector<`anonymous namespace'::MemberData,std::allocator<`anonymous namespace'::MemberData> > >) DataOrErr = <Unable to convert register kind=4 reg_num=63 to a native register number.>

(llvm::Expected<std::vector<`anonymous namespace'::MemberData,std::allocator<`anonymous namespace'::MemberData> > >) DataOrErr = <Unable to convert register kind=4 reg_num=24 to a native register number.>

(llvm::Expected<std::vector<`anonymous namespace'::MemberData,std::allocator<`anonymous namespace'::MemberData> > >) DataOrErr = <Unable to convert register kind=4 reg_num=24 to a native register number.>

(llvm::Expected<std::vector<`anonymous namespace'::MemberData,std::allocator<`anonymous namespace'::MemberData> > >) DataOrErr = <Unable to convert register kind=4 reg_num=24 to a native register number.>

(llvm::Expected<std::vector<`anonymous namespace'::MemberData,std::allocator<`anonymous namespace'::MemberData> > >) DataOrErr = <Unable to convert register kind=4 reg_num=24 to a native register number.>

(llvm::Expected<std::vector<`anonymous namespace'::MemberData,std::allocator<`anonymous namespace'::MemberData> > >) DataOrErr = <Unable to convert register kind=4 reg_num=24 to a native register number.>

(llvm::Expected<std::vector<`anonymous namespace'::MemberData,std::allocator<`anonymous namespace'::MemberData> > >) DataOrErr = <Unable to convert register kind=4 reg_num=27 to a native register number.>

(llvm::Expected<std::vector<`anonymous namespace'::MemberData,std::allocator<`anonymous namespace'::MemberData> > >) DataOrErr = <register rcx is not available>

(llvm::Expected<std::vector<`anonymous namespace'::MemberData,std::allocator<`anonymous namespace'::MemberData> > >) DataOrErr = <register rcx is not available>

(llvm::Expected<std::vector<`anonymous namespace'::MemberData,std::allocator<`anonymous namespace'::MemberData> > >) DataOrErr = <extracting data from value failed>

(llvm::Expected<std::vector<`anonymous namespace'::MemberData,std::allocator<`anonymous namespace'::MemberData> > >) DataOrErr = <extracting data from value failed>

(llvm::Expected<std::vector<`anonymous namespace'::MemberData,std::allocator<`anonymous namespace'::MemberData> > >) DataOrErr = <Unable to convert register kind=4 reg_num=24 to a native register number.>

(llvm::Expected<std::vector<`anonymous namespace'::MemberData,std::allocator<`anonymous namespace'::MemberData> > >) DataOrErr = <Unable to convert register kind=4 reg_num=24 to a native register number.>

(llvm::Expected<std::vector<`anonymous namespace'::MemberData,std::allocator<`anonymous namespace'::MemberData> > >) DataOrErr = <Unable to convert register kind=4 reg_num=24 to a native register number.>

(llvm::Expected<std::vector<`anonymous namespace'::MemberData,std::allocator<`anonymous namespace'::MemberData> > >) DataOrErr =(llvm::Error) E = <Unable to convert register kind=4 reg_num=24 to a native register number.
>

(llvm::Error) E = <Unable to convert register kind=4 reg_num=24 to a native register number.>

(llvm::Error) E = <Unable to convert register kind=4 reg_num=24 to a native register number.>

(llvm::Error) E = <Unable to convert register kind=4 reg_num=24 to a native register number.>

(llvm::Error) E =
(llvm::Error) E =
(llvm::Error) E =
(unsigned long long) LastOffset = 40482232
(unsigned long long) LastOffset = 40482232
(unsigned long long) LastOffset = 40482232
(unsigned long long) LastOffset = 40482232
(const char *) Sym64Env = <register rax is not available>

(const char *) Sym64Env = <register rax is not available>

(int) Sym64Threshold = <Unable to convert register kind=4 reg_num=28 to a native register number.>

(int) Sym64Threshold = <Unable to convert register kind=4 reg_num=28 to a native register number.>

(unsigned long long) MaxOffset = <register rcx is not available>

(unsigned long long) MaxOffset = <register rdx is not available>

(unsigned long long) MaxOffset = <register rdx is not available>

((anonymous namespace)::MemberData *) <end>$L1 = 0x0000000000540ca8
((anonymous namespace)::MemberData *) <end>$L1 = 0x0000000000540ca8
((anonymous namespace)::MemberData *) <end>$L1 = 0x0000000000540ca8
((anonymous namespace)::MemberData *) <end>$L1 = 0x0000000000540ca8

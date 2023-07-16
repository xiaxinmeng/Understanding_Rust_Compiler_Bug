
#0  0x00007ffff7132274 in raise () from /nix/store/fysbl29a8p8sa9z3wpnqpn56a0b54fap-glibc-2.26-75/lib/libc.so.6
#1  0x00007ffff7133675 in abort () from /nix/store/fysbl29a8p8sa9z3wpnqpn56a0b54fap-glibc-2.26-75/lib/libc.so.6
#2  0x00007ffff712acf7 in __assert_fail_base () from /nix/store/fysbl29a8p8sa9z3wpnqpn56a0b54fap-glibc-2.26-75/lib/libc.so.6
#3  0x00007ffff712ada2 in __assert_fail () from /nix/store/fysbl29a8p8sa9z3wpnqpn56a0b54fap-glibc-2.26-75/lib/libc.so.6
#4  0x00007fffeee5ce8a in llvm::SmallVectorBase::grow_pod(void*, unsigned long, unsigned long) ()
   from /home/eddy/Projects/rust-2/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/../lib/librustc_llvm-2d59d9b96ed97eac.so
#5  0x00007fffee943e4a in void llvm::BitstreamWriter::EmitAbbreviatedField<unsigned long>(llvm::BitCodeAbbrevOp const&, unsigned long) ()
   from /home/eddy/Projects/rust-2/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/../lib/librustc_llvm-2d59d9b96ed97eac.so
#6  0x00007fffee94389c in void llvm::BitstreamWriter::EmitRecordWithAbbrevImpl<unsigned long>(unsigned int, llvm::ArrayRef<unsigned long>, llvm::StringRef, llvm::Optional<unsigned int>)
    () from /home/eddy/Projects/rust-2/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/../lib/librustc_llvm-2d59d9b96ed97eac.so
#7  0x00007fffee9434d9 in void llvm::BitstreamWriter::EmitRecord<llvm::SmallVector<unsigned long, 64u> >(unsigned int, llvm::SmallVector<unsigned long, 64u> const&, unsigned int) ()
   from /home/eddy/Projects/rust-2/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/../lib/librustc_llvm-2d59d9b96ed97eac.so
#8  0x00007fffee9423b8 in (anonymous namespace)::ModuleBitcodeWriter::writeValueSymbolTable(llvm::ValueSymbolTable const&, bool, llvm::DenseMap<llvm::Function const*, unsigned long, llvm::DenseMapInfo<llvm::Function const*>, llvm::detail::DenseMapPair<llvm::Function const*, unsigned long> >*) ()
   from /home/eddy/Projects/rust-2/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/../lib/librustc_llvm-2d59d9b96ed97eac.so
#9  0x00007fffee935ce9 in (anonymous namespace)::ModuleBitcodeWriter::write() ()
   from /home/eddy/Projects/rust-2/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/../lib/librustc_llvm-2d59d9b96ed97eac.so
#10 0x00007fffee92c968 in llvm::BitcodeWriter::writeModule(llvm::Module const*, bool, llvm::ModuleSummaryIndex const*, bool) ()
   from /home/eddy/Projects/rust-2/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/../lib/librustc_llvm-2d59d9b96ed97eac.so
#11 0x00007fffee939291 in llvm::WriteBitcodeToFile(llvm::Module const*, llvm::raw_ostream&, bool, llvm::ModuleSummaryIndex const*, bool) ()
   from /home/eddy/Projects/rust-2/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/../lib/librustc_llvm-2d59d9b96ed97eac.so
#12 0x00007fffedbb02aa in (anonymous namespace)::WriteThinLTOBitcode::runOnModule(llvm::Module&) ()
   from /home/eddy/Projects/rust-2/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/../lib/librustc_llvm-2d59d9b96ed97eac.so
#13 0x00007fffeecace08 in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
   from /home/eddy/Projects/rust-2/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/../lib/librustc_llvm-2d59d9b96ed97eac.so
#14 0x00007fffed515803 in LLVMRustThinLTOBufferCreate () from /home/eddy/Projects/rust-2/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/../lib/librustc_llvm-2d59d9b96ed97eac.so
#15 0x00007ffff4f2766c in rustc_trans::back::lto::ThinBuffer::new () at src/librustc_trans/back/lto.rs:555
#16 rustc_trans::back::write::codegen () at src/librustc_trans/back/write.rs:659
#17 0x00007ffff4f195e9 in rustc_trans::back::write::execute_work_item () at src/librustc_trans/back/write.rs:1303


    frame #0: 0x00007ffba90ebe44 ntdll.dll`NtWaitForSingleObject + 20
    frame #1: 0x00007ffba6b826ee KernelBase.dll`WaitForSingleObjectEx + 142
    frame #2: 0x000000006494209c libwinpthread-1.dll`pthread_cond_init + 556
    frame #3: 0x00000000649421de libwinpthread-1.dll`pthread_cond_init + 878
    frame #4: 0x000000006494246f libwinpthread-1.dll`pthread_cond_signal + 207
    frame #5: 0x00000000029a5119 rust-lld.exe`std::condition_variable::notify_one() + 9
    frame #6: 0x000000000218854c rust-lld.exe`llvm::parallel::detail::TaskGroup::spawn(std::function<void ()>) + 412
    frame #7: 0x0000000000ffd62b rust-lld.exe`(anonymous namespace)::Writer::run() + 5499
    frame #8: 0x000000000119e465 rust-lld.exe`lld::coff::writeResult() + 837
    frame #9: 0x00000000011aa4b4 rust-lld.exe`lld::coff::LinkerDriver::link(llvm::ArrayRef<char const*>) + 30628
    frame #10: 0x00000000011c99b3 rust-lld.exe`lld::coff::link(llvm::ArrayRef<char const*>, bool, llvm::raw_ostream&, ll
vm::raw_ostream&) + 243
    frame #11: 0x000000000122ba1e rust-lld.exe`lld::mingw::link(llvm::ArrayRef<char const*>, bool, llvm::raw_ostream&, l
lvm::raw_ostream&) + 12894
    frame #12: 0x0000000002c4143b rust-lld.exe`main + 1243
    frame #13: 0x00000000004013f8 rust-lld.exe`__tmainCRTStartup + 584
    frame #14: 0x000000000040151b rust-lld.exe
    frame #15: 0x00007ffba82f6fd4 kernel32.dll`BaseThreadInitThunk + 20
    frame #16: 0x00007ffba909cec1 ntdll.dll`RtlUserThreadStart + 33

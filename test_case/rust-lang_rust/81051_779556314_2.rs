
frame #0: 0x000007fec2110ec5 rustc_driver-b05da10288c4445f.dll`static class std::error_code llvm::sys::fs::realPathFromHandle(H=0x00000000000000a8, H=0x00000000000000a8, H=0x00000000000000a8, Buffer=0x000000000269b030, Buffer=0x000000000269b030, Buffer=0x000000000269b030) at Path.inc:353
   350
   351  static std::error_code realPathFromHandle(HANDLE H,
   352                                            SmallVectorImpl<wchar_t> &Buffer) {
-> 353    DWORD CountChars = ::GetFinalPathNameByHandleW(
   354        H, Buffer.begin(), Buffer.capacity(), FILE_NAME_NORMALIZED);
   355    if (CountChars && CountChars >= Buffer.capacity()) {
   356      // The buffer wasn't big enough, try again.  In this case the return value
(lldb) vo
(void *) H = 0x00000000000000a8
(void *) H = 0x00000000000000a8
(void *) H = 0x00000000000000a8
(llvm::SmallVectorImpl<wchar_t> &) Buffer = 0x000000000269b030
(llvm::SmallVectorImpl<wchar_t> &) Buffer = 0x000000000269b030
(llvm::SmallVectorImpl<wchar_t> &) Buffer = 0x000000000269b030
(unsigned long) CountChars = <Unable to convert register kind=4 reg_num=24 to a native register number.>

(unsigned long) CountChars = <Unable to convert register kind=4 reg_num=24 to a native register number.>

(unsigned long) CountChars = <Unable to convert register kind=4 reg_num=24 to a native register number.>

(unsigned long) CountChars = <Unable to convert register kind=4 reg_num=24 to a native register number.>

(unsigned long) CountChars = <Unable to convert register kind=4 reg_num=24 to a native register number.>

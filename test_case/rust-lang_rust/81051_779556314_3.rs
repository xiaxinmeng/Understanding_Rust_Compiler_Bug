
(lldb) up
frame #1: 0x000007fec21130f7 rustc_driver-b05da10288c4445f.dll`static class std::error_code llvm::sys::fs::setDeleteDisposition(Handle=<unavailable>, Handle=0x00000000000000a8, Delete=<unavailable>, Delete=<unavailable>) at Path.inc:408
   405    // First, check if the file is on a network (non-local) drive. If so, don't
   406    // set DeleteFile to true, since it prevents opening the file for writes.
   407    SmallVector<wchar_t, 128> FinalPath;
-> 408    if (std::error_code EC = realPathFromHandle(Handle, FinalPath))
   409      return EC;
   410
   411    bool IsLocal;
(lldb) vo
(void *) Handle = <register rdx is not available>

(void *) Handle = 0x00000000000000a8
(bool) Delete = <Unable to convert register kind=4 reg_num=29 to a native register number.>

(bool) Delete = <Unable to convert register kind=4 reg_num=68 to a native register number.>

(bool) IsLocal = false
(_FILE_DISPOSITION_INFO) Disposition =
(llvm::SmallVector<wchar_t,128>) FinalPath = <register rcx is not available>

(llvm::SmallVector<wchar_t,128>) FinalPath = <register rcx is not available>

(llvm::SmallVector<wchar_t,128>) FinalPath =
(std::error_code) EC =
(std::error_code) EC =
(std::error_code) EC =


$ fgrep 'std::system_category()' -r . --include '*.h' --include '*.inc' --include '*.cpp'
./lib/Support/ErrorHandling.cpp:    return std::error_code(EV, std::system_category());
./lib/Support/LockFileManager.cpp:    return std::error_code(errno, std::system_category());
./lib/Support/Path.cpp:      std::error_code(ERROR_NOT_SAME_DEVICE, std::system_category())) {
./lib/Support/RandomNumberGenerator.cpp:  return std::error_code(GetLastError(), std::system_category());
./lib/Support/RandomNumberGenerator.cpp:      Ret = std::error_code(errno, std::system_category());
./lib/Support/RandomNumberGenerator.cpp:      Ret = std::error_code(EIO, std::system_category());
./lib/Support/RandomNumberGenerator.cpp:      Ret = std::error_code(errno, std::system_category());
./lib/Support/RandomNumberGenerator.cpp:  return std::error_code(errno, std::system_category());
./lib/Support/VirtualFileSystem.cpp:                         std::system_category());
./lib/Support/Windows/Path.inc:        std::error_code(ERROR_CALL_NOT_IMPLEMENTED, std::system_category())) {

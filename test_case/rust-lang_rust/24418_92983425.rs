
   Compiling gcc v0.3.4
   Compiling libc v0.1.6
   Compiling time v0.1.24
failed to run custom build command for `time v0.1.24`
Process didn't exit successfully: `D:\Users\amader\Projekte\test\target\debug\build\time-15ec407cfcd2d643\build-script-build` (exit code: 101)
--- stdout
TARGET = Some("x86_64-pc-windows-gnu")
CARGO_MANIFEST_DIR = Some("D:\\Users\\amader\\.cargo\\registry\\src\\github.com-1ecc6299db9ec823\\time-0.1.24")
OUT_DIR = Some("D:\\Users\\amader\\Projekte\\test\\target\\debug\\build\\time-15ec407cfcd2d643\\out")
TARGET = Some("x86_64-pc-windows-gnu")
OPT_LEVEL = Some("0")
PROFILE = Some("debug")
debug 0
TARGET = Some("x86_64-pc-windows-gnu")
HOST = Some("x86_64-pc-windows-gnu")
CC_x86_64-pc-windows-gnu = None
CC_x86_64_pc_windows_gnu = None
HOST_CC = None
CC = None
TARGET = Some("x86_64-pc-windows-gnu")
HOST = Some("x86_64-pc-windows-gnu")
CFLAGS_x86_64-pc-windows-gnu = None
CFLAGS_x86_64_pc_windows_gnu = None
HOST_CFLAGS = None
CFLAGS = None
TARGET = Some("x86_64-pc-windows-gnu")
HOST = Some("x86_64-pc-windows-gnu")
CC_x86_64-pc-windows-gnu = None
CC_x86_64_pc_windows_gnu = None
HOST_CC = None
CC = None
running: "gcc" "-O0" "-c" "-ffunction-sections" "-fdata-sections" "-mwin32" "-m64" "-fPIC" "D:\Users\amader\.cargo\registry\src\github.com-1ecc6299db9ec823\time-0.1.24\src/time_helpers.c" "-o" "D:\Users\amader\Projekte\test\target\debug\build\time-15ec407cfcd2d643\out\src\time_helpers.o"


command did not execute successfully, got: exit code: 1



--- stderr
thread '<main>' panicked at 'explicit panic', D:\Users\amader\.cargo\registry\src\github.com-1ecc6299db9ec823\gcc-0.3.4\src\lib.rs:380


2020-03-11T22:40:47.7996438Z error: failed to run custom build command for `compositor-windows v0.1.0 (D:\a\1\s\build\ct\webrender\example-compositor\compositor-windows)`
2020-03-11T22:40:47.7997526Z 
2020-03-11T22:40:47.7997853Z Caused by:
2020-03-11T22:40:47.7998491Z   process didn't exit successfully: `D:\a\1\s\build\ct\webrender\target\debug\build\compositor-windows-a42f9fe4a6ab9aa2\build-script-build` (exit code: 101)
2020-03-11T22:40:47.7999145Z --- stdout
2020-03-11T22:40:47.7999502Z TARGET = Some("x86_64-pc-windows-msvc")
2020-03-11T22:40:47.7999847Z OPT_LEVEL = Some("0")
2020-03-11T22:40:47.8000230Z TARGET = Some("x86_64-pc-windows-msvc")
2020-03-11T22:40:47.8000607Z HOST = Some("x86_64-pc-windows-msvc")
2020-03-11T22:40:47.8001023Z TARGET = Some("x86_64-pc-windows-msvc")
2020-03-11T22:40:47.8001472Z TARGET = Some("x86_64-pc-windows-msvc")
2020-03-11T22:40:47.8001845Z HOST = Some("x86_64-pc-windows-msvc")
2020-03-11T22:40:47.8002234Z CC_x86_64-pc-windows-msvc = None
2020-03-11T22:40:47.8002580Z CC_x86_64_pc_windows_msvc = None
2020-03-11T22:40:47.8002996Z HOST_CC = None
2020-03-11T22:40:47.8003256Z CC = None
2020-03-11T22:40:47.8003510Z TARGET = Some("x86_64-pc-windows-msvc")
2020-03-11T22:40:47.8003847Z HOST = Some("x86_64-pc-windows-msvc")
2020-03-11T22:40:47.8004478Z CFLAGS_x86_64-pc-windows-msvc = None
2020-03-11T22:40:47.8004863Z CFLAGS_x86_64_pc_windows_msvc = None
2020-03-11T22:40:47.8005128Z HOST_CFLAGS = None
2020-03-11T22:40:47.8005401Z CFLAGS = None
2020-03-11T22:40:47.8005670Z DEBUG = Some("true")
2020-03-11T22:40:47.8006518Z running: "C:\\Program Files (x86)\\Microsoft Visual Studio\\2017\\Enterprise\\VC\\Tools\\MSVC\\14.16.27023\\bin\\HostX64\\x64\\cl.exe" "/nologo" "/MD" "/Z7" "/I" "../../../angle/checkout/include" "/W4" "/FoD:\\a\\1\\s\\build\\ct\\webrender\\target\\debug\\build\\compositor-windows-4a6bb37668b71e84\\out\\src/lib.o" "/c" "src/lib.cpp"
2020-03-11T22:40:47.8007284Z lib.cpp
2020-03-11T22:40:47.8007702Z src/lib.cpp(20): fatal error C1083: Cannot open include file: 'EGL/egl.h': No such file or directory
2020-03-11T22:40:47.8008086Z exit code: 2

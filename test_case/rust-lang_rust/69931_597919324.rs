plain
2020-03-11T22:37:34.2789689Z Initialized empty Git repository in C:/MORE_SPACE/ct/webrender/.git/
2020-03-11T22:37:34.3055834Z fatal: Could not parse object '6f23331299bf47e7e4683b815d10320770e14e21'.
2020-03-11T22:37:37.9108175Z From https://github.com/servo/webrender
2020-03-11T22:37:37.9108541Z  * branch            master     -> FETCH_HEAD
2020-03-11T22:37:38.6676379Z HEAD is now at 6f23331 Auto merge of #3881 - moz-gfx:github-sync, r=auto
2020-03-11T22:37:38.7460786Z     Updating git repository `https://github.com/rust-windowing/android-rs-glue.git`
---
2020-03-11T22:38:24.8391902Z    Compiling procedural-masquerade v0.1.6
2020-03-11T22:38:25.0441265Z    Compiling dtoa v0.4.2
2020-03-11T22:38:25.2734302Z    Compiling itoa v0.4.1
2020-03-11T22:38:25.4982941Z    Compiling percent-encoding v2.1.0
2020-03-11T22:38:25.9847610Z    Compiling bytemuck v1.2.0
2020-03-11T22:38:26.2551846Z    Compiling binary-space-partition v0.1.2
2020-03-11T22:38:26.3652470Z    Compiling httparse v1.2.4
2020-03-11T22:38:26.8188453Z    Compiling svg_fmt v0.4.0
2020-03-11T22:38:27.3279425Z    Compiling semver-parser v0.7.0
---
2020-03-11T22:40:41.5599724Z    Compiling ws v0.9.0
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
2020-03-11T22:40:47.8008321Z 
2020-03-11T22:40:47.8008521Z --- stderr
2020-03-11T22:40:47.8008805Z thread 'main' panicked at '
2020-03-11T22:40:47.8009012Z 
2020-03-11T22:40:47.8009012Z 
2020-03-11T22:40:47.8009940Z Internal error occurred: Command "C:\\Program Files (x86)\\Microsoft Visual Studio\\2017\\Enterprise\\VC\\Tools\\MSVC\\14.16.27023\\bin\\HostX64\\x64\\cl.exe" "/nologo" "/MD" "/Z7" "/I" "../../../angle/checkout/include" "/W4" "/FoD:\\a\\1\\s\\build\\ct\\webrender\\target\\debug\\build\\compositor-windows-4a6bb37668b71e84\\out\\src/lib.o" "/c" "src/lib.cpp" with args "cl.exe" did not execute successfully (status code exit code: 2).
2020-03-11T22:40:47.8011238Z ', C:\Users\VssAdministrator\.cargo\registry\src\github.com-1ecc6299db9ec823\cc-1.0.15\src\lib.rs:2158:5
2020-03-11T22:40:47.8011804Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-11T22:40:47.8012085Z 
2020-03-11T22:40:47.8012498Z warning: build failed, waiting for other jobs to finish...
---
2020-03-11T22:51:15.0667182Z   local time: Wed Mar 11 22:51:14 CUT 2020
2020-03-11T22:51:15.2274541Z   network time: Wed, 11 Mar 2020 22:51:15 GMT
2020-03-11T22:51:15.2307681Z == end clock drift check ==
2020-03-11T22:51:15.3434840Z 
2020-03-11T22:51:15.6847448Z ##[error]Bash exited with code '1'.
2020-03-11T22:51:15.7954194Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-11T22:51:15.8917300Z ==============================================================================
2020-03-11T22:51:15.8917753Z Task         : Get sources
2020-03-11T22:51:15.8918129Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

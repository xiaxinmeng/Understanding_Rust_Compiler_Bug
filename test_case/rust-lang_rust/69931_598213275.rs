plain
2020-03-12T14:02:58.5116988Z Initialized empty Git repository in C:/MORE_SPACE/ct/webrender/.git/
2020-03-12T14:02:58.5374460Z fatal: Could not parse object '6f23331299bf47e7e4683b815d10320770e14e21'.
2020-03-12T14:03:00.0720501Z From https://github.com/servo/webrender
2020-03-12T14:03:00.0721421Z  * branch            master     -> FETCH_HEAD
2020-03-12T14:03:00.6951113Z HEAD is now at 6f23331 Auto merge of #3881 - moz-gfx:github-sync, r=auto
2020-03-12T14:03:00.7892035Z     Updating git repository `https://github.com/rust-windowing/android-rs-glue.git`
---
2020-03-12T14:03:51.3530715Z    Compiling procedural-masquerade v0.1.6
2020-03-12T14:03:51.3531035Z    Compiling percent-encoding v2.1.0
2020-03-12T14:03:51.3531318Z    Compiling dtoa v0.4.2
2020-03-12T14:03:51.3531528Z    Compiling itoa v0.4.1
2020-03-12T14:03:51.3531836Z    Compiling bytemuck v1.2.0
2020-03-12T14:03:51.3532347Z    Compiling binary-space-partition v0.1.2
2020-03-12T14:03:51.3532640Z    Compiling lazy_static v0.2.11
2020-03-12T14:03:51.3532866Z    Compiling svg_fmt v0.4.0
2020-03-12T14:03:51.3533395Z    Compiling semver-parser v0.7.0
---
2020-03-12T14:06:37.0034476Z    Compiling ws v0.9.0
2020-03-12T14:06:41.4892254Z error: failed to run custom build command for `compositor-windows v0.1.0 (D:\a\1\s\build\ct\webrender\example-compositor\compositor-windows)`
2020-03-12T14:06:41.4893223Z 
2020-03-12T14:06:41.4935663Z Caused by:
2020-03-12T14:06:41.4936579Z   process didn't exit successfully: `D:\a\1\s\build\ct\webrender\target\debug\build\compositor-windows-734cecd79004d476\build-script-build` (exit code: 101)
2020-03-12T14:06:41.4937301Z --- stdout
2020-03-12T14:06:41.4937630Z TARGET = Some("x86_64-pc-windows-msvc")
2020-03-12T14:06:41.5170057Z OPT_LEVEL = Some("0")
2020-03-12T14:06:41.5481007Z TARGET = Some("x86_64-pc-windows-msvc")
2020-03-12T14:06:41.5481592Z HOST = Some("x86_64-pc-windows-msvc")
2020-03-12T14:06:41.5482364Z TARGET = Some("x86_64-pc-windows-msvc")
2020-03-12T14:06:41.5482752Z TARGET = Some("x86_64-pc-windows-msvc")
2020-03-12T14:06:41.5483200Z HOST = Some("x86_64-pc-windows-msvc")
2020-03-12T14:06:41.5483943Z CC_x86_64-pc-windows-msvc = None
2020-03-12T14:06:41.5484499Z CC_x86_64_pc_windows_msvc = None
2020-03-12T14:06:41.5484857Z HOST_CC = None
2020-03-12T14:06:41.5485119Z CC = None
2020-03-12T14:06:41.5485483Z TARGET = Some("x86_64-pc-windows-msvc")
2020-03-12T14:06:41.5485888Z HOST = Some("x86_64-pc-windows-msvc")
2020-03-12T14:06:41.5486248Z CFLAGS_x86_64-pc-windows-msvc = None
2020-03-12T14:06:41.5486652Z CFLAGS_x86_64_pc_windows_msvc = None
2020-03-12T14:06:41.5486973Z HOST_CFLAGS = None
2020-03-12T14:06:41.5487303Z CFLAGS = None
2020-03-12T14:06:41.5487637Z DEBUG = Some("true")
2020-03-12T14:06:41.5488681Z running: "C:\\Program Files (x86)\\Microsoft Visual Studio\\2017\\Enterprise\\VC\\Tools\\MSVC\\14.16.27023\\bin\\HostX64\\x64\\cl.exe" "/nologo" "/MD" "/Z7" "/I" "../../../angle/checkout/include" "/W4" "/FoD:\\a\\1\\s\\build\\ct\\webrender\\target\\debug\\build\\compositor-windows-44acd910b2bf945c\\out\\src/lib.o" "/c" "src/lib.cpp"
2020-03-12T14:06:41.5489664Z lib.cpp
2020-03-12T14:06:41.5490165Z src/lib.cpp(20): fatal error C1083: Cannot open include file: 'EGL/egl.h': No such file or directory
2020-03-12T14:06:41.5490914Z 
2020-03-12T14:06:41.5491152Z --- stderr
2020-03-12T14:06:41.5491488Z thread 'main' panicked at '
2020-03-12T14:06:41.5491732Z 
2020-03-12T14:06:41.5491732Z 
2020-03-12T14:06:41.5492896Z Internal error occurred: Command "C:\\Program Files (x86)\\Microsoft Visual Studio\\2017\\Enterprise\\VC\\Tools\\MSVC\\14.16.27023\\bin\\HostX64\\x64\\cl.exe" "/nologo" "/MD" "/Z7" "/I" "../../../angle/checkout/include" "/W4" "/FoD:\\a\\1\\s\\build\\ct\\webrender\\target\\debug\\build\\compositor-windows-44acd910b2bf945c\\out\\src/lib.o" "/c" "src/lib.cpp" with args "cl.exe" did not execute successfully (status code exit code: 2).
2020-03-12T14:06:41.5494492Z ', C:\Users\VssAdministrator\.cargo\registry\src\github.com-1ecc6299db9ec823\cc-1.0.15\src\lib.rs:2158:5
2020-03-12T14:06:41.5495163Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-12T14:06:41.5495502Z 
2020-03-12T14:06:41.5495893Z warning: build failed, waiting for other jobs to finish...
---
2020-03-12T14:19:58.8455391Z   local time: Thu Mar 12 14:19:58 CUT 2020
2020-03-12T14:19:59.2723644Z   network time: Thu, 12 Mar 2020 14:19:59 GMT
2020-03-12T14:19:59.2771705Z == end clock drift check ==
2020-03-12T14:19:59.3681874Z 
2020-03-12T14:19:59.6529919Z ##[error]Bash exited with code '1'.
2020-03-12T14:19:59.7090856Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-12T14:19:59.9020081Z ==============================================================================
2020-03-12T14:19:59.9020462Z Task         : Get sources
2020-03-12T14:19:59.9024683Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

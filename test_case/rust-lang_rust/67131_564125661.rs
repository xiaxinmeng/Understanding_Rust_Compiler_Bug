plain
2019-12-10T13:46:28.2841007Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-10T13:46:28.2841834Z 
2019-12-10T13:46:28.2842061Z   git checkout -b <new-branch-name>
2019-12-10T13:46:28.2842259Z 
2019-12-10T13:46:28.2842488Z HEAD is now at 0ecb374d1 Auto merge of #67131 - Centril:item-merge, r=petrochenkov
2019-12-10T13:46:28.3272478Z ##[section]Starting: Setup environment
2019-12-10T13:46:28.3401849Z ==============================================================================
2019-12-10T13:46:28.3401943Z Task         : Bash
2019-12-10T13:46:28.3402036Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-10T13:46:30.1641438Z 
2019-12-10T13:46:30.1641469Z 
2019-12-10T13:46:30.1641503Z 
2019-12-10T13:46:30.1641555Z 
2019-12-10T13:46:30.1641637Z       - `default`ness on `trait` items,
2019-12-10T13:46:30.1641753Z       - `impl` items without a body / definition (`const`, `type`, and `fn`),
2019-12-10T13:46:30.1641860Z       - and associated `type`s in `impl`s with bounds, e.g., `type Foo: Ord;`.
2019-12-10T13:46:30.1642021Z     - `...` can occur anywhere in the list syntactically (`fn foo(..., x: usize) {}`),
2019-12-10T13:46:30.1642135Z     - `fn`s in all contexts now syntactically allow `...`,
2019-12-10T13:46:30.1642226Z     - and `...` can be the sole parameter (`fn foo(...) {}`.
2019-12-10T13:46:30.1642340Z    - In particular, it requires that we syntactically allow (under `#[cfg(FALSE)]`):
2019-12-10T13:46:30.1642441Z    - The syntactic restrictions are replaced by semantic ones in `ast_validation`.
2019-12-10T13:46:30.1642553Z    - This is done by using the cover grammar of both forms.
2019-12-10T13:46:30.1642685Z - Merge `{Trait,Impl}Item{Kind?}` into `AssocItem{Kind?}` as discussed in https://github.com/rust-lang/rust/issues/65041#issuecomment-538105286.
2019-12-10T13:46:30.1642811Z - Move syntactic restrictions around C-variadic parameters from the parser into `ast_validation`:
2019-12-10T13:46:30.1643350Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-12-10T13:46:30.1643997Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-12-10T13:46:30.1644097Z AGENT_HOMEDIRECTORY=C:\agents\2.160.1
2019-12-10T13:46:30.1644166Z AGENT_ID=516
---
2019-12-10T13:46:30.1654917Z BUILD_SOURCEBRANCHNAME=auto
2019-12-10T13:46:30.1654985Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-12-10T13:46:30.1655120Z BUILD_SOURCEVERSION=0ecb374d13d500af84e0453ffde16d81cb97b52a
2019-12-10T13:46:30.1655211Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-12-10T13:46:30.1655319Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #67131 - Centril:item-merge, r=petrochenkov
2019-12-10T13:46:30.1655487Z CI_JOB_NAME=i686-mingw-2
2019-12-10T13:46:30.1655571Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-12-10T13:46:30.1655649Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-12-10T13:46:30.1655746Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-12-10T13:46:30.1661019Z MAVEN_OPTS=-Xms256m
2019-12-10T13:46:30.1661105Z MSDEPLOY_HTTP_USER_AGENT=VSTS_d439fc94-e01f-4249-b63e-d8392bc0247c_build_10_0
2019-12-10T13:46:30.1661212Z MSMPI_BIN=C:\Program Files\Microsoft MPI\Bin\
2019-12-10T13:46:30.1661298Z MSYSTEM=MINGW64
2019-12-10T13:46:30.1661371Z Merge `TraitItem` & `ImplItem into `AssocItem`
2019-12-10T13:46:30.1661577Z NPM_CONFIG_CACHE=C:\npm\cache
2019-12-10T13:46:30.1661684Z NPM_CONFIG_PREFIX=C:\npm\prefix
2019-12-10T13:46:30.1661786Z NUMBER_OF_PROCESSORS=2
2019-12-10T13:46:30.1661868Z OS=Windows_NT
---
2019-12-10T16:30:46.4103675Z test [ui] ui\parser\tag-variant-disr-non-nullary.rs ... ok
2019-12-10T16:30:46.5912271Z test [ui] ui\parser\trailing-carriage-return-in-string.rs ... ok
2019-12-10T16:30:47.0686641Z test [ui] ui\parser\trailing-plus-in-bounds.rs ... ok
2019-12-10T16:30:47.4305444Z test [ui] ui\parser\trait-bounds-not-on-impl.rs ... ok
2019-12-10T16:30:47.6316456Z test [ui] ui\parser\trait-item-with-defaultness-fail-semantic.rs ... ok
2019-12-10T16:30:47.8026349Z test [ui] ui\parser\trait-item-with-defaultness-pass.rs ... ok
2019-12-10T16:30:48.1936962Z test [ui] ui\parser\trait-object-lifetime-parens.rs ... ok
2019-12-10T16:30:48.3829081Z test [ui] ui\parser\trait-object-polytrait-priority.rs ... ok
2019-12-10T16:30:48.5926516Z test [ui] ui\parser\trait-object-trait-parens.rs ... ok
2019-12-10T16:30:49.0429178Z test [ui] ui\parser\trait-plusequal-splitting.rs ... ok
---
2019-12-10T16:49:11.1778032Z ---- [ui] ui\backtrace-debuginfo.rs stdout ----
2019-12-10T16:49:11.1778084Z 
2019-12-10T16:49:11.1778266Z error: test run failed!
2019-12-10T16:49:11.1778339Z status: exit code: 101
2019-12-10T16:49:11.1779746Z command: PATH="D:\a\1\s\build\i686-pc-windows-gnu\stage2\lib\rustlib\i686-pc-windows-gnu\lib;D:\a\1\s\build\i686-pc-windows-gnu\stage0-bootstrap-tools\i686-pc-windows-gnu\release\deps;D:\a\1\s\build\i686-pc-windows-gnu\stage0\bin;D:\a\1\s\ninja;D:\a\1\s\mingw32\bin;C:\Python27amd64;D:\a\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.160.1\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\npm\prefix;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.5\x64\bin;C:\Go1.12.7\bin;C:\Program Files\Git\bin;C:\Program Files\Git\usr\bin;C:\Program Files\Git\mingw64\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\Subversion\bin;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.2\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files (x86)\Microsoft SQL Server\120" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\test\\ui\\backtrace-debuginfo\\a.exe"
2019-12-10T16:49:11.1781138Z ------------------------------------------
2019-12-10T16:49:11.1781242Z ---------------------------------------
2019-12-10T16:49:11.1781322Z trace does not match position list
2019-12-10T16:49:11.1781322Z trace does not match position list
2019-12-10T16:49:11.1781435Z still need to find ["backtrace-debuginfo.rs:189", "backtrace-debuginfo.rs:125"]
2019-12-10T16:49:11.1781587Z --- stdout
2019-12-10T16:49:11.1781653Z backtrace-debuginfo-aux.rs:6
2019-12-10T16:49:11.1781744Z backtrace-debuginfo.rs:88
2019-12-10T16:49:11.1781816Z backtrace-debuginfo.rs:125
2019-12-10T16:49:11.1781816Z backtrace-debuginfo.rs:125
2019-12-10T16:49:11.1781902Z backtrace-debuginfo.rs:189
2019-12-10T16:49:11.1781948Z 
2019-12-10T16:49:11.1782023Z --- stderr
2019-12-10T16:49:11.1782085Z test case 2
2019-12-10T16:49:11.1782195Z thread 'main' panicked at 'explicit panic', D:\a\1\s\src/test\ui\backtrace-debuginfo.rs:89:9
2019-12-10T16:49:11.1782624Z stack backtrace:
2019-12-10T16:49:11.1782730Z    0: 0x64655d2c - _set_invalid_parameter_handler
2019-12-10T16:49:11.1782835Z    1: 0x6461adb5 - _set_invalid_parameter_handler
2019-12-10T16:49:11.1782923Z    2: 0x6462b4cf - _set_invalid_parameter_handler
2019-12-10T16:49:11.1783159Z    3: 0x6462b1d8 - _set_invalid_parameter_handler
2019-12-10T16:49:11.1783248Z    4: 0x6462bbee - _set_invalid_parameter_handler
2019-12-10T16:49:11.1783367Z    5:   0x41a230 - std::panicking::begin_panic::hdfc91a6bf0a55891
2019-12-10T16:49:11.1783466Z                        at D:\a\1\s\src\libstd/panicking.rs:404
2019-12-10T16:49:11.1783790Z    6:   0x409a5f - backtrace_debuginfo::inner::{{closure}}::he203707f23085cd6
2019-12-10T16:49:11.1784101Z                        at D:\a\1\s\src/test\ui/backtrace-debuginfo.rs:89
2019-12-10T16:49:11.1784306Z    7:   0x40996b - backtrace_debuginfo::aux::callback::h7593cc62965c93e6
2019-12-10T16:49:11.1784514Z                        at D:\a\1\s\src/test\ui/backtrace-debuginfo-aux.rs:6
2019-12-10T16:49:11.1784718Z    8:   0x4123cb - backtrace_debuginfo::inner::hba9feb2184e80f2b
2019-12-10T16:49:11.1784905Z                        at D:\a\1\s\src/test\ui/backtrace-debuginfo.rs:88
2019-12-10T16:49:11.1785110Z    9:   0xa0c4c8 - _set_invalid_parameter_handler
2019-12-10T16:49:11.1785288Z   10:   0x40d21c - core::str::<impl str>::parse::h87ccddfdea055ac7
2019-12-10T16:49:11.1785571Z                        at D:\a\1\s\src\libcore\str/mod.rs:3970
2019-12-10T16:49:11.1785904Z ---------------------------------------
2019-12-10T16:49:11.1786088Z trace does not match position list
2019-12-10T16:49:11.1786088Z trace does not match position list
2019-12-10T16:49:11.1787314Z still need to find ["backtrace-debuginfo.rs:189", "backtrace-debuginfo.rs:126"]
2019-12-10T16:49:11.1790298Z --- stdout
2019-12-10T16:49:11.1790385Z backtrace-debuginfo.rs:109
2019-12-10T16:49:11.1790711Z backtrace-debuginfo.rs:126
2019-12-10T16:49:11.1790786Z backtrace-debuginfo.rs:189
2019-12-10T16:49:11.1790786Z backtrace-debuginfo.rs:189
2019-12-10T16:49:11.1790831Z 
2019-12-10T16:49:11.1790908Z --- stderr
2019-12-10T16:49:11.1790970Z test case 6
2019-12-10T16:49:11.1791082Z thread 'main' panicked at 'explicit panic', D:\a\1\s\src/test\ui\backtrace-debuginfo.rs:107:9
2019-12-10T16:49:11.1791207Z stack backtrace:
2019-12-10T16:49:11.1791289Z    0: 0x64655d2c - _set_invalid_parameter_handler
2019-12-10T16:49:11.1791396Z    1: 0x6461adb5 - _set_invalid_parameter_handler
2019-12-10T16:49:11.1791491Z    2: 0x6462b4cf - _set_invalid_parameter_handler
2019-12-10T16:49:11.1791596Z    3: 0x6462b1d8 - _set_invalid_parameter_handler
2019-12-10T16:49:11.1792172Z    4: 0x6462bbee - _set_invalid_parameter_handler
2019-12-10T16:49:11.1792285Z    5:   0x41a230 - std::panicking::begin_panic::hdfc91a6bf0a55891
2019-12-10T16:49:11.1792401Z                        at D:\a\1\s\src\libstd/panicking.rs:404
2019-12-10T16:49:11.1792509Z    6:   0x412a26 - backtrace_debuginfo::inner_inlined::inner_further_inlined::hc6cb011b12fdc07c
2019-12-10T16:49:11.1792638Z                        at D:\a\1\s\src/test\ui/backtrace-debuginfo.rs:107
2019-12-10T16:49:11.1792739Z    7:   0x4126cc - backtrace_debuginfo::inner_inlined::h77ab595dd670898f
2019-12-10T16:49:11.1792858Z                        at D:\a\1\s\src/test\ui/backtrace-debuginfo.rs:109
2019-12-10T16:49:11.1792978Z    8:   0x9ec4c8 - _set_invalid_parameter_handler
2019-12-10T16:49:11.1793073Z    9:   0x40d21c - core::str::<impl str>::parse::h87ccddfdea055ac7
2019-12-10T16:49:11.1793195Z                        at D:\a\1\s\src\libcore\str/mod.rs:3970
2019-12-10T16:49:11.1793328Z ---------------------------------------
2019-12-10T16:49:11.1793405Z trace does not match position list
2019-12-10T16:49:11.1793405Z trace does not match position list
2019-12-10T16:49:11.1793518Z still need to find ["backtrace-debuginfo.rs:189", "backtrace-debuginfo.rs:126"]
2019-12-10T16:49:11.1793654Z --- stdout
2019-12-10T16:49:11.1793719Z backtrace-debuginfo-aux.rs:6
2019-12-10T16:49:11.1793810Z backtrace-debuginfo.rs:111
2019-12-10T16:49:11.1793884Z backtrace-debuginfo.rs:126
2019-12-10T16:49:11.1793884Z backtrace-debuginfo.rs:126
2019-12-10T16:49:11.1793972Z backtrace-debuginfo.rs:189
2019-12-10T16:49:11.1794017Z 
2019-12-10T16:49:11.1794092Z --- stderr
2019-12-10T16:49:11.1794155Z test case 7
2019-12-10T16:49:11.1794295Z thread 'main' panicked at 'explicit panic', D:\a\1\s\src/test\ui\backtrace-debuginfo.rs:112:9
2019-12-10T16:49:11.1794527Z stack backtrace:
2019-12-10T16:49:11.1794624Z    0: 0x64655d2c - _set_invalid_parameter_handler
2019-12-10T16:49:11.1794721Z    1: 0x6461adb5 - _set_invalid_parameter_handler
2019-12-10T16:49:11.1794825Z    2: 0x6462b4cf - _set_invalid_parameter_handler
2019-12-10T16:49:11.1794912Z    3: 0x6462b1d8 - _set_invalid_parameter_handler
2019-12-10T16:49:11.1795016Z    4: 0x6462bbee - _set_invalid_parameter_handler
2019-12-10T16:49:11.1795127Z    5:   0x41a230 - std::panicking::begin_panic::hdfc91a6bf0a55891
2019-12-10T16:49:11.1795227Z                        at D:\a\1\s\src\libstd/panicking.rs:404
2019-12-10T16:49:11.1795343Z    6:   0x409b5f - backtrace_debuginfo::inner_inlined::{{closure}}::hfb3950c4d50c2864
2019-12-10T16:49:11.1795451Z                        at D:\a\1\s\src/test\ui/backtrace-debuginfo.rs:112
2019-12-10T16:49:11.1795578Z    7:   0x40990b - backtrace_debuginfo::aux::callback::h2d349a730207a6f1
2019-12-10T16:49:11.1795704Z                        at D:\a\1\s\src/test\ui/backtrace-debuginfo-aux.rs:6
2019-12-10T16:49:11.1795850Z    8:   0x4127a0 - backtrace_debuginfo::inner_inlined::h77ab595dd670898f
2019-12-10T16:49:11.1796039Z                        at D:\a\1\s\src/test\ui/backtrace-debuginfo.rs:111
2019-12-10T16:49:11.1796555Z    9:   0x81c4c8 - _set_invalid_parameter_handler
2019-12-10T16:49:11.1796676Z   10:   0x40d21c - core::str::<impl str>::parse::h87ccddfdea055ac7
2019-12-10T16:49:11.1796791Z                        at D:\a\1\s\src\libcore\str/mod.rs:3970
2019-12-10T16:49:11.1796881Z 
2019-12-10T16:49:11.1796966Z ------------------------------------------
2019-12-10T16:49:11.1797040Z stderr:
2019-12-10T16:49:11.1797128Z ------------------------------------------
2019-12-10T16:49:11.1797128Z ------------------------------------------
2019-12-10T16:49:11.1797230Z thread 'main' panicked at 'found some errors', D:\a\1\s\src/test\ui\backtrace-debuginfo.rs:179:9
2019-12-10T16:49:11.1797428Z 
2019-12-10T16:49:11.1797513Z ------------------------------------------
2019-12-10T16:49:11.1797562Z 
2019-12-10T16:49:11.1797596Z 
---
2019-12-10T16:49:11.1816050Z thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:537:22
2019-12-10T16:49:11.1817170Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-10T16:49:11.1855494Z 
2019-12-10T16:49:11.1855810Z 
2019-12-10T16:49:11.1856570Z command did not execute successfully: "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\stage2\\bin" "--run-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\stage2\\lib\\rustlib\\i686-pc-windows-gnu\\lib" "--rustc-path" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\1\\s\\src/test\\ui" "--build-base" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\test\\ui" "--stage-id" "stage2-i686-pc-windows-gnu" "--mode" "ui" "--target" "i686-pc-windows-gnu" "--host" "i686-pc-windows-gnu" "--llvm-filecheck" "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\native\\rust-test-helpers" "--docck-python" "C:\\Python27amd64\\python2.7" "--lldb-python" "C:\\Python27amd64\\python2.7" "--gdb" "D:\\a\\1\\s\\mingw32\\bin\\gdb" "--llvm-version" "9.0.0-rust-1.41.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-10T16:49:11.1857446Z 
2019-12-10T16:49:11.1857503Z 
2019-12-10T16:49:11.2874640Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test src/test/ui src/test/compile-fail
2019-12-10T16:49:11.2874914Z Build completed unsuccessfully in 2:52:54
2019-12-10T16:49:11.2874914Z Build completed unsuccessfully in 2:52:54
2019-12-10T16:49:11.3683706Z make: *** [Makefile:91: ci-mingw-subset-2] Error 1
2019-12-10T16:49:11.4306978Z   local time: Tue Dec 10 16:49:11 CUT 2019
2019-12-10T16:49:11.8765053Z   network time: Tue, 10 Dec 2019 16:49:11 GMT
2019-12-10T16:49:11.8786019Z == end clock drift check ==
2019-12-10T16:49:11.9646984Z 
2019-12-10T16:49:11.9646984Z 
2019-12-10T16:49:12.2560072Z ##[error]Bash exited with code '2'.
2019-12-10T16:49:12.3072230Z ##[section]Starting: Checkout
2019-12-10T16:49:12.4035390Z ==============================================================================
2019-12-10T16:49:12.4035526Z Task         : Get sources
2019-12-10T16:49:12.4035618Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

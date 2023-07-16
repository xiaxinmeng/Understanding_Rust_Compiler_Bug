plain
To only update this specific test, also pass `--test-args panics\default-backtrace-ice.rs`

error: 1 errors occurred comparing output.
status: exit code: 101
command: PATH="C:\a\rust\rust\build\i686-pc-windows-gnu\stage2\bin;C:\a\rust\rust\build\i686-pc-windows-gnu\stage0-bootstrap-tools\i686-pc-windows-gnu\release\deps;C:\a\rust\rust\build\i686-pc-windows-gnu\stage0\bin;C:\a\rust\rust\ninja;C:\a\rust\rust\mingw32\bin;C:\hostedtoolcache\windows\Python\3.11.3\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.3\x64;C:\msys64\usr\bin;C:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.3.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.20.3\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\Program Files\OpenSSL\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.372-7\x64\bin;C:\Program Files\ImageMagick-7.1.1-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.7\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage2\\bin\\rustc.exe" "C:\\a\\rust\\rust\\tests\\ui\\panics\\default-backtrace-ice.rs" "-Zthreads=1" "--sysroot" "C:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage2" "--target=i686-pc-windows-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=C:\\a\\rust\\rust\\tests\\ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "C:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\test\\ui\\panics\\default-backtrace-ice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=C:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\native\\rust-test-helpers" "-L" "C:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\test\\ui\\panics\\default-backtrace-ice\\auxiliary" "-Z" "treat-err-as-bug=1"
stdout: none
error[E0425]: cannot find value `missing_ident` in this scope
error[E0425]: cannot find value `missing_ident` in this scope
  --> fake-test-src-base\panics\default-backtrace-ice.rs:21:13
   |
LL | fn main() { missing_ident; }


thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler\rustc_errors\src\lib.rs:1711:30
stack backtrace:
   0: 0x66f1e3fe - core::fmt::write::hf5380ac3ee3d71aa
   1: 0x66eb4184 - std::io::Write::write_fmt::hc306171dc7ebfc12
   2: 0x66ebd990 - std::sys_common::backtrace::print::h57d6ebb4a714f582
   3: 0x66ec078f - std::panicking::default_hook::{{closure}}::h942d09aeaff11c5f
   4:  0x1256358 - <unknown>
   5: 0x66ebfdb6 - ___rdl_dealloc
   6: 0x66ec0c68 - std::panicking::begin_panic_handler::{{closure}}::h2330c75e4d60f760
   7: 0x6a43bb07 - <rustc_errors[2007af0c6b9b7a02]::HandlerInner>::panic_if_treat_err_as_bug
   8: 0x6a43b10e - <rustc_errors[2007af0c6b9b7a02]::HandlerInner>::emit_diagnostic::{closure#2}
   9: 0x67597ad3 - rustc_interface[3670b4b1319935bc]::callbacks::track_diagnostic
  10: 0x6a43a09f - <rustc_errors[2007af0c6b9b7a02]::HandlerInner>::emit_diagnostic
  11: 0x6a4440d8 - <rustc_span[1fc5373ceb26c49]::ErrorGuaranteed as rustc_errors[2007af0c6b9b7a02]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  12: 0x684cc8fe - <rustc_resolve[8f76d2cca51f3187]::Resolver>::report_errors
  13: 0x68490f77 - <rustc_session[7463595a7fe00c8]::session::Session>::time::<(), <rustc_resolve[8f76d2cca51f3187]::Resolver>::resolve_crate::{closure#0}>
  14: 0x684f53b1 - <rustc_resolve[8f76d2cca51f3187]::Resolver>::resolve_crate
  15: 0x6754f328 - rustc_interface[3670b4b1319935bc]::passes::resolver_for_lowering
  16: 0x691b411b - rustc_query_system[d5db14f87540071f]::query::plumbing::try_execute_query::<rustc_query_impl[c5e22ac7ce001d03]::queries::resolver_for_lowering, rustc_query_impl[c5e22ac7ce001d03]::plumbing::QueryCtxt>
  17: 0x68f989bf - rustc_query_impl[c5e22ac7ce001d03]::get_query::resolver_for_lowering
  18: 0x6746d3d2 - <rustc_middle[cf9c5a59c6c6eca]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[1203a84557310c48]::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures[be249329499d1260]::steal::Steal<(rustc_middle[cf9c5a59c6c6eca]::ty::ResolverAstLowering, alloc[6cb854f4b384d7a6]::rc::Rc<rustc_ast[e3ff4c0dfdf8e34c]::ast::Crate>)>>
  19: 0x674a867d - <rustc_interface[3670b4b1319935bc]::queries::QueryResult<&rustc_middle[cf9c5a59c6c6eca]::ty::context::GlobalCtxt>>::enter::<&rustc_data_structures[be249329499d1260]::steal::Steal<(rustc_middle[cf9c5a59c6c6eca]::ty::ResolverAstLowering, alloc[6cb854f4b384d7a6]::rc::Rc<rustc_ast[e3ff4c0dfdf8e34c]::ast::Crate>)>, rustc_driver_impl[1203a84557310c48]::run_compiler::{closure#1}::{closure#2}::{closure#2}>
  20: 0x674a968a - <rustc_interface[3670b4b1319935bc]::interface::Compiler>::enter::<rustc_driver_impl[1203a84557310c48]::run_compiler::{closure#1}::{closure#2}, core[11d4c4719e5f176b]::result::Result<core[11d4c4719e5f176b]::option::Option<rustc_interface[3670b4b1319935bc]::queries::Linker>, rustc_span[1fc5373ceb26c49]::ErrorGuaranteed>>
  21: 0x67473d73 - std[72a8a6c0d59fa6f5]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3670b4b1319935bc]::util::run_in_thread_pool_with_globals<rustc_interface[3670b4b1319935bc]::interface::run_compiler<core[11d4c4719e5f176b]::result::Result<(), rustc_span[1fc5373ceb26c49]::ErrorGuaranteed>, rustc_driver_impl[1203a84557310c48]::run_compiler::{closure#1}>::{closure#0}, core[11d4c4719e5f176b]::result::Result<(), rustc_span[1fc5373ceb26c49]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[11d4c4719e5f176b]::result::Result<(), rustc_span[1fc5373ceb26c49]::ErrorGuaranteed>>
  22: 0x6746653a - <<std[72a8a6c0d59fa6f5]::thread::Builder>::spawn_unchecked_<rustc_interface[3670b4b1319935bc]::util::run_in_thread_pool_with_globals<rustc_interface[3670b4b1319935bc]::interface::run_compiler<core[11d4c4719e5f176b]::result::Result<(), rustc_span[1fc5373ceb26c49]::ErrorGuaranteed>, rustc_driver_impl[1203a84557310c48]::run_compiler::{closure#1}>::{closure#0}, core[11d4c4719e5f176b]::result::Result<(), rustc_span[1fc5373ceb26c49]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[11d4c4719e5f176b]::result::Result<(), rustc_span[1fc5373ceb26c49]::ErrorGuaranteed>>::{closure#1} as core[11d4c4719e5f176b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  23: 0x66ed3569 - __ZN3std3sys7windows6thread6Thread3new12thread_start17h2507f15c1a234d1eE@4
  24: 0x774105b9 - <unknown>
  25: 0x77e077ad - <unknown>
  26: 0x77e0777d - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (390e41af3 2023-05-08) running on i686-pc-windows-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z treat-err-as-bug=1
query stack during panic:
#0 [resolver_for_lowering] getting the resolver for lowering
end of query stack
------------------------------------------
---
test result: FAILED. 14713 passed; 1 failed; 234 ignored; 0 measured; 0 filtered out; finished in 944.46s

Some tests failed in compiletest suite=ui mode=ui host=i686-pc-windows-gnu target=i686-pc-windows-gnu
Build completed unsuccessfully in 0:57:47
make: *** [Makefile:80: ci-mingw-subset-2] Error 1


2020-07-08T17:36:12.9399991Z [0m[0m[1m[32m    Finished[0m release [optimized] target(s) in 6.91s
2020-07-08T17:36:12.9795029Z [0m[0m[1m[32m     Running[0m `build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo-miri.exe miri setup`
2020-07-08T17:36:13.2063128Z error: Cargo.lock file is missing from source dir
2020-07-08T17:36:13.2064255Z caused by: The filename, directory name, or volume label syntax is incorrect. (os error 123)
2020-07-08T17:36:13.2065139Z    0: backtrace::backtrace::trace
2020-07-08T17:36:13.2065414Z    1: backtrace::capture::Backtrace::new
2020-07-08T17:36:13.2065656Z    2: error_chain::make_backtrace
2020-07-08T17:36:13.2065918Z    3: <core::result::Result<T,E> as xargo::errors::ResultExt<T,E>>::chain_err
2020-07-08T17:36:13.2067011Z    4: alloc::raw_vec::RawVec<T,A>::reserve
2020-07-08T17:36:13.2067250Z    5: xargo::sysroot::update
2020-07-08T17:36:13.2067477Z    6: xargo::main_inner
2020-07-08T17:36:13.2067645Z    7: _cflush
2020-07-08T17:36:13.2067828Z    8: std::rt::lang_start_internal
2020-07-08T17:36:13.2067993Z    9: main
2020-07-08T17:36:13.2068617Z   10: invoke_main
2020-07-08T17:36:13.2068823Z              at d:\A01\_work\6\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
2020-07-08T17:36:13.2069025Z   11: __scrt_common_main_seh
2020-07-08T17:36:13.2069299Z              at d:\A01\_work\6\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
2020-07-08T17:36:13.2069492Z   12: BaseThreadInitThunk
2020-07-08T17:36:13.2069662Z   13: RtlUserThreadStart
2020-07-08T17:36:13.2069779Z 
2020-07-08T17:36:13.2106009Z fatal error: Failed to run xargo
2020-07-08T17:36:13.2149621Z [0m[0m[1m[31merror[0m[1m:[0m process didn't exit successfully: `build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo-miri.exe miri setup` (exit code: 1)

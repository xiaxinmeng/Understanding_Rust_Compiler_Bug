plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking hashbrown v0.11.0
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking miniz_oxide v0.4.0
    Checking addr2line v0.14.0
error[E0425]: cannot find value `NotSupported` in this scope
   --> library/std/src/sys/windows/mod.rs:94:67
94  |         c::ERROR_CALL_NOT_IMPLEMENTED                   => return NotSupported,
94  |         c::ERROR_CALL_NOT_IMPLEMENTED                   => return NotSupported,
    |                                                                   ^^^^^^^^^^^^ help: a unit variant with a similar name exists: `Unsupported`
   ::: library/std/src/io/error.rs:284:5
    |
284 |     Unsupported,
284 |     Unsupported,
    |     ----------- similarly named unit variant `Unsupported` defined here

error[E0425]: cannot find value `NotSupported` in this scope
   --> library/std/src/sys/windows/mod.rs:95:67
95  |         c::ERROR_CALL_NOT_IMPLEMENTED                   => return NotSupported,
95  |         c::ERROR_CALL_NOT_IMPLEMENTED                   => return NotSupported,
    |                                                                   ^^^^^^^^^^^^ help: a unit variant with a similar name exists: `Unsupported`
   ::: library/std/src/io/error.rs:284:5
    |
284 |     Unsupported,
284 |     Unsupported,
    |     ----------- similarly named unit variant `Unsupported` defined here

error[E0531]: cannot find unit struct, unit variant or constant `ERROR_HOST_UNREACHABLE` in module `c`
  --> library/std/src/sys/windows/mod.rs:96:12
   |
96 |         c::ERROR_HOST_UNREACHABLE                       => return HostUnreachable,
   |            ^^^^^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `ERROR_NETWORK_UNREACHABLE` in module `c`
  --> library/std/src/sys/windows/mod.rs:97:12
   |
97 |         c::ERROR_NETWORK_UNREACHABLE                    => return NetworkUnreachable,
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `ERROR_DIRECTORY` in module `c`
  --> library/std/src/sys/windows/mod.rs:98:12
   |
98 |         c::ERROR_DIRECTORY                              => return NotADirectory,
   |            ^^^^^^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `ERROR_DIRECTORY_NOT_SUPPORTED` in module `c`
  --> library/std/src/sys/windows/mod.rs:99:12
   |
99 |         c::ERROR_DIRECTORY_NOT_SUPPORTED                => return IsADirectory,
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `ERROR_DIR_NOT_EMPTY` in module `c`
   --> library/std/src/sys/windows/mod.rs:100:12
    |
100 |         c::ERROR_DIR_NOT_EMPTY                          => return DirectoryNotEmpty,
    |            ^^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `ERROR_WRITE_PROTECT` in module `c`
   --> library/std/src/sys/windows/mod.rs:101:12
    |
101 |         c::ERROR_WRITE_PROTECT                          => return ReadOnlyFilesystem,
    |            ^^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `ERROR_DISK_FULL` in module `c`
   --> library/std/src/sys/windows/mod.rs:102:12
    |
102 |         c::ERROR_DISK_FULL
    |            ^^^^^^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `ERROR_HANDLE_DISK_FULL` in module `c`
   --> library/std/src/sys/windows/mod.rs:103:14
    |
103 |         | c::ERROR_HANDLE_DISK_FULL                     => return StorageFull,
    |              ^^^^^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `ERROR_SEEK_ON_DEVICE` in module `c`
   --> library/std/src/sys/windows/mod.rs:104:12
    |
104 |         c::ERROR_SEEK_ON_DEVICE                         => return NotSeekable,
    |            ^^^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `ERROR_DISK_QUOTA_EXCEEDED` in module `c`
   --> library/std/src/sys/windows/mod.rs:105:12
    |
105 |         c::ERROR_DISK_QUOTA_EXCEEDED                    => return FilesystemQuotaExceeded,
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `ERROR_FILE_TOO_LARGE` in module `c`
   --> library/std/src/sys/windows/mod.rs:106:12
    |
106 |         c::ERROR_FILE_TOO_LARGE                         => return FileTooLarge,
    |            ^^^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `ERROR_BUSY` in module `c`
   --> library/std/src/sys/windows/mod.rs:107:12
    |
107 |         c::ERROR_BUSY                                   => return ResourceBusy,
    |            ^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `ERROR_POSSIBLE_DEADLOCK` in module `c`
   --> library/std/src/sys/windows/mod.rs:108:12
108 |         c::ERROR_POSSIBLE_DEADLOCK                      => return Deadlock,
    |            ^^^^^^^^^^^^^^^^^^^^^^^ not found in `c`


error[E0531]: cannot find unit struct, unit variant or constant `ERROR_NOT_SAME_DEVICE` in module `c`
   --> library/std/src/sys/windows/mod.rs:109:12
    |
109 |         c::ERROR_NOT_SAME_DEVICE                        => return CrossDeviceLink,
    |            ^^^^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0425]: cannot find value `CrossDeviceLink` in this scope
   --> library/std/src/sys/windows/mod.rs:109:67
    |
109 |         c::ERROR_NOT_SAME_DEVICE                        => return CrossDeviceLink,


error[E0531]: cannot find unit struct, unit variant or constant `ERROR_TOO_MANY_LINKS` in module `c`
   --> library/std/src/sys/windows/mod.rs:110:12
    |
110 |         c::ERROR_TOO_MANY_LINKS                         => return TooManyLinks,
    |            ^^^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `ERROR_FILENAME_EXCED_RANGE` in module `c`
   --> library/std/src/sys/windows/mod.rs:111:12
    |
111 |         c::ERROR_FILENAME_EXCED_RANGE                   => return FilenameTooLong,
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `WSAEHOSTUNREACH` in module `c`
   --> library/std/src/sys/windows/mod.rs:126:12
    |
126 |         c::WSAEHOSTUNREACH      => HostUnreachable,
    |            ^^^^^^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `WSAENETDOWN` in module `c`
   --> library/std/src/sys/windows/mod.rs:127:12
    |
127 |         c::WSAENETDOWN          => NetworkDown,
    |            ^^^^^^^^^^^ help: a constant with a similar name exists: `WSAENOTCONN`
    | 
   ::: library/std/src/sys/windows/c.rs:141:1
    |
141 | pub const WSAENOTCONN: c_int = 10057;
    | ------------------------------------- similarly named constant `WSAENOTCONN` defined here

error[E0531]: cannot find unit struct, unit variant or constant `WSAENETUNREACH` in module `c`
   --> library/std/src/sys/windows/mod.rs:128:12
    |
128 |         c::WSAENETUNREACH       => NetworkUnreachable,
    |            ^^^^^^^^^^^^^^ not found in `c`
error: aborting due to 22 previous errors

Some errors have detailed explanations: E0425, E0531.
For more information about an error, try `rustc --explain E0425`.
For more information about an error, try `rustc --explain E0425`.
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:29

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
    Checking addr2line v0.14.0
error: expected pattern, found `,`
  --> library/std/src/sys/windows/mod.rs:93:57
   |
93 |         c::ERROR_BUSY => return ErrorKind::ResourceBusy,,
   |                                                         ^ expected pattern

error[E0531]: cannot find unit struct, unit variant or constant `ERROR_HOST_UNREACHABLE` in module `c`
  --> library/std/src/sys/windows/mod.rs:82:12
   |
82 |         c::ERROR_HOST_UNREACHABLE => return ErrorKind::HostUnreachable,
   |            ^^^^^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `ERROR_NETWORK_UNREACHABLE` in module `c`
  --> library/std/src/sys/windows/mod.rs:83:12
   |
83 |         c::ERROR_NETWORK_UNREACHABLE => return ErrorKind::NetworkUnreachable,
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `ERROR_DIRECTORY` in module `c`
  --> library/std/src/sys/windows/mod.rs:84:12
   |
84 |         c::ERROR_DIRECTORY => return ErrorKind::NotADirectory,
   |            ^^^^^^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `ERROR_DIRECTORY_NOT_SUPPORTED` in module `c`
  --> library/std/src/sys/windows/mod.rs:85:12
   |
85 |         c::ERROR_DIRECTORY_NOT_SUPPORTED => return ErrorKind::IsADirectory,
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `ERROR_DIR_NOT_EMPTY` in module `c`
  --> library/std/src/sys/windows/mod.rs:86:12
   |
86 |         c::ERROR_DIR_NOT_EMPTY => return ErrorKind::DirectoryNotEmpty,
   |            ^^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `ERROR_WRITE_PROTECT` in module `c`
  --> library/std/src/sys/windows/mod.rs:87:12
   |
87 |         c::ERROR_WRITE_PROTECT => return ErrorKind::ReadOnlyFilesystem,
   |            ^^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `ERROR_DISK_FULL` in module `c`
  --> library/std/src/sys/windows/mod.rs:88:12
   |
88 |         c::ERROR_DISK_FULL
   |            ^^^^^^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `ERROR_HANDLE_DISK_FULL` in module `c`
  --> library/std/src/sys/windows/mod.rs:89:14
   |
89 |         | c::ERROR_HANDLE_DISK_FULL => return ErrorKind::StorageFull,
   |              ^^^^^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `ERROR_SEEK_ON_DEVICE` in module `c`
  --> library/std/src/sys/windows/mod.rs:90:12
   |
90 |         c::ERROR_SEEK_ON_DEVICE => return ErrorKind::NotSeekable,
   |            ^^^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `ERROR_DISK_QUOTA_EXCEEDED` in module `c`
  --> library/std/src/sys/windows/mod.rs:91:12
   |
91 |         c::ERROR_DISK_QUOTA_EXCEEDED => return ErrorKind::FilesystemQuotaExceeded,
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `ERROR_FILE_TOO_LARGE` in module `c`
  --> library/std/src/sys/windows/mod.rs:92:12
   |
92 |         c::ERROR_FILE_TOO_LARGE => return ErrorKind::FileTooLarge,
   |            ^^^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `ERROR_BUSY` in module `c`
  --> library/std/src/sys/windows/mod.rs:93:12
   |
93 |         c::ERROR_BUSY => return ErrorKind::ResourceBusy,,
   |            ^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `WSAEHOSTUNREACH` in module `c`
   --> library/std/src/sys/windows/mod.rs:112:12
    |
112 |         c::WSAEHOSTUNREACH => return ErrorKind::HostUnreachable,
    |            ^^^^^^^^^^^^^^^ not found in `c`

error[E0531]: cannot find unit struct, unit variant or constant `WSAENETUNREACH` in module `c`
   --> library/std/src/sys/windows/mod.rs:113:12
    |
113 |         c::WSAENETUNREACH => return ErrorKind::NetworkUnreachable,
    |            ^^^^^^^^^^^^^^ not found in `c`
error: unreachable expression
   --> library/std/src/sys/windows/mod.rs:101:5
    |
    |
56  | /     match errno as c::DWORD {
57  | |         c::ERROR_ACCESS_DENIED => return ErrorKind::PermissionDenied,
58  | |         c::ERROR_ALREADY_EXISTS => return ErrorKind::AlreadyExists,
59  | |         c::ERROR_FILE_EXISTS => return ErrorKind::AlreadyExists,
98  | |         _ => {}
99  | |     }
99  | |     }
    | |_____- any code following this `match` expression is unreachable, as all arms diverge
101 | /     match errno {
101 | /     match errno {
102 | |         c::WSAEACCES => ErrorKind::PermissionDenied,
103 | |         c::WSAEADDRINUSE => ErrorKind::AddrInUse,
104 | |         c::WSAEADDRNOTAVAIL => ErrorKind::AddrNotAvailable,
115 | |         _ => ErrorKind::Other,
116 | |     }
    | |_____^ unreachable expression
    |
    |
    = note: `-D unreachable-code` implied by `-D warnings`
error: aborting due to 16 previous errors

For more information about this error, try `rustc --explain E0531`.
error: could not compile `std`
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:25

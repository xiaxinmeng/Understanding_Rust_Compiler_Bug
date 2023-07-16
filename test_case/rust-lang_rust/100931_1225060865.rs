plain
   Compiling walkdir v2.3.2
   Compiling ignore v0.4.18
   Compiling toml v0.5.9
   Compiling xz2 v0.1.6
error[E0425]: cannot find function `getuid` in crate `libc`
    |
    |
403 |                 let uid = unsafe { libc::getuid() };
    |                                          ^^^^^^ help: a function with a similar name exists: `getpid`
   ::: C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\libc-0.2.126\src\windows\mod.rs:474:5
    |
474 |     pub fn getpid() -> ::c_int;
474 |     pub fn getpid() -> ::c_int;
    |     -------------------------- similarly named function `getpid` defined here
For more information about this error, try `rustc --explain E0425`.
error: could not compile `bootstrap` due to previous error
failed to run: D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin\cargo.exe build --manifest-path D:\a\rust\rust\src/bootstrap/Cargo.toml --locked --features build-metrics
Build completed unsuccessfully in 0:01:34
Build completed unsuccessfully in 0:01:34
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 2/5:
Building rustbuild
   Compiling bootstrap v0.0.0 (D:\a\rust\rust\src\bootstrap)
error[E0425]: cannot find function `getuid` in crate `libc`
    |
    |
403 |                 let uid = unsafe { libc::getuid() };
    |                                          ^^^^^^ help: a function with a similar name exists: `getpid`
   ::: C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\libc-0.2.126\src\windows\mod.rs:474:5
    |
474 |     pub fn getpid() -> ::c_int;
474 |     pub fn getpid() -> ::c_int;
    |     -------------------------- similarly named function `getpid` defined here
For more information about this error, try `rustc --explain E0425`.
error: could not compile `bootstrap` due to previous error
failed to run: D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin\cargo.exe build --manifest-path D:\a\rust\rust\src/bootstrap/Cargo.toml --locked --features build-metrics
Build completed unsuccessfully in 0:00:02
Build completed unsuccessfully in 0:00:02
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 3/5:
Building rustbuild
   Compiling bootstrap v0.0.0 (D:\a\rust\rust\src\bootstrap)
error[E0425]: cannot find function `getuid` in crate `libc`
    |
    |
403 |                 let uid = unsafe { libc::getuid() };
    |                                          ^^^^^^ help: a function with a similar name exists: `getpid`
   ::: C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\libc-0.2.126\src\windows\mod.rs:474:5
    |
474 |     pub fn getpid() -> ::c_int;
474 |     pub fn getpid() -> ::c_int;
    |     -------------------------- similarly named function `getpid` defined here
For more information about this error, try `rustc --explain E0425`.
error: could not compile `bootstrap` due to previous error
failed to run: D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin\cargo.exe build --manifest-path D:\a\rust\rust\src/bootstrap/Cargo.toml --locked --features build-metrics
Build completed unsuccessfully in 0:00:02
Build completed unsuccessfully in 0:00:02
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 4/5:
Building rustbuild
   Compiling bootstrap v0.0.0 (D:\a\rust\rust\src\bootstrap)
error[E0425]: cannot find function `getuid` in crate `libc`
    |
    |
403 |                 let uid = unsafe { libc::getuid() };
    |                                          ^^^^^^ help: a function with a similar name exists: `getpid`
   ::: C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\libc-0.2.126\src\windows\mod.rs:474:5
    |
474 |     pub fn getpid() -> ::c_int;
474 |     pub fn getpid() -> ::c_int;
    |     -------------------------- similarly named function `getpid` defined here
For more information about this error, try `rustc --explain E0425`.
error: could not compile `bootstrap` due to previous error
failed to run: D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin\cargo.exe build --manifest-path D:\a\rust\rust\src/bootstrap/Cargo.toml --locked --features build-metrics
Build completed unsuccessfully in 0:00:02
Build completed unsuccessfully in 0:00:02
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 5/5:
Building rustbuild
   Compiling bootstrap v0.0.0 (D:\a\rust\rust\src\bootstrap)
error[E0425]: cannot find function `getuid` in crate `libc`
    |
    |
403 |                 let uid = unsafe { libc::getuid() };
    |                                          ^^^^^^ help: a function with a similar name exists: `getpid`
   ::: C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\libc-0.2.126\src\windows\mod.rs:474:5
    |
474 |     pub fn getpid() -> ::c_int;
474 |     pub fn getpid() -> ::c_int;
    |     -------------------------- similarly named function `getpid` defined here
For more information about this error, try `rustc --explain E0425`.
error: could not compile `bootstrap` due to previous error
failed to run: D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin\cargo.exe build --manifest-path D:\a\rust\rust\src/bootstrap/Cargo.toml --locked --features build-metrics
Build completed unsuccessfully in 0:00:02

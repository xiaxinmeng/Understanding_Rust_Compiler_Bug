rust
/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust 
$ ./go
~/build/2nonpkgs/rust.stuff/rust/rust ~/build/2nonpkgs/rust.stuff/rust
./go:47+ set -xev
#export RUSTFLAGS="-Z verbose"
export RUSTFLAGS="-Zverbose"
./go:49+ export RUSTFLAGS=-Zverbose
./go:49+ RUSTFLAGS=-Zverbose
verbose='-vv'
./go:50+ verbose=-vv
#verbose=''
threads=5
./go:52+ threads=5
#threads=1
time PATH="${HOME}/build/1packages/4used/chro/4_chromium-dev-git/makepkg_pacman/chromium-dev/src/python-path/:${PATH}" python2 ./x.py build $verbose --incremental -j $threads
./go:54+ PATH=/home/xftroxgpx/build/1packages/4used/chro/4_chromium-dev-git/makepkg_pacman/chromium-dev/src/python-path/:/opt/google-cloud-sdk/bin:/opt/google-cloud-sdk/bin:/usr/local/sbin:/usr/local/bin:/usr/bin:/opt/depot_tools:/usr/bin/site_perl:/usr/bin/vendor_perl:/usr/bin/core_perl:/home/xftroxgpx/bin:/home/xftroxgpx/build/2nonpkgs/rust.stuff/racer/target/release:/home/xftroxgpx/build/2nonpkgs/rust.stuff/chars/target/debug:/home/xftroxgpx/build/2nonpkgs/rust.stuff/chars/target/release:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rustfmt/target/release:/home/xftroxgpx/.cargo/bin:/opt/depot_tools:/usr/bin/site_perl:/usr/bin/vendor_perl:/usr/bin/core_perl:/home/xftroxgpx/bin:/home/xftroxgpx/build/2nonpkgs/rust.stuff/racer/target/release:/home/xftroxgpx/build/2nonpkgs/rust.stuff/chars/target/debug:/home/xftroxgpx/build/2nonpkgs/rust.stuff/chars/target/release:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rustfmt/target/release:/home/xftroxgpx/.cargo/bin
./go:54+ python2 ./x.py build -vv --incremental -j 5
running: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/bootstrap/Cargo.toml --verbose
       Fresh unicode-xid v0.1.0
       Fresh fixedbitset v0.1.9
       Fresh ordermap v0.3.5
       Fresh cfg-if v0.1.6
       Fresh cc v1.0.28
       Fresh itoa v0.4.3
       Fresh lazy_static v0.2.11
       Fresh getopts v0.2.17
       Fresh build_helper v0.1.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/build_helper)
       Fresh petgraph v0.4.13
       Fresh cmake v0.1.33
       Fresh proc-macro2 v0.4.24
       Fresh ryu v0.2.7
       Fresh serde v1.0.82
       Fresh libc v0.2.46
       Fresh quote v0.6.10
       Fresh toml v0.4.10
       Fresh serde_json v1.0.33
       Fresh num_cpus v1.8.0
       Fresh filetime v0.2.4
       Fresh time v0.1.40
       Fresh syn v0.15.22
       Fresh serde_derive v1.0.81
       Fresh bootstrap v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/bootstrap)
    Finished dev [unoptimized] target(s) in 0.69s
running: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/bootstrap build -vv --incremental -j 5
finding compilers
CC_x86_64-unknown-linux-gnu = "cc"
CFLAGS_x86_64-unknown-linux-gnu = ["-ffunction-sections", "-fdata-sections", "-fPIC", "-m64"]
AR_x86_64-unknown-linux-gnu = "ar"
CXX_x86_64-unknown-linux-gnu = "c++"
running sanity check
learning about cargo
finding compilers
CC_x86_64-unknown-linux-gnu = "cc"
CFLAGS_x86_64-unknown-linux-gnu = ["-ffunction-sections", "-fdata-sections", "-fPIC", "-m64"]
AR_x86_64-unknown-linux-gnu = "ar"
CXX_x86_64-unknown-linux-gnu = "c++"
running sanity check
learning about cargo
> Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  > Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    > Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
    < Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
    > Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      > Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        > Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          > StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          < StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          > Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          < Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          > StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
            > Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
              c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
            < Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
            c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
            c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          < StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        < Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        > TestLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        < TestLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      < Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      > RustcLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      < RustcLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    < Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
    > CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" }
      c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      > Llvm { target: "x86_64-unknown-linux-gnu", emscripten: false }
      < Llvm { target: "x86_64-unknown-linux-gnu", emscripten: false }
    < CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" }
    > Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    < Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    > Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    < Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  < Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  > Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    > Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      > Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        > StartupObjects { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        < StartupObjects { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        > StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
          c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
          c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
          c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        < StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      < Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      > TestLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      < TestLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    < Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    > RustcLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    < RustcLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
  < Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  > CodegenBackend { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" }
    c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Llvm { target: "x86_64-unknown-linux-gnu", emscripten: false }
  < CodegenBackend { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" }
  > Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  < Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
  > Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  < Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
  c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
< Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
> Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  > StartupObjects { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
  < StartupObjects { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
  c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  c Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  > StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
Dirty - /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2-tools
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  < StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
< Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
> Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  c Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  c Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  > TestLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  < TestLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
< Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
> Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  c Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  > RustcLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  < RustcLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
< Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
> CodegenBackend { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" }
  c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  c CodegenBackend { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" }
< CodegenBackend { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" }
> Rustdoc { host: "x86_64-unknown-linux-gnu" }
  c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
Dirty - /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release
  c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
< Rustdoc { host: "x86_64-unknown-linux-gnu" }
> Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  > Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    > Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
    < Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
    > Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      > Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        > Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          > StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          < StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          > Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          < Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "5" "-v" "--release" "--features" "panic-unwind backtrace" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/Cargo.toml" "--message-format" "json"
       Fresh cc v1.0.28
       Fresh core v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libcore)
       Fresh build_helper v0.1.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/build_helper)
       Fresh cmake v0.1.33
       Fresh rustc-std-workspace-core v1.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/tools/rustc-std-workspace-core)
       Fresh libc v0.2.46
       Fresh compiler_builtins v0.1.4
       Fresh alloc v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/liballoc)
       Fresh panic_abort v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libpanic_abort)
       Fresh rustc-demangle v0.1.10
       Fresh unwind v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libunwind)
       Fresh backtrace-sys v0.1.27
       Fresh rustc_asan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_asan)
       Fresh rustc_tsan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_tsan)
       Fresh rustc_msan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_msan)
       Fresh rustc_lsan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_lsan)
       Fresh panic_unwind v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libpanic_unwind)
       Fresh std v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd)
    Finished release [optimized] target(s) in 0.81s
not updating "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/.libstd.stamp"; contents equal and SystemTime { tv_sec: 1547512026, tv_nsec: 805623527 } <= SystemTime { tv_sec: 1547512026, tv_nsec: 911622195 }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          > StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
            > Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
              c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
            < Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
            c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
            c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          < StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 0.903
        < Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "5" "-v" "--release" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libtest/Cargo.toml" "--message-format" "json"
       Fresh proc_macro v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libproc_macro)
       Fresh getopts v0.2.17
       Fresh term v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libterm)
       Fresh test v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libtest)
    Finished release [optimized] target(s) in 0.65s
not updating "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/.libtest.stamp"; contents equal and SystemTime { tv_sec: 1547512058, tv_nsec: 234228416 } <= SystemTime { tv_sec: 1547512058, tv_nsec: 292227687 }
        c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        > TestLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
          c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        < TestLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[TIMING] Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 0.679
      < Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "5" "-v" "--release" "--features" "" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/rustc/Cargo.toml" "--message-format" "json"
       Fresh cfg-if v0.1.6
       Fresh nodrop v0.1.12
       Fresh rand_core v0.3.0
       Fresh lazy_static v1.2.0
       Fresh void v1.0.2
       Fresh memoffset v0.2.1
       Fresh scopeguard v0.3.3
       Fresh stable_deref_trait v1.1.0
       Fresh semver-parser v0.7.0
       Fresh either v1.5.0
       Fresh byteorder v1.2.7
       Fresh bitflags v1.0.4
       Fresh unicode-width v0.1.5
       Fresh graphviz v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libgraphviz)
       Fresh scoped-tls v0.1.2
       Fresh termcolor v1.0.4
       Fresh lazy_static v0.2.11
       Fresh remove_dir_all v0.5.1
       Fresh datafrog v2.0.1
       Fresh rustc-demangle v0.1.10
       Fresh rustc_fs_util v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_fs_util)
       Fresh fmt_macros v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libfmt_macros)
       Fresh rustc-serialize v0.3.24
       Fresh quick-error v1.2.2
       Fresh rustc_platform_intrinsics v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_platform_intrinsics)
       Fresh cc v1.0.28
       Fresh crossbeam-utils v0.2.2
       Fresh log v0.4.6
       Fresh arrayvec v0.4.7
       Fresh rand_core v0.2.2
       Fresh rand_xorshift v0.1.0
       Fresh rand_isaac v0.1.1
       Fresh rand_hc v0.1.0
       Fresh log_settings v0.1.2
       Fresh unreachable v1.0.0
       Fresh owning_ref v0.3.3
       Fresh semver v0.9.0
       Fresh rustc-hash v1.0.1
       Fresh chalk-macros v0.1.0
       Fresh rls-span v0.4.0
       Fresh humantime v1.2.0
       Fresh libc v0.2.46
       Fresh rustc_cratesio_shim v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_cratesio_shim)
       Fresh ena v0.11.0
       Fresh crossbeam-epoch v0.3.1
       Fresh smallvec v0.6.7
       Fresh lock_api v0.1.3
       Fresh rustc_version v0.2.3
       Fresh polonius-engine v0.6.2
       Fresh crc32fast v1.1.2
       Fresh chalk-engine v0.9.0
       Fresh rls-data v0.18.1
       Fresh num_cpus v1.8.0
       Fresh rand v0.4.3
       Fresh rand v0.5.5
       Fresh atty v0.2.11
       Fresh jobserver v0.1.12
       Fresh memmap v0.6.2
       Fresh crossbeam-deque v0.2.0
       Fresh serialize v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libserialize)
       Fresh rustc_apfloat v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_apfloat)
       Fresh miniz-sys v0.1.11
       Fresh backtrace-sys v0.1.27
       Fresh parking_lot_core v0.3.0
       Fresh env_logger v0.5.13
       Fresh rustc-rayon-core v0.1.1
       Fresh flate2 v1.0.6
       Fresh backtrace v0.3.11
       Fresh parking_lot v0.6.4
       Fresh rustc-rayon v0.1.1
       Fresh rand_chacha v0.1.0
       Fresh rand_pcg v0.1.1
       Fresh rustc_data_structures v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_data_structures)
       Fresh rand v0.6.1
       Fresh arena v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libarena)
       Fresh rustc_target v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_target)
       Fresh tempfile v3.0.5
       Fresh syntax_pos v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libsyntax_pos)
       Fresh rustc_errors v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_errors)
       Fresh syntax v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libsyntax)
       Fresh syntax_ext v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libsyntax_ext)
       Fresh rustc v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc)
       Fresh rustc_mir v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_mir)
       Fresh rustc_metadata v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_metadata)
       Fresh rustc_incremental v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_incremental)
       Fresh rustc_typeck v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_typeck)
       Fresh rustc_traits v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_traits)
       Fresh rustc_allocator v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_allocator)
       Fresh rustc_lint v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_lint)
       Fresh rustc_passes v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_passes)
       Fresh rustc_borrowck v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_borrowck)
       Fresh rustc_resolve v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_resolve)
       Fresh rustc_plugin v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_plugin)
       Fresh rustc_codegen_utils v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_utils)
       Fresh rustc_privacy v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_privacy)
       Fresh rustc_save_analysis v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_save_analysis)
       Fresh rustc_codegen_ssa v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_ssa)
       Fresh rustc_driver v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_driver)
       Fresh rustc-main v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/rustc)
    Finished release [optimized] target(s) in 0.79s
not updating "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/.librustc.stamp"; contents equal and SystemTime { tv_sec: 1547513772, tv_nsec: 537676703 } <= SystemTime { tv_sec: 1547513773, tv_nsec: 890659694 }
      c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      > RustcLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
        c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      < RustcLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[TIMING] Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 0.848
    < Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
    > CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" }
      c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      > Llvm { target: "x86_64-unknown-linux-gnu", emscripten: false }
      < Llvm { target: "x86_64-unknown-linux-gnu", emscripten: false }
Building stage0 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
running: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "5" "-v" "--release" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_llvm/Cargo.toml" "--features" "" "--message-format" "json"
       Fresh build_helper v0.1.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/build_helper)
       Fresh rustc-demangle v0.1.10
       Fresh cc v1.0.28
       Fresh libc v0.2.46
       Fresh num_cpus v1.8.0
       Fresh memmap v0.6.2
       Fresh rustc_llvm v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_llvm)
       Fresh rustc_codegen_llvm v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_llvm)
    Finished release [optimized] target(s) in 0.64s
not updating "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/.tmp.stamp"; contents equal and SystemTime { tv_sec: 1547514040, tv_nsec: 671305807 } <= SystemTime { tv_sec: 1547514040, tv_nsec: 813304022 }
[TIMING] CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" } -- 0.678
    < CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" }
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
    > Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    < Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    > Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    < Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[TIMING] Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } } -- 0.406
  < Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  > Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    > Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      > Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        > StartupObjects { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        < StartupObjects { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "5" "-v" "--release" "--features" "panic-unwind backtrace" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/Cargo.toml" "--message-format" "json"
       Fresh cc v1.0.28
       Fresh core v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libcore)
       Fresh build_helper v0.1.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/build_helper)
       Fresh cmake v0.1.33
       Fresh rustc-std-workspace-core v1.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/tools/rustc-std-workspace-core)
       Fresh libc v0.2.46
       Fresh compiler_builtins v0.1.4
       Fresh alloc v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/liballoc)
       Fresh rustc-demangle v0.1.10
       Fresh panic_abort v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libpanic_abort)
       Fresh unwind v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libunwind)
       Fresh backtrace-sys v0.1.27
       Fresh rustc_msan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_msan)
       Fresh rustc_asan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_asan)
       Fresh rustc_tsan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_tsan)
       Fresh rustc_lsan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_lsan)
       Fresh panic_unwind v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libpanic_unwind)
       Fresh std v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd)
    Finished release [optimized] target(s) in 0.68s
not updating "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/.libstd.stamp"; contents equal and SystemTime { tv_sec: 1547514282, tv_nsec: 355267428 } <= SystemTime { tv_sec: 1547514282, tv_nsec: 502265580 }
        c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        > StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
          c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
          c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
          c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
          c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        < StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } } -- 0.753
      < Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
Building stage1 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "5" "-v" "--release" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libtest/Cargo.toml" "--message-format" "json"
       Fresh term v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libterm)
       Fresh proc_macro v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libproc_macro)
       Fresh getopts v0.2.17
       Fresh test v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libtest)
    Finished release [optimized] target(s) in 0.67s
not updating "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/.libtest.stamp"; contents equal and SystemTime { tv_sec: 1547514336, tv_nsec: 763583423 } <= SystemTime { tv_sec: 1547514336, tv_nsec: 834582530 }
      c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      > TestLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copying stage1 test from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
        c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      < TestLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[TIMING] Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } } -- 0.731
    < Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "5" "-v" "--release" "--features" "" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/rustc/Cargo.toml" "--message-format" "json"
       Fresh cfg-if v0.1.6
       Fresh nodrop v0.1.12
       Fresh scopeguard v0.3.3
       Fresh void v1.0.2
       Fresh rand_core v0.3.0
       Fresh lazy_static v1.2.0
       Fresh memoffset v0.2.1
       Fresh stable_deref_trait v1.1.0
       Fresh semver-parser v0.7.0
       Fresh either v1.5.0
       Fresh unicode-width v0.1.5
       Fresh byteorder v1.2.7
       Fresh bitflags v1.0.4
       Fresh graphviz v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libgraphviz)
       Fresh scoped-tls v0.1.2
       Fresh termcolor v1.0.4
       Fresh lazy_static v0.2.11
       Fresh datafrog v2.0.1
       Fresh remove_dir_all v0.5.1
       Fresh rustc-demangle v0.1.10
       Fresh fmt_macros v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libfmt_macros)
       Fresh rustc_fs_util v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_fs_util)
       Fresh rustc-serialize v0.3.24
       Fresh quick-error v1.2.2
       Fresh rustc_platform_intrinsics v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_platform_intrinsics)
       Fresh cc v1.0.28
       Fresh crossbeam-utils v0.2.2
       Fresh log v0.4.6
       Fresh arrayvec v0.4.7
       Fresh unreachable v1.0.0
       Fresh rand_core v0.2.2
       Fresh rand_isaac v0.1.1
       Fresh rand_xorshift v0.1.0
       Fresh rand_hc v0.1.0
       Fresh log_settings v0.1.2
       Fresh owning_ref v0.3.3
       Fresh semver v0.9.0
       Fresh rustc-hash v1.0.1
   Compiling miniz-sys v0.1.11
   Compiling backtrace-sys v0.1.27
     Running `/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/rustc --crate-name build_script_build /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/miniz-sys-0.1.11/build.rs --color always --error-format json --crate-type bin --emit=dep-info,link -C opt-level=2 -C metadata=a29a963360e9048d -C extra-filename=-a29a963360e9048d --out-dir /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-a29a963360e9048d -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern cc=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-fdcbce84d6a0d07b.rlib --cap-lints allow`
     Running `/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/rustc --crate-name build_script_build /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-sys-0.1.27/build.rs --color always --error-format json --crate-type bin --emit=dep-info,link -C opt-level=2 -C metadata=b2c42670f20af394 -C extra-filename=-b2c42670f20af394 --out-dir /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-b2c42670f20af394 -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern cc=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-fdcbce84d6a0d07b.rlib --cap-lints allow`
       Fresh chalk-macros v0.1.0
rustc command: "LD_LIBRARY_PATH"="/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "--crate-name" "build_script_build" "/home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-sys-0.1.27/build.rs" "--color" "always" "--crate-type" "bin" "--emit=dep-info,link" "-C" "opt-level=2" "-C" "metadata=b2c42670f20af394-rustc" "-C" "extra-filename=-b2c42670f20af394" "--out-dir" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-b2c42670f20af394" "-L" "dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps" "--extern" "cc=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-fdcbce84d6a0d07b.rlib" "--cap-lints" "allow" "--cfg" "stage1" "--cfg" "parallel_queries" "-Dwarnings" "-Dbare_trait_objects"
sysroot: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1"
libdir: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib"
   Compiling rls-span v0.4.0
       Fresh humantime v1.2.0
       Fresh rustc_cratesio_shim v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_cratesio_shim)
       Fresh ena v0.11.0
       Fresh crossbeam-epoch v0.3.1
       Fresh libc v0.2.46
     Running `/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/rustc --crate-name rls_span /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/rls-span-0.4.0/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 --cfg 'feature="default"' --cfg 'feature="rustc-serialize"' --cfg 'feature="serialize-rustc"' -C metadata=5b3cac2d721197f6 -C extra-filename=-5b3cac2d721197f6 --out-dir /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern rustc_serialize=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-c507f636a24d78a5.rlib --cap-lints allow -Zverbose`
       Fresh smallvec v0.6.7
rustc command: "LD_LIBRARY_PATH"="/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "--crate-name" "build_script_build" "/home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/miniz-sys-0.1.11/build.rs" "--color" "always" "--crate-type" "bin" "--emit=dep-info,link" "-C" "opt-level=2" "-C" "metadata=a29a963360e9048d-rustc" "-C" "extra-filename=-a29a963360e9048d" "--out-dir" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-a29a963360e9048d" "-L" "dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps" "--extern" "cc=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-fdcbce84d6a0d07b.rlib" "--cap-lints" "allow" "--cfg" "stage1" "--cfg" "parallel_queries" "-Dwarnings" "-Dbare_trait_objects"
sysroot: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1"
libdir: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib"
       Fresh lock_api v0.1.3
rustc command: "LD_LIBRARY_PATH"="/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "--crate-name" "rls_span" "/home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/rls-span-0.4.0/src/lib.rs" "--color" "always" "--crate-type" "lib" "--emit=dep-info,link" "-C" "opt-level=2" "--cfg" "feature=\"default\"" "--cfg" "feature=\"rustc-serialize\"" "--cfg" "feature=\"serialize-rustc\"" "-C" "metadata=5b3cac2d721197f6-rustc" "-C" "extra-filename=-5b3cac2d721197f6" "--out-dir" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps" "--target" "x86_64-unknown-linux-gnu" "-L" "dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps" "--extern" "rustc_serialize=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-c507f636a24d78a5.rlib" "--cap-lints" "allow" "-Zverbose" "--cfg" "stage1" "--sysroot" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1" "-Cprefer-dynamic" "-C" "debug-assertions=n" "-C" "codegen-units=1" "-C" "link-args=-Wl,-rpath,$ORIGIN/../lib" "-Z" "force-unstable-if-unmarked" "--cfg" "parallel_queries" "-Dwarnings" "-Dbare_trait_objects"
sysroot: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1"
libdir: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib"
   Compiling rustc_version v0.2.3
   Compiling polonius-engine v0.6.2
     Running `/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/rustc --crate-name rustc_version /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc_version-0.2.3/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=cb537d0af9e6a756 -C extra-filename=-cb537d0af9e6a756 --out-dir /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern semver=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libsemver-11bbdc35781e5aad.rlib --cap-lints allow`
     Running `/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/rustc --crate-name polonius_engine /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/polonius-engine-0.6.2/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=eec502b05eb50d75 -C extra-filename=-eec502b05eb50d75 --out-dir /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern datafrog=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libdatafrog-905c8c68d748daaf.rlib --extern log=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-2f8969a1a6d486b4.rlib --extern rustc_hash=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hash-312d0b1068ca470d.rlib --cap-lints allow -Zverbose`
rustc command: "LD_LIBRARY_PATH"="/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "--crate-name" "rustc_version" "/home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc_version-0.2.3/src/lib.rs" "--color" "always" "--crate-type" "lib" "--emit=dep-info,link" "-C" "opt-level=2" "-C" "metadata=cb537d0af9e6a756-rustc" "-C" "extra-filename=-cb537d0af9e6a756" "--out-dir" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps" "-L" "dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps" "--extern" "semver=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libsemver-11bbdc35781e5aad.rlib" "--cap-lints" "allow" "--cfg" "stage1" "--cfg" "parallel_queries" "-Dwarnings" "-Dbare_trait_objects"
sysroot: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1"
libdir: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib"
rustc command: "LD_LIBRARY_PATH"="/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "--crate-name" "polonius_engine" "/home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/polonius-engine-0.6.2/src/lib.rs" "--color" "always" "--crate-type" "lib" "--emit=dep-info,link" "-C" "opt-level=2" "-C" "metadata=eec502b05eb50d75-rustc" "-C" "extra-filename=-eec502b05eb50d75" "--out-dir" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps" "--target" "x86_64-unknown-linux-gnu" "-L" "dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps" "--extern" "datafrog=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libdatafrog-905c8c68d748daaf.rlib" "--extern" "log=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-2f8969a1a6d486b4.rlib" "--extern" "rustc_hash=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hash-312d0b1068ca470d.rlib" "--cap-lints" "allow" "-Zverbose" "--cfg" "stage1" "--sysroot" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1" "-Cprefer-dynamic" "-C" "debug-assertions=n" "-C" "codegen-units=1" "-C" "link-args=-Wl,-rpath,$ORIGIN/../lib" "-Z" "force-unstable-if-unmarked" "--cfg" "parallel_queries" "-Dwarnings" "-Dbare_trait_objects"
sysroot: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1"
libdir: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib"
[RUSTC-TIMING] rls_span test:false 4.389
   Compiling crc32fast v1.1.2
     Running `/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/rustc --crate-name crc32fast /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/crc32fast-1.1.2/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=0c36298f365bf194 -C extra-filename=-0c36298f365bf194 --out-dir /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern cfg_if=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libcfg_if-618399a6bdb38742.rlib --cap-lints allow -Zverbose --cfg crc32fast_stdarchx86`
rustc command: "LD_LIBRARY_PATH"="/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "--crate-name" "crc32fast" "/home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/crc32fast-1.1.2/src/lib.rs" "--color" "always" "--crate-type" "lib" "--emit=dep-info,link" "-C" "opt-level=2" "-C" "metadata=0c36298f365bf194-rustc" "-C" "extra-filename=-0c36298f365bf194" "--out-dir" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps" "--target" "x86_64-unknown-linux-gnu" "-L" "dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps" "--extern" "cfg_if=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libcfg_if-618399a6bdb38742.rlib" "--cap-lints" "allow" "-Zverbose" "--cfg" "crc32fast_stdarchx86" "--cfg" "stage1" "--sysroot" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1" "-Cprefer-dynamic" "-C" "debug-assertions=n" "-C" "codegen-units=1" "-C" "link-args=-Wl,-rpath,$ORIGIN/../lib" "-Z" "force-unstable-if-unmarked" "--cfg" "parallel_queries" "-Dwarnings" "-Dbare_trait_objects"
sysroot: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1"
libdir: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib"
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-a29a963360e9048d/build_script_build-a29a963360e9048d.build_script_build.uljpnif4-cgu.0.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-a29a963360e9048d/build_script_build-a29a963360e9048d.build_script_build.uljpnif4-cgu.1.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-a29a963360e9048d/build_script_build-a29a963360e9048d.build_script_build.uljpnif4-cgu.10.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-a29a963360e9048d/build_script_build-a29a963360e9048d.build_script_build.uljpnif4-cgu.11.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-a29a963360e9048d/build_script_build-a29a963360e9048d.build_script_build.uljpnif4-cgu.12.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-a29a963360e9048d/build_script_build-a29a963360e9048d.build_script_build.uljpnif4-cgu.13.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-a29a963360e9048d/build_script_build-a29a963360e9048d.build_script_build.uljpnif4-cgu.14.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-a29a963360e9048d/build_script_build-a29a963360e9048d.build_script_build.uljpnif4-cgu.15.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-a29a963360e9048d/build_script_build-a29a963360e9048d.build_script_build.uljpnif4-cgu.2.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-a29a963360e9048d/build_script_build-a29a963360e9048d.build_script_build.uljpnif4-cgu.3.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-a29a963360e9048d/build_script_build-a29a963360e9048d.build_script_build.uljpnif4-cgu.4.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-a29a963360e9048d/build_script_build-a29a963360e9048d.build_script_build.uljpnif4-cgu.5.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-a29a963360e9048d/build_script_build-a29a963360e9048d.build_script_build.uljpnif4-cgu.6.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-a29a963360e9048d/build_script_build-a29a963360e9048d.build_script_build.uljpnif4-cgu.7.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-a29a963360e9048d/build_script_build-a29a963360e9048d.build_script_build.uljpnif4-cgu.8.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-a29a963360e9048d/build_script_build-a29a963360e9048d.build_script_build.uljpnif4-cgu.9.rcgu.o" "-o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-a29a963360e9048d/build_script_build-a29a963360e9048d" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-a29a963360e9048d/build_script_build-a29a963360e9048d.4f402igejrkm5gad.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-fdcbce84d6a0d07b.rlib" "-Wl,--start-group" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-758c3eb8e584903c.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-93bb00b0b988d3b6.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace_sys-b7bea45b82f77e09.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-1ef73c7606af9f89.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-199a8d6bb5ab7660.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-d567cc3ebeb54e37.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-703b665cf617ceb6.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-fc315886a647a5ae.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-58cd2ebb7fab4d14.rlib" "-Wl,--end-group" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-052ef173b8c848e0.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil"
  = note: /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-fdcbce84d6a0d07b.rlib(cc-fdcbce84d6a0d07b.cc.72mdxmxi-cgu.0.rcgu.o): in function `cc::Build::try_compile':
          cc.72mdxmxi-cgu.0:(.text._ZN2cc5Build11try_compile17h1f7796efa49d777eE+0x409f): undefined reference to `<std::ffi::os_str::OsStr as core::cmp::PartialEq>::eq'
          /usr/bin/ld: cc.72mdxmxi-cgu.0:(.text._ZN2cc5Build11try_compile17h1f7796efa49d777eE+0x4274): undefined reference to `<std::env::SplitPaths<'a> as core::iter::iterator::Iterator>::next'
          /usr/bin/ld: cc.72mdxmxi-cgu.0:(.text._ZN2cc5Build11try_compile17h1f7796efa49d777eE+0x43db): undefined reference to `<std::path::Display<'a> as core::fmt::Display>::fmt'
          /usr/bin/ld: cc.72mdxmxi-cgu.0:(.text._ZN2cc5Build11try_compile17h1f7796efa49d777eE+0x46a0): undefined reference to `<std::path::Display<'a> as core::fmt::Display>::fmt'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-fdcbce84d6a0d07b.rlib(cc-fdcbce84d6a0d07b.cc.72mdxmxi-cgu.0.rcgu.o): in function `cc::Build::get_base_compiler':
          cc.72mdxmxi-cgu.0:(.text._ZN2cc5Build17get_base_compiler17h3e354b456eb9af2aE+0x1a7d): undefined reference to `<std::path::Display<'a> as core::fmt::Display>::fmt'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-fdcbce84d6a0d07b.rlib(cc-fdcbce84d6a0d07b.cc.72mdxmxi-cgu.10.rcgu.o): in function `<alloc::boxed::Box<T> as core::fmt::Debug>::fmt':
          cc.72mdxmxi-cgu.10:(.text._ZN63_$LT$alloc..boxed..Box$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hd5a4febe9097ef77E+0xf): undefined reference to `<(dyn core::any::Any + core::marker::Send + 'static) as core::fmt::Debug>::fmt'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-fdcbce84d6a0d07b.rlib(cc-fdcbce84d6a0d07b.cc.72mdxmxi-cgu.11.rcgu.o): in function `<std::io::buffered::BufWriter<W> as core::ops::drop::Drop>::drop':
          cc.72mdxmxi-cgu.11:(.text._ZN79_$LT$std..io..buffered..BufWriter$LT$W$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hf64e2e5ea66f59eeE+0x142): undefined reference to `std::error::<impl core::convert::From<&'b str> for alloc::boxed::Box<(dyn std::error::Error + core::marker::Send + core::marker::Sync + 'a)>>::from'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-fdcbce84d6a0d07b.rlib(cc-fdcbce84d6a0d07b.cc.72mdxmxi-cgu.13.rcgu.o): in function `std::io::Write::write_all':
          cc.72mdxmxi-cgu.13:(.text._ZN3std2io5Write9write_all17h4dd8b8c82c8ce99fE+0x12d): undefined reference to `std::error::<impl core::convert::From<&'b str> for alloc::boxed::Box<(dyn std::error::Error + core::marker::Send + core::marker::Sync + 'a)>>::from'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-fdcbce84d6a0d07b.rlib(cc-fdcbce84d6a0d07b.cc.72mdxmxi-cgu.13.rcgu.o): in function `std::io::Write::write_fmt':
          cc.72mdxmxi-cgu.13:(.text._ZN3std2io5Write9write_fmt17hbe6be704f54c5273E+0x62): undefined reference to `std::error::<impl core::convert::From<&'b str> for alloc::boxed::Box<(dyn std::error::Error + core::marker::Send + core::marker::Sync + 'a)>>::from'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-fdcbce84d6a0d07b.rlib(cc-fdcbce84d6a0d07b.cc.72mdxmxi-cgu.15.rcgu.o): in function `core::slice::<impl [T]>::copy_from_slice':
          cc.72mdxmxi-cgu.15:(.text._ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$15copy_from_slice17h6f34130da1434152E+0xb8): undefined reference to `<core::fmt::Arguments<'_> as core::fmt::Display>::fmt'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-fdcbce84d6a0d07b.rlib(cc-fdcbce84d6a0d07b.cc.72mdxmxi-cgu.15.rcgu.o): in function `core::slice::<impl [T]>::contains':
          cc.72mdxmxi-cgu.15:(.text._ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$8contains17h595b9c28f0b08d6fE+0x19): undefined reference to `<std::ffi::os_str::OsString as core::cmp::PartialEq>::eq'
          collect2: error: ld returned 1 exit status
          

error: aborting due to previous error

error: Could not compile `miniz-sys`.

Caused by:
  process didn't exit successfully: `/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/rustc --crate-name build_script_build /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/miniz-sys-0.1.11/build.rs --color always --error-format json --crate-type bin --emit=dep-info,link -C opt-level=2 -C metadata=a29a963360e9048d -C extra-filename=-a29a963360e9048d --out-dir /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-a29a963360e9048d -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern cc=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-fdcbce84d6a0d07b.rlib --cap-lints allow` (exit code: 1)
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] polonius_engine test:false 5.186                       ] 66/128
error: linking with `cc` failed: exit code: 1                         ] 68/128
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-b2c42670f20af394/build_script_build-b2c42670f20af394.build_script_build.1xo2eyrc-cgu.0.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-b2c42670f20af394/build_script_build-b2c42670f20af394.build_script_build.1xo2eyrc-cgu.1.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-b2c42670f20af394/build_script_build-b2c42670f20af394.build_script_build.1xo2eyrc-cgu.10.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-b2c42670f20af394/build_script_build-b2c42670f20af394.build_script_build.1xo2eyrc-cgu.11.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-b2c42670f20af394/build_script_build-b2c42670f20af394.build_script_build.1xo2eyrc-cgu.12.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-b2c42670f20af394/build_script_build-b2c42670f20af394.build_script_build.1xo2eyrc-cgu.13.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-b2c42670f20af394/build_script_build-b2c42670f20af394.build_script_build.1xo2eyrc-cgu.14.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-b2c42670f20af394/build_script_build-b2c42670f20af394.build_script_build.1xo2eyrc-cgu.15.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-b2c42670f20af394/build_script_build-b2c42670f20af394.build_script_build.1xo2eyrc-cgu.2.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-b2c42670f20af394/build_script_build-b2c42670f20af394.build_script_build.1xo2eyrc-cgu.3.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-b2c42670f20af394/build_script_build-b2c42670f20af394.build_script_build.1xo2eyrc-cgu.4.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-b2c42670f20af394/build_script_build-b2c42670f20af394.build_script_build.1xo2eyrc-cgu.5.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-b2c42670f20af394/build_script_build-b2c42670f20af394.build_script_build.1xo2eyrc-cgu.6.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-b2c42670f20af394/build_script_build-b2c42670f20af394.build_script_build.1xo2eyrc-cgu.7.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-b2c42670f20af394/build_script_build-b2c42670f20af394.build_script_build.1xo2eyrc-cgu.8.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-b2c42670f20af394/build_script_build-b2c42670f20af394.build_script_build.1xo2eyrc-cgu.9.rcgu.o" "-o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-b2c42670f20af394/build_script_build-b2c42670f20af394" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-b2c42670f20af394/build_script_build-b2c42670f20af394.bv6jzgxbofo9cdx.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-fdcbce84d6a0d07b.rlib" "-Wl,--start-group" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-758c3eb8e584903c.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-93bb00b0b988d3b6.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace_sys-b7bea45b82f77e09.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-1ef73c7606af9f89.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-199a8d6bb5ab7660.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-d567cc3ebeb54e37.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-703b665cf617ceb6.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-fc315886a647a5ae.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-58cd2ebb7fab4d14.rlib" "-Wl,--end-group" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-052ef173b8c848e0.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil"
  = note: /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-fdcbce84d6a0d07b.rlib(cc-fdcbce84d6a0d07b.cc.72mdxmxi-cgu.0.rcgu.o): in function `cc::Build::try_compile':
          cc.72mdxmxi-cgu.0:(.text._ZN2cc5Build11try_compile17h1f7796efa49d777eE+0x409f): undefined reference to `<std::ffi::os_str::OsStr as core::cmp::PartialEq>::eq'
          /usr/bin/ld: cc.72mdxmxi-cgu.0:(.text._ZN2cc5Build11try_compile17h1f7796efa49d777eE+0x4274): undefined reference to `<std::env::SplitPaths<'a> as core::iter::iterator::Iterator>::next'
          /usr/bin/ld: cc.72mdxmxi-cgu.0:(.text._ZN2cc5Build11try_compile17h1f7796efa49d777eE+0x43db): undefined reference to `<std::path::Display<'a> as core::fmt::Display>::fmt'
          /usr/bin/ld: cc.72mdxmxi-cgu.0:(.text._ZN2cc5Build11try_compile17h1f7796efa49d777eE+0x46a0): undefined reference to `<std::path::Display<'a> as core::fmt::Display>::fmt'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-fdcbce84d6a0d07b.rlib(cc-fdcbce84d6a0d07b.cc.72mdxmxi-cgu.0.rcgu.o): in function `cc::Build::get_base_compiler':
          cc.72mdxmxi-cgu.0:(.text._ZN2cc5Build17get_base_compiler17h3e354b456eb9af2aE+0x1a7d): undefined reference to `<std::path::Display<'a> as core::fmt::Display>::fmt'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-fdcbce84d6a0d07b.rlib(cc-fdcbce84d6a0d07b.cc.72mdxmxi-cgu.10.rcgu.o): in function `<alloc::boxed::Box<T> as core::fmt::Debug>::fmt':
          cc.72mdxmxi-cgu.10:(.text._ZN63_$LT$alloc..boxed..Box$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hd5a4febe9097ef77E+0xf): undefined reference to `<(dyn core::any::Any + core::marker::Send + 'static) as core::fmt::Debug>::fmt'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-fdcbce84d6a0d07b.rlib(cc-fdcbce84d6a0d07b.cc.72mdxmxi-cgu.11.rcgu.o): in function `<std::io::buffered::BufWriter<W> as core::ops::drop::Drop>::drop':
          cc.72mdxmxi-cgu.11:(.text._ZN79_$LT$std..io..buffered..BufWriter$LT$W$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hf64e2e5ea66f59eeE+0x142): undefined reference to `std::error::<impl core::convert::From<&'b str> for alloc::boxed::Box<(dyn std::error::Error + core::marker::Send + core::marker::Sync + 'a)>>::from'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-fdcbce84d6a0d07b.rlib(cc-fdcbce84d6a0d07b.cc.72mdxmxi-cgu.13.rcgu.o): in function `std::io::Write::write_all':
          cc.72mdxmxi-cgu.13:(.text._ZN3std2io5Write9write_all17h4dd8b8c82c8ce99fE+0x12d): undefined reference to `std::error::<impl core::convert::From<&'b str> for alloc::boxed::Box<(dyn std::error::Error + core::marker::Send + core::marker::Sync + 'a)>>::from'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-fdcbce84d6a0d07b.rlib(cc-fdcbce84d6a0d07b.cc.72mdxmxi-cgu.13.rcgu.o): in function `std::io::Write::write_fmt':
          cc.72mdxmxi-cgu.13:(.text._ZN3std2io5Write9write_fmt17hbe6be704f54c5273E+0x62): undefined reference to `std::error::<impl core::convert::From<&'b str> for alloc::boxed::Box<(dyn std::error::Error + core::marker::Send + core::marker::Sync + 'a)>>::from'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-fdcbce84d6a0d07b.rlib(cc-fdcbce84d6a0d07b.cc.72mdxmxi-cgu.15.rcgu.o): in function `core::slice::<impl [T]>::copy_from_slice':
          cc.72mdxmxi-cgu.15:(.text._ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$15copy_from_slice17h6f34130da1434152E+0xb8): undefined reference to `<core::fmt::Arguments<'_> as core::fmt::Display>::fmt'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-fdcbce84d6a0d07b.rlib(cc-fdcbce84d6a0d07b.cc.72mdxmxi-cgu.15.rcgu.o): in function `core::slice::<impl [T]>::contains':
          cc.72mdxmxi-cgu.15:(.text._ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$8contains17h595b9c28f0b08d6fE+0x19): undefined reference to `<std::ffi::os_str::OsString as core::cmp::PartialEq>::eq'
          collect2: error: ld returned 1 exit status
          

error: aborting due to previous error

error: Could not compile `backtrace-sys`.

Caused by:
  process didn't exit successfully: `/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/rustc --crate-name build_script_build /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-sys-0.1.27/build.rs --color always --error-format json --crate-type bin --emit=dep-info,link -C opt-level=2 -C metadata=b2c42670f20af394 -C extra-filename=-b2c42670f20af394 --out-dir /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-b2c42670f20af394 -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern cc=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-fdcbce84d6a0d07b.rlib --cap-lints allow` (exit code: 1)
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] crc32fast test:false 1.591=>                           ] 68/128
error: build failed
command did not execute successfully: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "5" "-v" "--release" "--features" "" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/rustc/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101
Traceback (most recent call last):
  File "./x.py", line 11, in <module>
    bootstrap.main()
  File "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/bootstrap/bootstrap.py", line 843, in main
    bootstrap(help_triggered)
  File "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/bootstrap/bootstrap.py", line 829, in bootstrap
    run(args, env=env, verbose=build.verbose)
  File "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/bootstrap/bootstrap.py", line 141, in run
    raise RuntimeError(err)
RuntimeError: failed to run: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/bootstrap build -vv --incremental -j 5

real	0m21.573s
user	0m34.076s
sys	0m1.760s


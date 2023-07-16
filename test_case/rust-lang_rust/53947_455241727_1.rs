rust
/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust 
$ ./go
~/build/2nonpkgs/rust.stuff/rust/rust ~/build/2nonpkgs/rust.stuff/rust
./go:47+ set -xev
#export RUSTFLAGS="-Z verbose"
#export RUSTFLAGS="-Zverbose"
#^ https://github.com/rust-lang/rust/issues/57596
verbose='-vv'
./go:51+ verbose=-vv
#verbose=''
#threads=5 #running out of memory with 2G autofree and le9d.patch and 5 threads here
#threads=1 #too slow!230+44mins!
threads=4 #will test
./go:55+ threads=4

time PATH="${HOME}/build/1packages/4used/chro/4_chromium-dev-git/makepkg_pacman/chromium-dev/src/python-path/:${PATH}" python2 ./x.py build $verbose --incremental -j $threads
./go:57+ PATH=/home/xftroxgpx/build/1packages/4used/chro/4_chromium-dev-git/makepkg_pacman/chromium-dev/src/python-path/:/opt/google-cloud-sdk/bin:/opt/google-cloud-sdk/bin:/usr/local/sbin:/usr/local/bin:/usr/bin:/opt/depot_tools:/usr/bin/site_perl:/usr/bin/vendor_perl:/usr/bin/core_perl:/home/xftroxgpx/bin:/home/xftroxgpx/build/2nonpkgs/rust.stuff/racer/target/release:/home/xftroxgpx/build/2nonpkgs/rust.stuff/chars/target/debug:/home/xftroxgpx/build/2nonpkgs/rust.stuff/chars/target/release:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rustfmt/target/release:/home/xftroxgpx/.cargo/bin:/opt/depot_tools:/usr/bin/site_perl:/usr/bin/vendor_perl:/usr/bin/core_perl:/home/xftroxgpx/bin:/home/xftroxgpx/build/2nonpkgs/rust.stuff/racer/target/release:/home/xftroxgpx/build/2nonpkgs/rust.stuff/chars/target/debug:/home/xftroxgpx/build/2nonpkgs/rust.stuff/chars/target/release:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rustfmt/target/release:/home/xftroxgpx/.cargo/bin
./go:57+ python2 ./x.py build -vv --incremental -j 4
Updating only changed submodules
Submodules updated in 0.06 seconds
running: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/bootstrap/Cargo.toml --verbose
       Fresh unicode-xid v0.1.0
       Fresh fixedbitset v0.1.9
       Fresh ordermap v0.3.5
       Fresh cc v1.0.28
       Fresh cfg-if v0.1.6
       Fresh itoa v0.4.3
       Fresh build_helper v0.1.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/build_helper)
       Fresh lazy_static v0.2.11
       Fresh getopts v0.2.17
       Fresh petgraph v0.4.13
       Fresh cmake v0.1.33
       Fresh proc-macro2 v0.4.24
       Fresh libc v0.2.46
       Fresh serde v1.0.82
       Fresh ryu v0.2.7
       Fresh quote v0.6.10
       Fresh filetime v0.2.4
       Fresh time v0.1.40
       Fresh num_cpus v1.8.0
       Fresh toml v0.4.10
       Fresh serde_json v1.0.33
       Fresh syn v0.15.22
       Fresh serde_derive v1.0.81
       Fresh bootstrap v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/bootstrap)
    Finished dev [unoptimized] target(s) in 0.67s
running: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/bootstrap build -vv --incremental -j 4
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
          > Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
            > Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
            < Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          < Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          > StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
            c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
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
        c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
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
  c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
  > StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
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
          > Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
            > Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
            < Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          < Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "-v" "--release" "--features" "panic-unwind backtrace" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/Cargo.toml" "--message-format" "json"
       Fresh cc v1.0.28
       Fresh core v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libcore)
       Fresh build_helper v0.1.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/build_helper)
       Fresh cmake v0.1.33
       Fresh rustc-std-workspace-core v1.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/tools/rustc-std-workspace-core)
       Fresh libc v0.2.46
       Fresh compiler_builtins v0.1.4
       Fresh alloc v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/liballoc)
       Fresh unwind v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libunwind)
       Fresh panic_abort v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libpanic_abort)
       Fresh rustc-demangle v0.1.10
       Fresh backtrace-sys v0.1.27
       Fresh rustc_msan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_msan)
       Fresh rustc_tsan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_tsan)
       Fresh rustc_asan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_asan)
       Fresh rustc_lsan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_lsan)
       Fresh panic_unwind v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libpanic_unwind)
       Fresh std v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd)
    Finished release [optimized] target(s) in 0.71s
not updating "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/.libstd.stamp"; contents equal and SystemTime { tv_sec: 1547733897, tv_nsec: 647240705 } <= SystemTime { tv_sec: 1547733897, tv_nsec: 801238769 }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          > StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
            c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
            c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
            c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          < StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 0.794
        < Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "-v" "--release" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libtest/Cargo.toml" "--message-format" "json"
       Fresh getopts v0.2.17
       Fresh term v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libterm)
       Fresh proc_macro v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libproc_macro)
       Fresh test v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libtest)
    Finished release [optimized] target(s) in 0.62s
not updating "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/.libtest.stamp"; contents equal and SystemTime { tv_sec: 1547733920, tv_nsec: 136957971 } <= SystemTime { tv_sec: 1547733920, tv_nsec: 188957317 }
        c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        > TestLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
          c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        < TestLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[TIMING] Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 0.686
      < Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "-v" "--release" "--features" "" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/rustc/Cargo.toml" "--message-format" "json"
       Fresh cfg-if v0.1.6
       Fresh nodrop v0.1.12
       Fresh void v1.0.2
       Fresh lazy_static v1.2.0
       Fresh memoffset v0.2.1
       Fresh scopeguard v0.3.3
       Fresh rand_core v0.3.0
       Fresh stable_deref_trait v1.1.0
       Fresh semver-parser v0.7.0
       Fresh byteorder v1.2.7
       Fresh either v1.5.0
       Fresh unicode-width v0.1.5
       Fresh bitflags v1.0.4
       Fresh graphviz v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libgraphviz)
       Fresh scoped-tls v0.1.2
       Fresh termcolor v1.0.4
       Fresh lazy_static v0.2.11
       Fresh rustc-demangle v0.1.10
       Fresh remove_dir_all v0.5.1
       Fresh datafrog v2.0.1
       Fresh fmt_macros v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libfmt_macros)
       Fresh rustc_fs_util v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_fs_util)
       Fresh rustc-serialize v0.3.24
       Fresh quick-error v1.2.2
       Fresh cc v1.0.28
       Fresh crossbeam-utils v0.2.2
       Fresh log v0.4.6
       Fresh arrayvec v0.4.7
       Fresh unreachable v1.0.0
       Fresh log_settings v0.1.2
       Fresh rand_core v0.2.2
       Fresh rand_isaac v0.1.1
       Fresh rand_xorshift v0.1.0
       Fresh rand_hc v0.1.0
       Fresh owning_ref v0.3.3
       Fresh semver v0.9.0
       Fresh rustc-hash v1.0.1
       Fresh chalk-macros v0.1.0
       Fresh rls-span v0.4.0
       Fresh humantime v1.2.0
       Fresh rustc_cratesio_shim v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_cratesio_shim)
       Fresh ena v0.11.0
       Fresh libc v0.2.46
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
       Fresh backtrace-sys v0.1.27
       Fresh miniz-sys v0.1.11
       Fresh parking_lot_core v0.3.0
       Fresh env_logger v0.5.13
       Fresh rustc-rayon-core v0.1.1
       Fresh backtrace v0.3.11
       Fresh flate2 v1.0.6
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
       Fresh rustc_incremental v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_incremental)
       Fresh rustc_metadata v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_metadata)
       Fresh rustc_typeck v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_typeck)
       Fresh rustc_lint v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_lint)
       Fresh rustc_allocator v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_allocator)
       Fresh rustc_traits v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_traits)
       Fresh rustc_borrowck v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_borrowck)
       Fresh rustc_passes v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_passes)
       Fresh rustc_codegen_utils v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_utils)
       Fresh rustc_plugin v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_plugin)
       Fresh rustc_resolve v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_resolve)
       Fresh rustc_privacy v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_privacy)
       Fresh rustc_save_analysis v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_save_analysis)
       Fresh rustc_codegen_ssa v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_ssa)
       Fresh rustc_driver v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_driver)
       Fresh rustc-main v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/rustc)
    Finished release [optimized] target(s) in 0.75s
not updating "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/.librustc.stamp"; contents equal and SystemTime { tv_sec: 1547736456, tv_nsec: 183075554 } <= SystemTime { tv_sec: 1547736457, tv_nsec: 426059927 }
      c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      > RustcLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
        c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      < RustcLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[TIMING] Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 0.807
    < Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
    > CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" }
      c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      > Llvm { target: "x86_64-unknown-linux-gnu", emscripten: false }
      < Llvm { target: "x86_64-unknown-linux-gnu", emscripten: false }
Building stage0 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
running: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "-v" "--release" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_llvm/Cargo.toml" "--features" "" "--message-format" "json"
       Fresh build_helper v0.1.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/build_helper)
       Fresh cc v1.0.28
       Fresh rustc-demangle v0.1.10
       Fresh libc v0.2.46
       Fresh rustc_llvm v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_llvm)
       Fresh num_cpus v1.8.0
       Fresh memmap v0.6.2
       Fresh rustc_codegen_llvm v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_llvm)
    Finished release [optimized] target(s) in 0.64s
not updating "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/.tmp.stamp"; contents equal and SystemTime { tv_sec: 1547736663, tv_nsec: 535468781 } <= SystemTime { tv_sec: 1547736663, tv_nsec: 665467147 }
[TIMING] CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" } -- 0.683
    < CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" }
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
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
        c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "-v" "--release" "--features" "panic-unwind backtrace" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/Cargo.toml" "--message-format" "json"
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
       Fresh rustc_tsan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_tsan)
       Fresh rustc_asan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_asan)
       Fresh rustc_msan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_msan)
       Fresh rustc_lsan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_lsan)
       Fresh panic_unwind v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libpanic_unwind)
       Fresh std v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd)
    Finished release [optimized] target(s) in 0.71s
not updating "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/.libstd.stamp"; contents equal and SystemTime { tv_sec: 1547736800, tv_nsec: 714744204 } <= SystemTime { tv_sec: 1547736800, tv_nsec: 831742733 }
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
[TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } } -- 0.789
      < Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
Building stage1 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "-v" "--release" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libtest/Cargo.toml" "--message-format" "json"
       Fresh term v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libterm)
       Fresh proc_macro v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libproc_macro)
       Fresh getopts v0.2.17
       Fresh test v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libtest)
    Finished release [optimized] target(s) in 0.63s
not updating "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/.libtest.stamp"; contents equal and SystemTime { tv_sec: 1547736829, tv_nsec: 101387336 } <= SystemTime { tv_sec: 1547736829, tv_nsec: 159386606 }
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
[TIMING] Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } } -- 0.660
    < Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "-v" "--release" "--features" "" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/rustc/Cargo.toml" "--message-format" "json"
       Fresh cfg-if v0.1.6
       Fresh nodrop v0.1.12
       Fresh memoffset v0.2.1
       Fresh scopeguard v0.3.3
       Fresh void v1.0.2
       Fresh rand_core v0.3.0
       Fresh lazy_static v1.2.0
       Fresh stable_deref_trait v1.1.0
       Fresh semver-parser v0.7.0
       Fresh bitflags v1.0.4
       Fresh unicode-width v0.1.5
       Fresh byteorder v1.2.7
       Fresh either v1.5.0
       Fresh graphviz v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libgraphviz)
       Fresh scoped-tls v0.1.2
       Fresh lazy_static v0.2.11
       Fresh termcolor v1.0.4
       Fresh remove_dir_all v0.5.1
       Fresh datafrog v2.0.1
       Fresh rustc-demangle v0.1.10
       Fresh rustc_fs_util v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_fs_util)
       Fresh fmt_macros v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libfmt_macros)
       Fresh rustc-serialize v0.3.24
       Fresh quick-error v1.2.2
       Fresh cc v1.0.28
       Fresh crossbeam-utils v0.2.2
       Fresh log v0.4.6
       Fresh arrayvec v0.4.7
       Fresh unreachable v1.0.0
       Fresh rand_core v0.2.2
       Fresh rand_hc v0.1.0
       Fresh rand_xorshift v0.1.0
       Fresh rand_isaac v0.1.1
       Fresh log_settings v0.1.2
       Fresh owning_ref v0.3.3
       Fresh semver v0.9.0
       Fresh rustc-hash v1.0.1
       Fresh chalk-macros v0.1.0
       Fresh rls-span v0.4.0
       Fresh humantime v1.2.0
       Fresh rustc_cratesio_shim v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_cratesio_shim)
       Fresh ena v0.11.0
       Fresh libc v0.2.46
       Fresh crossbeam-epoch v0.3.1
       Fresh smallvec v0.6.7
       Fresh lock_api v0.1.3
       Fresh rustc_version v0.2.3
       Fresh polonius-engine v0.6.2
       Fresh crc32fast v1.1.2
       Fresh chalk-engine v0.9.0
       Fresh rls-data v0.18.1
       Fresh rand v0.4.3
       Fresh num_cpus v1.8.0
       Fresh rand v0.5.5
       Fresh atty v0.2.11
       Fresh jobserver v0.1.12
       Fresh memmap v0.6.2
       Fresh crossbeam-deque v0.2.0
       Fresh serialize v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libserialize)
       Fresh rustc_apfloat v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_apfloat)
       Fresh backtrace-sys v0.1.27
       Fresh miniz-sys v0.1.11
       Fresh parking_lot_core v0.3.0
       Fresh env_logger v0.5.13
       Fresh rustc-rayon-core v0.1.1
       Fresh backtrace v0.3.11
       Fresh flate2 v1.0.6
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
       Fresh rustc_allocator v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_allocator)
       Fresh rustc_traits v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_traits)
       Fresh rustc_lint v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_lint)
       Fresh rustc_passes v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_passes)
       Fresh rustc_borrowck v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_borrowck)
       Fresh rustc_plugin v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_plugin)
       Fresh rustc_resolve v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_resolve)
       Fresh rustc_codegen_utils v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_utils)
       Fresh rustc_privacy v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_privacy)
       Fresh rustc_save_analysis v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_save_analysis)
       Fresh rustc_codegen_ssa v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_ssa)
       Fresh rustc_driver v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_driver)
       Fresh rustc-main v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/rustc)
    Finished release [optimized] target(s) in 0.76s
not updating "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/.librustc.stamp"; contents equal and SystemTime { tv_sec: 1547739978, tv_nsec: 648792163 } <= SystemTime { tv_sec: 1547739979, tv_nsec: 985775354 }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    > RustcLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copying stage1 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
      c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    < RustcLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[TIMING] Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } } -- 0.821
  < Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  > CodegenBackend { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" }
    c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Llvm { target: "x86_64-unknown-linux-gnu", emscripten: false }
Building stage1 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
running: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "-v" "--release" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_llvm/Cargo.toml" "--features" "" "--message-format" "json"
       Fresh build_helper v0.1.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/build_helper)
       Fresh cc v1.0.28
       Fresh rustc-demangle v0.1.10
       Fresh libc v0.2.46
       Fresh memmap v0.6.2
       Fresh num_cpus v1.8.0
       Fresh rustc_llvm v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_llvm)
       Fresh rustc_codegen_llvm v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_llvm)
    Finished release [optimized] target(s) in 0.64s
not updating "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-codegen/x86_64-unknown-linux-gnu/release/.tmp.stamp"; contents equal and SystemTime { tv_sec: 1547740243, tv_nsec: 59468070 } <= SystemTime { tv_sec: 1547740243, tv_nsec: 288465191 }
[TIMING] CodegenBackend { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" } -- 0.669
  < CodegenBackend { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" }
Assembling stage2 compiler (x86_64-unknown-linux-gnu)
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
Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
  c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
  > StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
    c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
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
Uplifting stage1 test (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
  c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  > TestLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copying stage2 test from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
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
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
  c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  > RustcLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
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
  c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
Building rustdoc for stage2 (x86_64-unknown-linux-gnu)
running: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "-v" "--release" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/tools/rustdoc/Cargo.toml"
       Fresh semver-parser v0.7.0
       Fresh void v1.0.2
       Fresh rand_core v0.3.0
       Fresh stable_deref_trait v1.1.0
       Fresh scopeguard v0.3.3
       Fresh cfg-if v0.1.6
       Fresh macro-utils v0.1.2
       Fresh bitflags v0.9.1
       Fresh remove_dir_all v0.5.1
       Fresh semver v0.9.0
       Fresh unreachable v1.0.0
       Fresh rand_core v0.2.2
       Fresh rand_xorshift v0.1.0
       Fresh rand_isaac v0.1.1
       Fresh rand_hc v0.1.0
       Fresh owning_ref v0.3.3
       Fresh minifier v0.0.26
       Fresh rustc_version v0.2.3
       Fresh libc v0.2.46
       Fresh smallvec v0.6.7
       Fresh lock_api v0.1.3
       Fresh pulldown-cmark v0.1.2
       Fresh rand v0.5.5
       Fresh parking_lot_core v0.3.0
       Fresh rand_pcg v0.1.1
       Fresh rand_chacha v0.1.0
       Fresh parking_lot v0.6.4
       Fresh rand v0.6.1
       Fresh tempfile v3.0.5
       Fresh rustdoc v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustdoc)
       Fresh rustdoc-tool v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/tools/rustdoc)
    Finished release [optimized] target(s) in 0.67s
  c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[TIMING] Rustdoc { host: "x86_64-unknown-linux-gnu" } -- 0.711
< Rustdoc { host: "x86_64-unknown-linux-gnu" }
Build completed successfully in 0:00:15

real	0m15.957s
user	0m14.440s
sys	0m1.422s
#with 1 threads and idle cpu:
#real	230m8.523s
#user	226m12.745s
#sys	2m30.431s
#remove rustup's nightly installation (or else, I guess some leftovers remain there so size will be 1.9G instead of like 1.2G or less)
rm -rf -- "${HOME}/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu"
./go:63+ rm -rf -- /home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu
mkdir -p -- "${HOME}/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu"
./go:64+ mkdir -p -- /home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu
time PATH="${HOME}/build/1packages/4used/chro/4_chromium-dev-git/makepkg_pacman/chromium-dev/src/python-path/:${PATH}" python2 ./x.py install $verbose --incremental -j $threads
./go:65+ PATH=/home/xftroxgpx/build/1packages/4used/chro/4_chromium-dev-git/makepkg_pacman/chromium-dev/src/python-path/:/opt/google-cloud-sdk/bin:/opt/google-cloud-sdk/bin:/usr/local/sbin:/usr/local/bin:/usr/bin:/opt/depot_tools:/usr/bin/site_perl:/usr/bin/vendor_perl:/usr/bin/core_perl:/home/xftroxgpx/bin:/home/xftroxgpx/build/2nonpkgs/rust.stuff/racer/target/release:/home/xftroxgpx/build/2nonpkgs/rust.stuff/chars/target/debug:/home/xftroxgpx/build/2nonpkgs/rust.stuff/chars/target/release:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rustfmt/target/release:/home/xftroxgpx/.cargo/bin:/opt/depot_tools:/usr/bin/site_perl:/usr/bin/vendor_perl:/usr/bin/core_perl:/home/xftroxgpx/bin:/home/xftroxgpx/build/2nonpkgs/rust.stuff/racer/target/release:/home/xftroxgpx/build/2nonpkgs/rust.stuff/chars/target/debug:/home/xftroxgpx/build/2nonpkgs/rust.stuff/chars/target/release:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rustfmt/target/release:/home/xftroxgpx/.cargo/bin
./go:65+ python2 ./x.py install -vv --incremental -j 4
Updating only changed submodules
Submodules updated in 0.06 seconds
running: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/bootstrap/Cargo.toml --verbose
       Fresh unicode-xid v0.1.0
       Fresh cfg-if v0.1.6
       Fresh cc v1.0.28
       Fresh fixedbitset v0.1.9
       Fresh itoa v0.4.3
       Fresh ordermap v0.3.5
       Fresh getopts v0.2.17
       Fresh build_helper v0.1.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/build_helper)
       Fresh lazy_static v0.2.11
       Fresh cmake v0.1.33
       Fresh petgraph v0.4.13
       Fresh proc-macro2 v0.4.24
       Fresh serde v1.0.82
       Fresh libc v0.2.46
       Fresh ryu v0.2.7
       Fresh quote v0.6.10
       Fresh toml v0.4.10
       Fresh time v0.1.40
       Fresh filetime v0.2.4
       Fresh num_cpus v1.8.0
       Fresh serde_json v1.0.33
       Fresh syn v0.15.22
       Fresh serde_derive v1.0.81
       Fresh bootstrap v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/bootstrap)
    Finished dev [unoptimized] target(s) in 0.65s
running: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/bootstrap install -vv --incremental -j 4
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
> Docs { stage: 2, target: "x86_64-unknown-linux-gnu", host: "x86_64-unknown-linux-gnu" }
  > Docs { stage: 2, host: "x86_64-unknown-linux-gnu" }
    > UnstableBook { target: "x86_64-unknown-linux-gnu" }
      > UnstableBookGen { target: "x86_64-unknown-linux-gnu" }
        > Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
          > Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
            > Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
            < Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
            > Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
              > Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
                > Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
                  > StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
                  < StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
                  > Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
                    > Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
                    < Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
                  < Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
                  c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
                  c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
                  > StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
                    c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
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
                c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
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
          c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          > StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
            c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
            c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
            c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
            c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
            c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
            c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
            c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
          < StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        < Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
        c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        > UnstableBookGen { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          > ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "unstable-book-gen", path: "src/tools/unstable-book-gen", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] }
            c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          < ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "unstable-book-gen", path: "src/tools/unstable-book-gen", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] }
        < UnstableBookGen { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      < UnstableBookGen { target: "x86_64-unknown-linux-gnu" }
      > RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "unstable-book", src: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/md-doc" }
        c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        > Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          > ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "rustbook", path: "src/tools/rustbook", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] }
            c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          < ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "rustbook", path: "src/tools/rustbook", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] }
        < Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
thread 'main' panicked at 'source "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/md-doc/unstable-book" failed to get metadata: No such file or directory (os error 2)', src/build_helper/lib.rs:156:19
note: Run with `RUST_BACKTRACE=1` for a backtrace.
Traceback (most recent call last):
  File "./x.py", line 11, in <module>
    bootstrap.main()
  File "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/bootstrap/bootstrap.py", line 843, in main
    bootstrap(help_triggered)
  File "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/bootstrap/bootstrap.py", line 829, in bootstrap
    run(args, env=env, verbose=build.verbose)
  File "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/bootstrap/bootstrap.py", line 141, in run
    raise RuntimeError(err)
RuntimeError: failed to run: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/bootstrap install -vv --incremental -j 4

real	0m9.165s
user	0m8.368s
sys	0m0.808s

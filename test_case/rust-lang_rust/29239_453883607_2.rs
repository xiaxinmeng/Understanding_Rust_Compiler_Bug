
/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust 
$ ./go
~/build/2nonpkgs/rust.stuff/rust/rust ~/build/2nonpkgs/rust.stuff/rust
./go:47+ set -xev
export RUSTFLAGS="-Z verbose"
./go:48+ export 'RUSTFLAGS=-Z verbose'
./go:48+ RUSTFLAGS='-Z verbose'
time PATH="${HOME}/build/1packages/4used/chro/4_chromium-dev-git/makepkg_pacman/chromium-dev/src/python-path/:${PATH}" python2 ./x.py build -vv --incremental -j 5 
./go:49+ PATH=/home/xftroxgpx/build/1packages/4used/chro/4_chromium-dev-git/makepkg_pacman/chromium-dev/src/python-path/:/opt/google-cloud-sdk/bin:/opt/google-cloud-sdk/bin:/usr/local/sbin:/usr/local/bin:/usr/bin:/opt/depot_tools:/usr/bin/site_perl:/usr/bin/vendor_perl:/usr/bin/core_perl:/home/xftroxgpx/bin:/home/xftroxgpx/build/2nonpkgs/rust.stuff/racer/target/release:/home/xftroxgpx/build/2nonpkgs/rust.stuff/chars/target/debug:/home/xftroxgpx/build/2nonpkgs/rust.stuff/chars/target/release:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rustfmt/target/release:/home/xftroxgpx/.cargo/bin:/opt/depot_tools:/usr/bin/site_perl:/usr/bin/vendor_perl:/usr/bin/core_perl:/home/xftroxgpx/bin:/home/xftroxgpx/build/2nonpkgs/rust.stuff/racer/target/release:/home/xftroxgpx/build/2nonpkgs/rust.stuff/chars/target/debug:/home/xftroxgpx/build/2nonpkgs/rust.stuff/chars/target/release:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rustfmt/target/release:/home/xftroxgpx/.cargo/bin
./go:49+ python2 ./x.py build -vv --incremental -j 5
Updating only changed submodules
Submodules updated in 0.09 seconds
running: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/bootstrap/Cargo.toml --verbose
       Fresh unicode-xid v0.1.0
       Fresh itoa v0.4.3
       Fresh ordermap v0.3.5
       Fresh cfg-if v0.1.6
       Fresh fixedbitset v0.1.9
       Fresh cc v1.0.25
       Fresh getopts v0.2.17
       Fresh lazy_static v0.2.11
       Fresh build_helper v0.1.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/build_helper)
       Fresh petgraph v0.4.13
       Fresh cmake v0.1.33
       Fresh proc-macro2 v0.4.24
       Fresh libc v0.2.46
       Fresh serde v1.0.82
       Fresh ryu v0.2.7
       Fresh quote v0.6.10
       Fresh filetime v0.2.4
       Fresh num_cpus v1.8.0
       Fresh time v0.1.40
       Fresh toml v0.4.10
       Fresh serde_json v1.0.33
       Fresh syn v0.15.22
       Fresh serde_derive v1.0.81
       Fresh bootstrap v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/bootstrap)
    Finished dev [unoptimized] target(s) in 0.88s
running: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/bootstrap build -vv --incremental -j 5
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
          > Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          < Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "5" "-v" "--release" "--features" "panic-unwind backtrace" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/Cargo.toml" "--message-format" "json"
       Fresh cc v1.0.25
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
       Fresh rustc_msan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_msan)
       Fresh rustc_tsan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_tsan)
       Fresh rustc_lsan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_lsan)
       Fresh panic_unwind v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libpanic_unwind)
       Fresh std v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd)
    Finished release [optimized] target(s) in 0.95s
not updating "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/.libstd.stamp"; contents equal and SystemTime { tv_sec: 1547427671, tv_nsec: 866313844 } <= SystemTime { tv_sec: 1547427672, tv_nsec: 17311946 }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          > StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
            > Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
              c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
            < Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
            c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
            c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          < StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 1.015
        < Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "5" "-v" "--release" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libtest/Cargo.toml" "--message-format" "json"
       Fresh getopts v0.2.17
       Fresh term v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libterm)
       Fresh proc_macro v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libproc_macro)
       Fresh test v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libtest)
    Finished release [optimized] target(s) in 0.68s
not updating "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/.libtest.stamp"; contents equal and SystemTime { tv_sec: 1547427695, tv_nsec: 912011549 } <= SystemTime { tv_sec: 1547427695, tv_nsec: 964010895 }
        c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        > TestLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
          c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        < TestLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[TIMING] Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 0.744
      < Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "5" "-v" "--release" "--features" "" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/rustc/Cargo.toml" "--message-format" "json"
       Fresh nodrop v0.1.12
       Fresh cfg-if v0.1.6
       Fresh scopeguard v0.3.3
       Fresh rand_core v0.3.0
       Fresh lazy_static v1.2.0
       Fresh void v1.0.2
       Fresh memoffset v0.2.1
       Fresh stable_deref_trait v1.1.0
       Fresh semver-parser v0.7.0
       Fresh unicode-width v0.1.5
       Fresh bitflags v1.0.4
       Fresh either v1.5.0
       Fresh byteorder v1.2.7
       Fresh graphviz v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libgraphviz)
       Fresh scoped-tls v0.1.2
       Fresh termcolor v1.0.4
       Fresh lazy_static v0.2.11
       Fresh rustc-demangle v0.1.10
       Fresh datafrog v2.0.1
       Fresh remove_dir_all v0.5.1
       Fresh fmt_macros v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libfmt_macros)
       Fresh rustc_fs_util v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_fs_util)
       Fresh rustc-serialize v0.3.24
       Fresh rustc_platform_intrinsics v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_platform_intrinsics)
       Fresh quick-error v1.2.2
       Fresh cc v1.0.25
       Fresh arrayvec v0.4.7
       Fresh crossbeam-utils v0.2.2
       Fresh log v0.4.6
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
       Fresh crossbeam-epoch v0.3.1
       Fresh rustc_cratesio_shim v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_cratesio_shim)
       Fresh ena v0.11.0
       Fresh smallvec v0.6.7
       Fresh lock_api v0.1.3
       Fresh rustc_version v0.2.3
       Fresh polonius-engine v0.6.2
       Fresh crc32fast v1.1.2
       Fresh chalk-engine v0.9.0
       Fresh rls-data v0.18.1
       Fresh num_cpus v1.8.0
       Fresh rand v0.5.5
       Fresh rand v0.4.3
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
       Fresh rustc v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc)
       Fresh syntax_ext v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libsyntax_ext)
       Fresh rustc_mir v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_mir)
       Fresh rustc_incremental v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_incremental)
       Fresh rustc_typeck v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_typeck)
       Fresh rustc_lint v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_lint)
       Fresh rustc_traits v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_traits)
       Fresh rustc_allocator v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_allocator)
       Fresh rustc_metadata v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_metadata)
       Fresh rustc_passes v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_passes)
       Fresh rustc_borrowck v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_borrowck)
       Fresh rustc_privacy v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_privacy)
       Fresh rustc_codegen_utils v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_utils)
       Fresh rustc_resolve v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_resolve)
       Fresh rustc_plugin v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_plugin)
       Fresh rustc_save_analysis v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_save_analysis)
       Fresh rustc_codegen_ssa v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_ssa)
       Fresh rustc_driver v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_driver)
   Compiling rustc-main v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/rustc)
     Running `/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/rustc --edition=2018 --crate-name rustc_binary src/rustc/rustc.rs --color always --error-format json --crate-type bin --emit=dep-info,link -C opt-level=2 -C metadata=86cfcdb180ca1244 -C extra-filename=-86cfcdb180ca1244 --out-dir /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C incremental=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/incremental -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_codegen_ssa=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_ssa-f68b88a15d5f6923.so --extern rustc_driver=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-d94f0ceba3b5a89b.so --extern rustc_target=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-9a73c2e4a0117f6a.so -Z verbose -L native=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-64431815b1652f33/out -L native=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-5c1d73718543201d/out`
rustc command: "LD_LIBRARY_PATH"="/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/rustc" "--edition=2018" "--crate-name" "rustc_binary" "src/rustc/rustc.rs" "--color" "always" "--crate-type" "bin" "--emit=dep-info,link" "-C" "opt-level=2" "-C" "metadata=86cfcdb180ca1244-rustc" "-C" "extra-filename=-86cfcdb180ca1244" "--out-dir" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "--target" "x86_64-unknown-linux-gnu" "-C" "incremental=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/incremental" "-L" "dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps" "--extern" "rustc_codegen_ssa=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_ssa-f68b88a15d5f6923.so" "--extern" "rustc_driver=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-d94f0ceba3b5a89b.so" "--extern" "rustc_target=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-9a73c2e4a0117f6a.so" "-Z" "verbose" "-L" "native=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-64431815b1652f33/out" "-L" "native=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-5c1d73718543201d/out" "--cfg" "stage0" "--sysroot" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot" "-Cprefer-dynamic" "-C" "debug-assertions=y" "-C" "codegen-units=1" "-C" "link-args=-Wl,-rpath,$ORIGIN/../lib" "-Z" "force-unstable-if-unmarked" "--cfg" "parallel_queries" "-Dwarnings" "-Dbare_trait_objects"
sysroot: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot"
libdir: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib"
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_binary-86cfcdb180ca1244.4bhpffqk3funqhcw.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_binary-86cfcdb180ca1244.4bnnlzdzsdhovhb8.rcgu.o" "-o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_binary-86cfcdb180ca1244" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-64431815b1652f33/out" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-5c1d73718543201d/out" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_driver-d94f0ceba3b5a89b" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_traits-a8773236c481a551" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_save_analysis-6202b8eecfed4af5" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_codegen_utils-ec8e74a195ef7344" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_resolve-d37bfd4921b228a0" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_incremental-de7c86489cd1b79b" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_privacy-b85faedc4ff48bd9" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_typeck-c4ad385f41519a05" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_platform_intrinsics-e62e309067db2736" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_plugin-d84f7948491fa69e" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_metadata-7a8d618be3637136" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lsyntax_ext-e59e3e461f4fb265" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_lint-bd52473efcc64668" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_passes-2ee491130475de2d" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_borrowck-03642d0ef468a1c2" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_mir-354544bfbcea27fc" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_allocator-a3c03bbba7fe7cc7" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc-282236f4f1f46ca9" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-ltest-f3467d7206e00635" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lterm-191793eb2af4b9c0" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_fs_util-c9dfb87f256b1ada" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lsyntax-a744286dd2425515" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_errors-e7d774cc7807d9b2" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lsyntax_pos-936239c3e5bfd9ca" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_target-9a73c2e4a0117f6a" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lfmt_macros-724be23d12ec6248" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-larena-4ae1c3a2423060a6" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_data_structures-7c6c775edbed4c1f" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_cratesio_shim-6fe5b72986d6800e" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lgraphviz-a2d84108033452cc" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lserialize-44f98e624dadc81a" "-Wl,--start-group" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-6d610542ea1eb444" "-Wl,--end-group" "-Wl,-Bstatic" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-d261cb8259a57d68.rlib" "-Wl,-Bdynamic" "-lutil" "-lutil" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../lib"
  = note: /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-d94f0ceba3b5a89b.so: undefined reference to `<(dyn core::any::Any + core::marker::Send + ReStatic) as core::fmt::Debug>::fmt'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-282236f4f1f46ca9.so: undefined reference to `<std::ffi::os_str::OsStr as core::cmp::PartialEq<std::ffi::os_str::OsStr>>::eq'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_cratesio_shim-6fe5b72986d6800e.so: undefined reference to `<core::str::Bytes<ReEarlyBound(0, 'a)> as core::iter_private::TrustedRandomAccess>::may_have_side_effect'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-d94f0ceba3b5a89b.so: undefined reference to `<core::time::Duration as core::ops::arith::Add<core::time::Duration>>::add'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-e7d774cc7807d9b2.so: undefined reference to `<std::io::stdio::StdoutLock<ReEarlyBound(0, 'a)> as std::io::Write>::write'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-d94f0ceba3b5a89b.so: undefined reference to `<alloc::vec::Vec<u8> as core::convert::From<&ReEarlyBound(0, 'a) str>>::from'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-d94f0ceba3b5a89b.so: undefined reference to `<core::time::Duration as core::ops::arith::Sub<core::time::Duration>>::sub'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-9a73c2e4a0117f6a.so: undefined reference to `<std::env::SplitPaths<ReEarlyBound(0, 'a)> as core::iter::iterator::Iterator>::next'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_save_analysis-6202b8eecfed4af5.so: undefined reference to `<std::path::Path as core::cmp::PartialEq<std::path::Path>>::eq'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-d94f0ceba3b5a89b.so: undefined reference to `<std::path::PathBuf as core::cmp::PartialEq<std::path::PathBuf>>::eq'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-282236f4f1f46ca9.so: undefined reference to `<&ReEarlyBound(0, 'a) std::fs::File as std::io::Write>::write'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_cratesio_shim-6fe5b72986d6800e.so: undefined reference to `<core::str::Bytes<ReEarlyBound(0, 'a)> as core::iter_private::TrustedRandomAccess>::get_unchecked'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_typeck-c4ad385f41519a05.so: undefined reference to `<&ReEarlyBound(1, 'b) alloc::string::String as core::str::pattern::Pattern<ReEarlyBound(0, 'a)>>::into_searcher'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-d94f0ceba3b5a89b.so: undefined reference to `<std::io::Guard<ReEarlyBound(0, 'a)> as core::ops::drop::Drop>::drop'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-936239c3e5bfd9ca.so: undefined reference to `<alloc::string::Drain<ReEarlyBound(0, 'a)> as core::ops::drop::Drop>::drop'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-de7c86489cd1b79b.so: undefined reference to `<std::sys::unix::time::Timespec as core::cmp::PartialEq<std::sys::unix::time::Timespec>>::eq'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-282236f4f1f46ca9.so: undefined reference to `<&ReEarlyBound(0, 'a) std::fs::File as std::io::Read>::read'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-6d610542ea1eb444.so: undefined reference to `<rustc_demangle::Demangle<'a> as core::fmt::Display>::fmt'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-d94f0ceba3b5a89b.so: undefined reference to `std::error::<impl core::convert::From<&ReEarlyBound(1, 'b) str> for alloc::boxed::Box<(dyn std::error::Error + core::marker::Sync + core::marker::Send + ReEarlyBound(0, 'a))>>::from'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_save_analysis-6202b8eecfed4af5.so: undefined reference to `<alloc::string::String as core::convert::From<alloc::borrow::Cow<ReEarlyBound(0, 'a), str>>>::from'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-de7c86489cd1b79b.so: undefined reference to `<std::sys::unix::time::Timespec as core::cmp::PartialOrd<std::sys::unix::time::Timespec>>::partial_cmp'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-282236f4f1f46ca9.so: undefined reference to `<std::io::stdio::StderrLock<ReEarlyBound(0, 'a)> as std::io::Write>::write'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-282236f4f1f46ca9.so: undefined reference to `std::error::<impl core::convert::From<alloc::string::String> for alloc::boxed::Box<(dyn std::error::Error + ReStatic)>>::from'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_save_analysis-6202b8eecfed4af5.so: undefined reference to `<core::num::bignum::Big32x40 as core::cmp::PartialOrd<core::num::bignum::Big32x40>>::partial_cmp'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_save_analysis-6202b8eecfed4af5.so: undefined reference to `<std::path::Components<ReEarlyBound(0, 'a)> as core::iter::iterator::Iterator>::next'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-f3467d7206e00635.so: undefined reference to `<std::time::Instant as core::ops::arith::Sub<std::time::Instant>>::sub'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_save_analysis-6202b8eecfed4af5.so: undefined reference to `<core::num::bignum::Big32x40 as core::cmp::PartialEq<core::num::bignum::Big32x40>>::eq'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_utils-ec8e74a195ef7344.so: undefined reference to `<(dyn core::any::Any + ReStatic) as core::fmt::Debug>::fmt'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-282236f4f1f46ca9.so: undefined reference to `<std::path::PathBuf as core::cmp::PartialOrd<std::path::PathBuf>>::partial_cmp'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-d94f0ceba3b5a89b.so: undefined reference to `std::error::<impl core::convert::From<alloc::string::String> for alloc::boxed::Box<(dyn std::error::Error + core::marker::Sync + core::marker::Send + ReStatic)>>::from'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-d94f0ceba3b5a89b.so: undefined reference to `<std::path::Display<ReEarlyBound(0, 'a)> as core::fmt::Display>::fmt'
          collect2: error: ld returned 1 exit status
          

error: aborting due to previous error

[RUSTC-TIMING] rustc_binary test:false 1.081
error: Could not compile `rustc-main`.

Caused by:
  process didn't exit successfully: `/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/rustc --edition=2018 --crate-name rustc_binary src/rustc/rustc.rs --color always --error-format json --crate-type bin --emit=dep-info,link -C opt-level=2 -C metadata=86cfcdb180ca1244 -C extra-filename=-86cfcdb180ca1244 --out-dir /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C incremental=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/incremental -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_codegen_ssa=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_ssa-f68b88a15d5f6923.so --extern rustc_driver=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-d94f0ceba3b5a89b.so --extern rustc_target=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-9a73c2e4a0117f6a.so -Z verbose -L native=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-64431815b1652f33/out -L native=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-5c1d73718543201d/out` (exit code: 1)
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

real	0m14.517s
user	0m12.624s
sys	0m1.256s

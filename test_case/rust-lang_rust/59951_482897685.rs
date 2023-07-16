`
Updating only changed submodules
Submodules updated in 0.03 seconds
running: /home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /home/matthias/vcs/github/rust_cpy/src/bootstrap/Cargo.toml --verbose --verbose --verbose --verbose
       Fresh unicode-xid v0.1.0
       Fresh fixedbitset v0.1.9
       Fresh itoa v0.4.3
       Fresh ordermap v0.3.5
       Fresh cc v1.0.35
       Fresh cfg-if v0.1.6
       Fresh lazy_static v0.2.11
       Fresh getopts v0.2.17
       Fresh build_helper v0.1.0 (/home/matthias/vcs/github/rust_cpy/src/build_helper)
       Fresh cmake v0.1.38
       Fresh petgraph v0.4.13
       Fresh proc-macro2 v0.4.24
       Fresh serde v1.0.82
       Fresh ryu v0.2.7
       Fresh quote v0.6.10
       Fresh libc v0.2.51
       Fresh toml v0.4.10
       Fresh syn v0.15.22
       Fresh time v0.1.40
       Fresh serde_json v1.0.33
       Fresh num_cpus v1.8.0
       Fresh filetime v0.2.4
       Fresh serde_derive v1.0.81
       Fresh bootstrap v0.0.0 (/home/matthias/vcs/github/rust_cpy/src/bootstrap)
    Finished dev [unoptimized] target(s) in 0.30s
running: /home/matthias/vcs/github/rust_cpy/build/bootstrap/debug/bootstrap build -vvvvv
finding compilers
CC_x86_64-unknown-linux-gnu = "clang"
CFLAGS_x86_64-unknown-linux-gnu = ["-ffunction-sections", "-fdata-sections", "-fPIC", "--target=x86_64-unknown-linux-gnu"]
AR_x86_64-unknown-linux-gnu = "ar"
CXX_x86_64-unknown-linux-gnu = "clang++"
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
          c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          > StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
            c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
            c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
            c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          < StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        < Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        > TestLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        < TestLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      < Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      > RustcLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
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
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
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
        c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        c Llvm { target: "x86_64-unknown-linux-gnu", emscripten: false }
        c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        > StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
          c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
          c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
          c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
          c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
          c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        < StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      < Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      > TestLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
        c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      < TestLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    < Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    > RustcLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
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
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Llvm { target: "x86_64-unknown-linux-gnu", emscripten: false }
  < CodegenBackend { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" }
  > Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  < Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
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
    c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
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
    c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
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
    c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
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
c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
> Cargo { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
  c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  > ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "cargo", path: "src/tools/cargo", mode: ToolRustc, is_optional_tool: false, source_type: Submodule, extra_features: [] }
    c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  < ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "cargo", path: "src/tools/cargo", mode: ToolRustc, is_optional_tool: false, source_type: Submodule, extra_features: [] }
< Cargo { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
> Rls { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", extra_features: [] }
  > Clippy { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", extra_features: [] }
    c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    > ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "clippy-driver", path: "src/tools/clippy", mode: ToolRustc, is_optional_tool: true, source_type: Submodule, extra_features: [] }
      c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    < ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "clippy-driver", path: "src/tools/clippy", mode: ToolRustc, is_optional_tool: true, source_type: Submodule, extra_features: [] }
  < Clippy { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", extra_features: [] }
  c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  > ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "rls", path: "src/tools/rls", mode: ToolRustc, is_optional_tool: true, source_type: Submodule, extra_features: ["clippy"] }
    c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  < ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "rls", path: "src/tools/rls", mode: ToolRustc, is_optional_tool: true, source_type: Submodule, extra_features: ["clippy"] }
< Rls { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", extra_features: [] }
c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
> Rustdoc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
< Rustdoc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
c Clippy { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", extra_features: [] }
c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
> Rustfmt { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", extra_features: [] }
  > ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "rustfmt", path: "src/tools/rustfmt", mode: ToolRustc, is_optional_tool: true, source_type: Submodule, extra_features: [] }
    c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  < ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "rustfmt", path: "src/tools/rustfmt", mode: ToolRustc, is_optional_tool: true, source_type: Submodule, extra_features: [] }
< Rustfmt { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", extra_features: [] }
c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
> Miri { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", extra_features: [] }
  > ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "miri", path: "src/tools/miri", mode: ToolRustc, is_optional_tool: true, source_type: Submodule, extra_features: [] }
    c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  < ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "miri", path: "src/tools/miri", mode: ToolRustc, is_optional_tool: true, source_type: Submodule, extra_features: [] }
< Miri { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", extra_features: [] }
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
          c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "-v" "-v" "-v" "-v" "--release" "--features" "panic-unwind backtrace profiler" "--manifest-path" "/home/matthias/vcs/github/rust_cpy/src/libstd/Cargo.toml" "--message-format" "json"
       Fresh cc v1.0.35
       Fresh core v0.0.0 (/home/matthias/vcs/github/rust_cpy/src/libcore)
       Fresh build_helper v0.1.0 (/home/matthias/vcs/github/rust_cpy/src/build_helper)
       Fresh cmake v0.1.38
       Fresh rustc-std-workspace-core v1.0.0 (/home/matthias/vcs/github/rust_cpy/src/tools/rustc-std-workspace-core)
       Fresh libc v0.2.51
       Fresh compiler_builtins v0.1.10
warning: /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.10/compiler-rt/lib/profile/InstrProfilingUtil.c:128:3: warning: implicit declaration of function 'flock' is invalid in C99 [-Wimplicit-function-declaration]
warning:   flock(fd, LOCK_EX);
warning:   ^
warning: /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.10/compiler-rt/lib/profile/InstrProfilingUtil.c:153:3: warning: implicit declaration of function 'flock' is invalid in C99 [-Wimplicit-function-declaration]
warning:   flock(fd, LOCK_UN);
warning:   ^
warning: 2 warnings generated.
       Fresh alloc v0.0.0 (/home/matthias/vcs/github/rust_cpy/src/liballoc)
       Fresh unwind v0.0.0 (/home/matthias/vcs/github/rust_cpy/src/libunwind)
       Fresh panic_abort v0.0.0 (/home/matthias/vcs/github/rust_cpy/src/libpanic_abort)
       Fresh rustc-demangle v0.1.10
       Fresh backtrace-sys v0.1.27
       Fresh profiler_builtins v0.0.0 (/home/matthias/vcs/github/rust_cpy/src/libprofiler_builtins)
       Fresh panic_unwind v0.0.0 (/home/matthias/vcs/github/rust_cpy/src/libpanic_unwind)
       Fresh rustc_asan v0.0.0 (/home/matthias/vcs/github/rust_cpy/src/librustc_asan)
       Fresh rustc_lsan v0.0.0 (/home/matthias/vcs/github/rust_cpy/src/librustc_lsan)
       Fresh rustc_tsan v0.0.0 (/home/matthias/vcs/github/rust_cpy/src/librustc_tsan)
       Fresh rustc_msan v0.0.0 (/home/matthias/vcs/github/rust_cpy/src/librustc_msan)
       Fresh std v0.0.0 (/home/matthias/vcs/github/rust_cpy/src/libstd)
    Finished release [optimized] target(s) in 0.34s
not updating "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/.libstd.stamp"; contents equal and SystemTime { tv_sec: 1555197480, tv_nsec: 877067189 } <= SystemTime { tv_sec: 1555197480, tv_nsec: 927068519 }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          > StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
            c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
            c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copy "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc-d03e85823c13e08d.rlib" to "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-d03e85823c13e08d.rlib"
Copy "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libbacktrace_sys-be3327a4c07b1a13.rlib" to "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace_sys-be3327a4c07b1a13.rlib"
Copy "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-b5646ffdff300a44.rlib" to "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-b5646ffdff300a44.rlib"
Copy "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-b1507b785f07a713.rlib" to "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-b1507b785f07a713.rlib"
Copy "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-36513c8d3b95cf0d.rlib" to "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-36513c8d3b95cf0d.rlib"
Copy "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-931dd8165e7284be.rlib" to "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-931dd8165e7284be.rlib"
Copy "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-0e526291e891a2c7.rlib" to "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-0e526291e891a2c7.rlib"
Copy "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libprofiler_builtins-f7aa0ec50263f14b.rlib" to "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libprofiler_builtins-f7aa0ec50263f14b.rlib"
Copy "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-428af034d8627249.rlib" to "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_asan-428af034d8627249.rlib"
Copy "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-9d56407601b0ca13.rlib" to "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-9d56407601b0ca13.rlib"
Copy "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-5e35a40228b5a190.rlib" to "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_lsan-5e35a40228b5a190.rlib"
Copy "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-dc2ca614386074b4.rlib" to "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_msan-dc2ca614386074b4.rlib"
Copy "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_std_workspace_core-4388263ef7bde094.rlib" to "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-4388263ef7bde094.rlib"
Copy "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-b13855bbcd9769b8.rlib" to "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_tsan-b13855bbcd9769b8.rlib"
Copy "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd-28bc271858497054.rlib" to "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-28bc271858497054.rlib"
Copy "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd-28bc271858497054.so" to "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-28bc271858497054.so"
Copy "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libunwind-65e3134e2660d7bc.rlib" to "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-65e3134e2660d7bc.rlib"
            c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          < StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        < Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "-v" "-v" "-v" "-v" "--release" "--manifest-path" "/home/matthias/vcs/github/rust_cpy/src/libtest/Cargo.toml" "--message-format" "json"
       Fresh term v0.0.0 (/home/matthias/vcs/github/rust_cpy/src/libterm)
       Fresh getopts v0.2.17
       Fresh proc_macro v0.0.0 (/home/matthias/vcs/github/rust_cpy/src/libproc_macro)
   Compiling test v0.0.0 (/home/matthias/vcs/github/rust_cpy/src/libtest)
     Running `CARGO_PKG_VERSION=0.0.0 CARGO_PKG_VERSION_MAJOR=0 CARGO_PKG_VERSION_PRE= CARGO_PKG_AUTHORS='The Rust Project Developers' CARGO_PKG_NAME=test CARGO_PKG_VERSION_PATCH=0 CARGO_MANIFEST_DIR=/home/matthias/vcs/github/rust_cpy/src/libtest CARGO_PRIMARY_PACKAGE=1 LD_LIBRARY_PATH='/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-test/release/deps:/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0/lib' CARGO=/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0/bin/cargo CARGO_PKG_VERSION_MINOR=0 CARGO_PKG_HOMEPAGE= CARGO_PKG_REPOSITORY= CARGO_PKG_DESCRIPTION= /home/matthias/.cargo/bin/sccache /home/matthias/vcs/github/rust_cpy/build/bootstrap/debug/rustc --edition=2018 --crate-name test src/libtest/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=d3d2eaf7458bec3f -C extra-filename=-d3d2eaf7458bec3f --out-dir /home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-test/release/deps --extern getopts=/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/deps/libgetopts-44069824bc6a7f7b.rlib --extern proc_macro=/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/deps/libproc_macro-799f711640640186.rlib --extern term=/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/deps/libterm-f7b9097f667c8ea0.so --extern term=/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/deps/libterm-f7b9097f667c8ea0.rlib -C target-cpu=native`
rustc command: "LD_LIBRARY_PATH"="/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0/lib:/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-test/release/deps:/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0/lib" "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0/bin/rustc" "--edition=2018" "--crate-name" "test" "src/libtest/lib.rs" "--color" "always" "--crate-type" "dylib" "--crate-type" "rlib" "--emit=dep-info,link" "-C" "prefer-dynamic" "-C" "opt-level=2" "-C" "metadata=d3d2eaf7458bec3f-rustc" "-C" "extra-filename=-d3d2eaf7458bec3f" "--out-dir" "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/deps" "--target" "x86_64-unknown-linux-gnu" "-L" "dependency=/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-test/release/deps" "--extern" "getopts=/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/deps/libgetopts-44069824bc6a7f7b.rlib" "--extern" "proc_macro=/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/deps/libproc_macro-799f711640640186.rlib" "--extern" "term=/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/deps/libterm-f7b9097f667c8ea0.so" "--extern" "term=/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/deps/libterm-f7b9097f667c8ea0.rlib" "-C" "target-cpu=native" "--cfg" "stage0" "--sysroot" "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-sysroot" "-Zexternal-macro-backtrace" "-Cprefer-dynamic" "-C" "debug-assertions=n" "-Zemit-stack-sizes" "-C" "link-args=-Wl,-rpath,$ORIGIN/../lib" "-Z" "force-unstable-if-unmarked" "-Dwarnings" "-Dbare_trait_objects"
sysroot: "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-sysroot"
libdir: "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0/lib"
error[E0460]: found possibly newer version of crate `std` which `getopts` depends on
  --> src/libtest/lib.rs:34:5
   |
34 | use getopts;
   |     ^^^^^^^
   |
   = note: perhaps that crate needs to be recompiled?
   = note: the following crate versions were found:
           crate `std`: /home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-28bc271858497054.rlib
           crate `std`: /home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-28bc271858497054.so
           crate `getopts`: /home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/deps/libgetopts-44069824bc6a7f7b.rlib

error: aborting due to previous error

For more information about this error, try `rustc --explain E0460`.
error: Could not compile `test`.

Caused by:
  process didn't exit successfully: `CARGO_PKG_VERSION=0.0.0 CARGO_PKG_VERSION_MAJOR=0 CARGO_PKG_VERSION_PRE= CARGO_PKG_AUTHORS='The Rust Project Developers' CARGO_PKG_NAME=test CARGO_PKG_VERSION_PATCH=0 CARGO_MANIFEST_DIR=/home/matthias/vcs/github/rust_cpy/src/libtest CARGO_PRIMARY_PACKAGE=1 LD_LIBRARY_PATH='/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-test/release/deps:/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0/lib' CARGO=/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0/bin/cargo CARGO_PKG_VERSION_MINOR=0 CARGO_PKG_HOMEPAGE= CARGO_PKG_REPOSITORY= CARGO_PKG_DESCRIPTION= /home/matthias/.cargo/bin/sccache /home/matthias/vcs/github/rust_cpy/build/bootstrap/debug/rustc --edition=2018 --crate-name test src/libtest/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=d3d2eaf7458bec3f -C extra-filename=-d3d2eaf7458bec3f --out-dir /home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-test/release/deps --extern getopts=/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/deps/libgetopts-44069824bc6a7f7b.rlib --extern proc_macro=/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/deps/libproc_macro-799f711640640186.rlib --extern term=/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/deps/libterm-f7b9097f667c8ea0.so --extern term=/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/deps/libterm-f7b9097f667c8ea0.rlib -C target-cpu=native` (exit code: 1)
command did not execute successfully: "/home/matthias/vcs/github/rust_cpy/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "-v" "-v" "-v" "-v" "--release" "--manifest-path" "/home/matthias/vcs/github/rust_cpy/src/libtest/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101
Traceback (most recent call last):
  File "./x.py", line 11, in <module>
    bootstrap.main()
  File "/home/matthias/vcs/github/rust_cpy/src/bootstrap/bootstrap.py", line 842, in main
    bootstrap(help_triggered)
  File "/home/matthias/vcs/github/rust_cpy/src/bootstrap/bootstrap.py", line 828, in bootstrap
    run(args, env=env, verbose=build.verbose)
  File "/home/matthias/vcs/github/rust_cpy/src/bootstrap/bootstrap.py", line 141, in run
    raise RuntimeError(err)
RuntimeError: failed to run: /home/matthias/vcs/github/rust_cpy/build/bootstrap/debug/bootstrap build -vvvvv

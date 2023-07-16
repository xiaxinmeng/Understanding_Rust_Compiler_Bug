
/home/user/build/2nonpkgs/rust.stuff/rust 
$ ./go
~/build/2nonpkgs/rust.stuff/rust/rust ~/build/2nonpkgs/rust.stuff/rust
!! RUSTFLAGS=''
running: /home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/bootstrap/Cargo.toml --verbose
       Fresh unicode-xid v0.1.0
       Fresh itoa v0.4.4
       Fresh cfg-if v0.1.8
       Fresh unicode-width v0.1.6
       Fresh fixedbitset v0.1.9
       Fresh ordermap v0.3.5
       Fresh cc v1.0.35
       Fresh build_helper v0.1.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/build_helper)
       Fresh lazy_static v1.3.0
       Fresh getopts v0.2.21
       Fresh petgraph v0.4.13
       Fresh cmake v0.1.38
       Fresh proc-macro2 v0.4.30
       Fresh libc v0.2.62
       Fresh ryu v1.0.0
       Fresh quote v0.6.12
       Fresh num_cpus v1.8.0
       Fresh time v0.1.40
       Fresh filetime v0.2.4
       Fresh syn v0.15.35
       Fresh serde_derive v1.0.81
       Fresh serde v1.0.99
       Fresh serde_json v1.0.40
       Fresh toml v0.5.3
       Fresh bootstrap v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/bootstrap)
    Finished dev [unoptimized] target(s) in 0.58s
running: /home/user/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/bootstrap build -vv -j 4
finding compilers
CC_x86_64-unknown-linux-gnu = "cc"
CFLAGS_x86_64-unknown-linux-gnu = ["-ffunction-sections", "-fdata-sections", "-fPIC", "-m64"]
CXX_x86_64-unknown-linux-gnu = "c++"
CXXFLAGS_x86_64-unknown-linux-gnu = ["-ffunction-sections", "-fdata-sections", "-fPIC", "-m64"]
AR_x86_64-unknown-linux-gnu = "ar"
running sanity check
learning about cargo
finding compilers
CC_x86_64-unknown-linux-gnu = "cc"
CFLAGS_x86_64-unknown-linux-gnu = ["-ffunction-sections", "-fdata-sections", "-fPIC", "-m64"]
CXX_x86_64-unknown-linux-gnu = "c++"
CXXFLAGS_x86_64-unknown-linux-gnu = ["-ffunction-sections", "-fdata-sections", "-fPIC", "-m64"]
AR_x86_64-unknown-linux-gnu = "ar"
running sanity check
learning about cargo
> Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  > Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    > Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
    < Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
    > Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      > Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        > StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        < StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
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
        < StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      < Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      > RustcLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      < RustcLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    < Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
    > CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" }
      c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
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
    > Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      > StartupObjects { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      < StartupObjects { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      > StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      < StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    < Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    > RustcLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    < RustcLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
  < Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  > CodegenBackend { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" }
    c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
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
  < StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
< Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
> Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  c Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  > RustcLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
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
  > ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "cargo", path: "src/tools/cargo", mode: ToolRustc, is_optional_tool: false, source_type: Submodule, extra_features: [] }
    c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
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
    > ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "clippy-driver", path: "src/tools/clippy", mode: ToolRustc, is_optional_tool: false, source_type: Submodule, extra_features: [] }
      c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    < ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "clippy-driver", path: "src/tools/clippy", mode: ToolRustc, is_optional_tool: false, source_type: Submodule, extra_features: [] }
  < Clippy { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", extra_features: [] }
  > ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "rls", path: "src/tools/rls", mode: ToolRustc, is_optional_tool: false, source_type: Submodule, extra_features: ["clippy"] }
    c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  < ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "rls", path: "src/tools/rls", mode: ToolRustc, is_optional_tool: false, source_type: Submodule, extra_features: ["clippy"] }
< Rls { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", extra_features: [] }
c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
> Rustdoc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
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
  > ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "rustfmt", path: "src/tools/rustfmt", mode: ToolRustc, is_optional_tool: false, source_type: Submodule, extra_features: [] }
    c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  < ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "rustfmt", path: "src/tools/rustfmt", mode: ToolRustc, is_optional_tool: false, source_type: Submodule, extra_features: [] }
< Rustfmt { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", extra_features: [] }
c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
> Miri { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", extra_features: [] }
  > ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "miri", path: "src/tools/miri", mode: ToolRustc, is_optional_tool: false, source_type: Submodule, extra_features: [] }
    c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  < ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "miri", path: "src/tools/miri", mode: ToolRustc, is_optional_tool: false, source_type: Submodule, extra_features: [] }
< Miri { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", extra_features: [] }
> Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  > Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    > Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
    < Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
    > Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      > Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        > StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        < StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        > Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          > Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          < Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        < Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "4" "-v" "--release" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
       Fresh cc v1.0.35
       Fresh core v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libcore)
       Fresh build_helper v0.1.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/build_helper)
       Fresh cmake v0.1.38
       Fresh rustc-std-workspace-core v1.99.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/tools/rustc-std-workspace-core)
       Fresh compiler_builtins v0.1.18
       Fresh libc v0.2.62
       Fresh alloc v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/liballoc)
       Fresh cfg-if v0.1.8
       Fresh rustc-demangle v0.1.16
       Fresh backtrace-sys v0.1.30
       Fresh panic_abort v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libpanic_abort)
       Fresh rustc-std-workspace-alloc v1.99.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/tools/rustc-std-workspace-alloc)
       Fresh unwind v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libunwind)
       Fresh rustc_asan v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_asan)
       Fresh rustc_msan v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_msan)
       Fresh backtrace v0.3.37
       Fresh rustc_tsan v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_tsan)
       Fresh rustc_lsan v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_lsan)
       Fresh hashbrown v0.5.0
       Fresh panic_unwind v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libpanic_unwind)
       Fresh std v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd)
       Fresh rustc-std-workspace-std v1.99.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/tools/rustc-std-workspace-std)
       Fresh term v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libterm)
       Fresh proc_macro v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libproc_macro)
       Fresh unicode-width v0.1.6
       Fresh getopts v0.2.21
       Fresh test v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libtest)
    Finished release [optimized + debuginfo] target(s) in 0.82s
        c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        > StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
          c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc-71f93ed1ab82547c.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-71f93ed1ab82547c.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libbacktrace-6cd73302dd4c98eb.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace-6cd73302dd4c98eb.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libbacktrace_sys-bae5f3028500dae3.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace_sys-bae5f3028500dae3.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcfg_if-55bdcc207a1bee34.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-55bdcc207a1bee34.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-c6be53ec1b911294.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-c6be53ec1b911294.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-c5ba46aa9bfe926c.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-c5ba46aa9bfe926c.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libgetopts-9a06aa12d125b84b.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-9a06aa12d125b84b.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libhashbrown-91037212e1d2d517.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-91037212e1d2d517.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-172079ea5a24aed9.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-172079ea5a24aed9.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-38f557647c6acd63.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-38f557647c6acd63.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-250b852a3643d9ea.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-250b852a3643d9ea.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libproc_macro-808ec94201f3177b.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-808ec94201f3177b.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-b5c1c0998b873b13.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_asan-b5c1c0998b873b13.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-5c18d7aaa0f136b1.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-5c18d7aaa0f136b1.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-e56cf120f5a7f7f9.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_lsan-e56cf120f5a7f7f9.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-ac5de375596001fd.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_msan-ac5de375596001fd.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_std_workspace_alloc-97a947d5db98f554.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-97a947d5db98f554.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_std_workspace_core-07743f9db7700ced.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-07743f9db7700ced.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_std_workspace_std-f939144e8078f62a.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_std-f939144e8078f62a.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-d7dbd4d0b8e240ad.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_tsan-d7dbd4d0b8e240ad.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd-dd2bfed78f6cfbad.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-dd2bfed78f6cfbad.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd-dd2bfed78f6cfbad.so" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-dd2bfed78f6cfbad.so"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libterm-278fcceba9167c74.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-278fcceba9167c74.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libtest-2d112b68cfc40bf7.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-2d112b68cfc40bf7.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libtest-2d112b68cfc40bf7.so" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-2d112b68cfc40bf7.so"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libunicode_width-f1c8f463fa042044.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-f1c8f463fa042044.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libunwind-74d6fa43cfe389bd.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-74d6fa43cfe389bd.rlib"
        < StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 0.867
      < Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "4" "-v" "--release" "--features" "" "--manifest-path" "/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
       Fresh cfg-if v0.1.8
       Fresh lazy_static v1.3.0
       Fresh semver-parser v0.7.0
       Fresh nodrop v0.1.12
       Fresh smallvec v0.6.10
       Fresh memoffset v0.2.1
       Fresh scopeguard v0.3.3
       Fresh unicode-xid v0.1.0
       Fresh scopeguard v1.0.0
       Fresh indexmap v1.0.2
       Fresh stable_deref_trait v1.1.0
       Fresh either v1.5.0
       Fresh graphviz v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libgraphviz)
       Fresh scoped-tls v1.0.0
       Fresh unicode-width v0.1.6
       Fresh unicode-xid v0.2.0
       Fresh annotate-snippets v0.6.1
       Fresh termcolor v1.0.4
       Fresh lazy_static v0.2.11
       Fresh datafrog v2.0.1
       Fresh rustc-demangle v0.1.16
       Fresh rustc_fs_util v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_fs_util)
       Fresh ppv-lite86 v0.2.5
       Fresh punycode v0.4.0
       Fresh remove_dir_all v0.5.2
       Fresh itoa v0.4.4
       Fresh cc v1.0.35
       Fresh once_cell v1.1.0
       Fresh semver v0.9.0
       Fresh crossbeam-utils v0.2.2
       Fresh crossbeam-utils v0.6.5
       Fresh log_settings v0.1.2
       Fresh arrayvec v0.4.7
       Fresh lock_api v0.3.1
       Fresh serialize v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libserialize)
       Fresh itertools v0.8.0
       Fresh rustc_lexer v0.1.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_lexer)
       Fresh chalk-macros v0.1.0
       Fresh c2-chacha v0.2.2
       Fresh libc v0.2.62
       Fresh rustc_version v0.2.3
       Fresh byteorder v1.3.2
       Fresh log v0.4.8
       Fresh crossbeam-epoch v0.3.1
       Fresh proc-macro2 v0.4.30
       Fresh rustc_index v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_index)
       Fresh bitflags v1.1.0
       Fresh crc32fast v1.1.2
       Fresh serde v1.0.99
       Fresh ryu v1.0.0
       Fresh rustc-hash v1.0.1
       Fresh num_cpus v1.8.0
       Fresh ena v0.13.1
       Fresh jobserver v0.1.16
       Fresh atty v0.2.11
       Fresh term_size v0.3.1
       Fresh memmap v0.6.2
       Fresh getrandom v0.1.12
       Fresh env_logger v0.7.0
       Fresh crossbeam-deque v0.2.0
       Fresh quote v0.6.12
       Fresh backtrace-sys v0.1.30
       Fresh rustc_apfloat v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_apfloat)
       Fresh miniz-sys v0.1.11
       Fresh rls-span v0.5.1
       Fresh chalk-engine v0.9.0
       Fresh polonius-engine v0.10.0
       Fresh serde_json v1.0.40
       Fresh measureme v0.3.0
       Fresh rand_core v0.5.0
       Fresh rustc-rayon-core v0.2.0
       Fresh syn v0.15.35
       Fresh backtrace v0.3.37
       Fresh parking_lot_core v0.6.2
       Fresh flate2 v1.0.6
       Fresh rls-data v0.19.0
       Fresh rustc-rayon v0.2.0
       Fresh synstructure v0.10.2
       Fresh rand_chacha v0.2.1
       Fresh parking_lot v0.9.0
       Fresh rustc_data_structures v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_data_structures)
       Fresh rustc_macros v0.1.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_macros)
       Fresh rand v0.7.0
       Fresh arena v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libarena)
       Fresh tempfile v3.1.0
       Fresh syntax_pos v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libsyntax_pos)
       Fresh rustc_errors v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_errors)
       Fresh rustc_target v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_target)
       Fresh fmt_macros v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libfmt_macros)
       Fresh syntax v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libsyntax)
       Fresh rustc v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc)
       Fresh syntax_ext v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libsyntax_ext)
       Fresh rustc_metadata v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_metadata)
       Fresh rustc_typeck v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_typeck)
       Fresh rustc_incremental v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_incremental)
       Fresh rustc_passes v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_passes)
       Fresh rustc_mir v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_mir)
       Fresh rustc_lint v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_lint)
       Fresh rustc_traits v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_traits)
       Fresh rustc_codegen_utils v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_utils)
       Fresh rustc_plugin_impl v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_plugin)
       Fresh rustc_privacy v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_privacy)
       Fresh rustc_resolve v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_resolve)
       Fresh rustc_codegen_ssa v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_ssa)
       Fresh rustc_save_analysis v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_save_analysis)
       Fresh rustc_interface v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_interface)
       Fresh rustc_plugin v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_plugin/deprecated)
       Fresh rustc_driver v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_driver)
       Fresh rustc-main v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/rustc)
    Finished release [optimized + debuginfo] target(s) in 0.58s
      c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      > RustcLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
        c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/librustc_macros-f624ba9a96c12dd9.so" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_macros-f624ba9a96c12dd9.so"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libannotate_snippets-6b6e5f2cd883a14d.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libannotate_snippets-6b6e5f2cd883a14d.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-b7671023e5531a1e.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libarena-b7671023e5531a1e.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarrayvec-5159a127fcdb9fd0.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libarrayvec-5159a127fcdb9fd0.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libatty-4b40c1a03d98333b.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libatty-4b40c1a03d98333b.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-5d07a574cf464e96.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace-5d07a574cf464e96.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace_sys-fd0ecb64e926d37f.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace_sys-fd0ecb64e926d37f.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-9f2121b1a5142506.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbitflags-9f2121b1a5142506.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-abbbb36d80390654.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbyteorder-abbbb36d80390654.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libc2_chacha-fd29959876dbfe5b.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libc2_chacha-fd29959876dbfe5b.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcc-2878faa7ab90c938.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcc-2878faa7ab90c938.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcfg_if-9ed064b040335418.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-9ed064b040335418.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_engine-1b42591d05246771.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libchalk_engine-1b42591d05246771.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_macros-32415281f111e41f.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libchalk_macros-32415281f111e41f.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcrc32fast-89023483a0ac76c3.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcrc32fast-89023483a0ac76c3.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcrossbeam_deque-f2636eb50e5d1832.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcrossbeam_deque-f2636eb50e5d1832.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcrossbeam_epoch-b31c2be1086d633f.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcrossbeam_epoch-b31c2be1086d633f.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcrossbeam_utils-0a7062d4142e4349.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcrossbeam_utils-0a7062d4142e4349.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcrossbeam_utils-dbb279a9272338ae.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcrossbeam_utils-dbb279a9272338ae.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libdatafrog-af4c6864b6b18e26.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libdatafrog-af4c6864b6b18e26.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-01ef4429ef9899fb.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libeither-01ef4429ef9899fb.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libena-bdb327bbafc0db48.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libena-bdb327bbafc0db48.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libenv_logger-d763101d179c2af0.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libenv_logger-d763101d179c2af0.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-9689f2a75195127a.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libflate2-9689f2a75195127a.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-5958351567b04352.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-5958351567b04352.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgetrandom-173b4b3411dea322.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetrandom-173b4b3411dea322.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-3bfe7805716a2c54.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgraphviz-3bfe7805716a2c54.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libindexmap-f05341697149ca40.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libindexmap-f05341697149ca40.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libitoa-1a1e671ccf8a2b0c.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libitoa-1a1e671ccf8a2b0c.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-140a2d07988f44ce.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libjobserver-140a2d07988f44ce.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-bca894736d6298da.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblazy_static-bca894736d6298da.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-c1893469e77cfe22.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblazy_static-c1893469e77cfe22.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-77fc029246564318.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-77fc029246564318.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblock_api-acd6466921e24045.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblock_api-acd6466921e24045.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-173305f1c5b82e53.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-173305f1c5b82e53.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-5024e538fe751a52.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog_settings-5024e538fe751a52.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libmeasureme-a24274d33767f0fc.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmeasureme-a24274d33767f0fc.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libmemmap-47f156fced208339.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemmap-47f156fced208339.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libmemoffset-17e182a9235a4380.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemoffset-17e182a9235a4380.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libminiz_sys-eb359dbc17b390a9.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_sys-eb359dbc17b390a9.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libnodrop-2f8485070be4ae15.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libnodrop-2f8485070be4ae15.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-451df79e0ae9fba0.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libnum_cpus-451df79e0ae9fba0.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libonce_cell-2f90a3e7edcc12c2.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libonce_cell-2f90a3e7edcc12c2.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-c3608bcf984e254f.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libparking_lot-c3608bcf984e254f.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot_core-d0b0e0428ce46b6c.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libparking_lot_core-d0b0e0428ce46b6c.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-881011295ef82b82.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpolonius_engine-881011295ef82b82.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libppv_lite86-2e101d142ee21d31.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libppv_lite86-2e101d142ee21d31.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpunycode-0698ea5c8a1ae6c5.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpunycode-0698ea5c8a1ae6c5.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librand-2565b33ade1663b4.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-2565b33ade1663b4.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librand_chacha-2579852265fe182b.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand_chacha-2579852265fe182b.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librand_core-bcfff016c7c53404.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand_core-bcfff016c7c53404.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libremove_dir_all-daacc9d5a7e86f8f.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libremove_dir_all-daacc9d5a7e86f8f.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librls_data-7c14a8295e243b2b.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librls_data-7c14a8295e243b2b.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librls_span-3416b9d0c92db664.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librls_span-3416b9d0c92db664.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-913629979841b7b6.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc-913629979841b7b6.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-2150ad41c4ccb279.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_apfloat-2150ad41c4ccb279.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_ssa-9201536e5f072916.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_codegen_ssa-9201536e5f072916.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_utils-dfa4bc1620021d25.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_codegen_utils-dfa4bc1620021d25.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-d94b630380cf4306.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_data_structures-d94b630380cf4306.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-ff97433987d44031.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-ff97433987d44031.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-f54644be02dcf898.so" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_driver-f54644be02dcf898.so"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-9922381a9c537afd.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_errors-9922381a9c537afd.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_fs_util-3782e217e362f4ef.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_fs_util-3782e217e362f4ef.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hash-25d6e7859b964332.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_hash-25d6e7859b964332.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-516fd58be13701d1.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_incremental-516fd58be13701d1.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-28b8627beb1f4a3a.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_index-28b8627beb1f4a3a.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_interface-193dcc2214d5f0fc.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_interface-193dcc2214d5f0fc.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lexer-33a4e671dd66c0d1.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_lexer-33a4e671dd66c0d1.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lint-cc2e33b26dfd5778.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_lint-cc2e33b26dfd5778.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_metadata-f680bd8786bf643e.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_metadata-f680bd8786bf643e.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-1b5bcb450ea6148a.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_mir-1b5bcb450ea6148a.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_passes-19b22609703b0d53.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_passes-19b22609703b0d53.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_plugin-0cda24dbce646118.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_plugin-0cda24dbce646118.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_plugin_impl-275e96d3eeea353f.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_plugin_impl-275e96d3eeea353f.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_privacy-766e40693059d20d.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_privacy-766e40693059d20d.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-6bc7542f15a5c06f.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_rayon-6bc7542f15a5c06f.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-6a93905ffb33bcd0.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_rayon_core-6a93905ffb33bcd0.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_resolve-477d52e7184bdf1a.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_resolve-477d52e7184bdf1a.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_save_analysis-02be0c88f6323a4e.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_save_analysis-02be0c88f6323a4e.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-e4f15817b325193b.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_target-e4f15817b325193b.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_traits-c78c755564b0f9cc.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_traits-c78c755564b0f9cc.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_typeck-8505741e57838511.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_typeck-8505741e57838511.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libryu-345ccfd6e1baedba.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libryu-345ccfd6e1baedba.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-577288e5afe5f4a8.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libscoped_tls-577288e5afe5f4a8.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscopeguard-572d5e9fc48414b0.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libscopeguard-572d5e9fc48414b0.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscopeguard-7b9159f5e7665206.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libscopeguard-7b9159f5e7665206.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserde-a2d312d4584796cc.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserde-a2d312d4584796cc.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserde_json-b2a46bbb73b68f94.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserde_json-b2a46bbb73b68f94.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-58c5137427099ad6.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-58c5137427099ad6.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-a9138a95188d1d4b.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsmallvec-a9138a95188d1d4b.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libstable_deref_trait-808d2d0bc4af47df.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstable_deref_trait-808d2d0bc4af47df.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-428fed48ebd8d7ff.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-428fed48ebd8d7ff.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_ext-d9cc3b78f7506ac9.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax_ext-d9cc3b78f7506ac9.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-69e55c95f9f92c04.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax_pos-69e55c95f9f92c04.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-29abbebc8e695142.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtempfile-29abbebc8e695142.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libterm_size-5e180ff0dd295227.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm_size-5e180ff0dd295227.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtermcolor-15514cdc6b0b60b4.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtermcolor-15514cdc6b0b60b4.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libunicode_width-cd0a6eb59a051bd7.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-cd0a6eb59a051bd7.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libunicode_xid-bff5697cd48aa673.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_xid-bff5697cd48aa673.rlib"
      < RustcLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[TIMING] Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 0.722
    < Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
    > CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" }
      c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      > Llvm { target: "x86_64-unknown-linux-gnu", emscripten: false }
      < Llvm { target: "x86_64-unknown-linux-gnu", emscripten: false }
Building stage0 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
running: "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "4" "-v" "--release" "--manifest-path" "/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_llvm/Cargo.toml" "--features" "" "--message-format" "json-render-diagnostics"
       Fresh cc v1.0.35
       Fresh build_helper v0.1.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/build_helper)
warning: In file included from ../rustllvm/rustllvm.h:4,
warning:                  from ../rustllvm/PassWrapper.cpp:6:
warning: /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/llvm-project/llvm/include/llvm/ADT/ArrayRef.h: In instantiation of ‘llvm::ArrayRef<T>::ArrayRef(const std::initializer_list<_Tp>&) [with T = long unsigned int]’:
warning: /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/llvm-project/llvm/include/llvm/IR/DIBuilder.h:644:74:   required from here
warning: /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/llvm-project/llvm/include/llvm/ADT/ArrayRef.h:101:37: warning: initializing ‘llvm::ArrayRef<long unsigned int>::Data’ from ‘std::initializer_list<long unsigned int>::begin’ does not extend the lifetime of the underlying array [-Winit-list-lifetime]
warning:   101 |     : Data(Vec.begin() == Vec.end() ? (T*)nullptr : Vec.begin()),
warning: In file included from ../rustllvm/rustllvm.h:4,
warning:                  from ../rustllvm/RustWrapper.cpp:1:
warning: /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/llvm-project/llvm/include/llvm/ADT/ArrayRef.h: In instantiation of ‘llvm::ArrayRef<T>::ArrayRef(const std::initializer_list<_Tp>&) [with T = long unsigned int]’:
warning: /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/llvm-project/llvm/include/llvm/IR/DIBuilder.h:644:74:   required from here
warning: /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/llvm-project/llvm/include/llvm/ADT/ArrayRef.h:101:37: warning: initializing ‘llvm::ArrayRef<long unsigned int>::Data’ from ‘std::initializer_list<long unsigned int>::begin’ does not extend the lifetime of the underlying array [-Winit-list-lifetime]
warning:   101 |     : Data(Vec.begin() == Vec.end() ? (T*)nullptr : Vec.begin()),
warning: In file included from ../rustllvm/rustllvm.h:4,
warning:                  from ../rustllvm/ArchiveWrapper.cpp:1:
warning: /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/llvm-project/llvm/include/llvm/ADT/ArrayRef.h: In instantiation of ‘llvm::ArrayRef<T>::ArrayRef(const std::initializer_list<_Tp>&) [with T = long unsigned int]’:
warning: /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/llvm-project/llvm/include/llvm/IR/DIBuilder.h:644:74:   required from here
warning: /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/llvm-project/llvm/include/llvm/ADT/ArrayRef.h:101:37: warning: initializing ‘llvm::ArrayRef<long unsigned int>::Data’ from ‘std::initializer_list<long unsigned int>::begin’ does not extend the lifetime of the underlying array [-Winit-list-lifetime]
warning:   101 |     : Data(Vec.begin() == Vec.end() ? (T*)nullptr : Vec.begin()),
warning: In file included from /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/llvm-project/llvm/include/llvm/Linker/IRMover.h:12,
warning:                  from /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/llvm-project/llvm/include/llvm/Linker/Linker.h:13,
warning:                  from ../rustllvm/Linker.cpp:1:
warning: /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/llvm-project/llvm/include/llvm/ADT/ArrayRef.h: In instantiation of ‘llvm::ArrayRef<T>::ArrayRef(const std::initializer_list<_Tp>&) [with T = long unsigned int]’:
warning: /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/llvm-project/llvm/include/llvm/IR/DIBuilder.h:644:74:   required from here
warning: /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/llvm-project/llvm/include/llvm/ADT/ArrayRef.h:101:37: warning: initializing ‘llvm::ArrayRef<long unsigned int>::Data’ from ‘std::initializer_list<long unsigned int>::begin’ does not extend the lifetime of the underlying array [-Winit-list-lifetime]
warning:   101 |     : Data(Vec.begin() == Vec.end() ? (T*)nullptr : Vec.begin()),
       Fresh rustc_llvm v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_llvm)
       Fresh rustc_codegen_llvm v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_llvm)
    Finished release [optimized + debuginfo] target(s) in 0.40s
[TIMING] CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" } -- 0.422
    < CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" }
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
    > Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    < Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-dd2bfed78f6cfbad.so" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-dd2bfed78f6cfbad.so"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-2d112b68cfc40bf7.so" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/libtest-2d112b68cfc40bf7.so"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_macros-f624ba9a96c12dd9.so" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_macros-f624ba9a96c12dd9.so"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_driver-f54644be02dcf898.so" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-f54644be02dcf898.so"
    > Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    < Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_llvm-d5b5f9202bd2539f.so" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so"
Install "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVM-9-rust-1.40.0-dev-8adf9bdcc.so" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libLLVM-9-rust-1.40.0-dev-8adf9bdcc.so"
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/rustc_binary" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc"
  < Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
  > Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    > Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      > StartupObjects { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      < StartupObjects { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "4" "-v" "--release" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
       Fresh cc v1.0.35
       Fresh core v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libcore)
       Fresh build_helper v0.1.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/build_helper)
       Fresh cmake v0.1.38
       Fresh rustc-std-workspace-core v1.99.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/tools/rustc-std-workspace-core)
       Fresh compiler_builtins v0.1.18
       Fresh libc v0.2.62
       Fresh alloc v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/liballoc)
       Fresh cfg-if v0.1.8
       Fresh rustc-demangle v0.1.16
       Fresh backtrace-sys v0.1.30
       Fresh panic_abort v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libpanic_abort)
       Fresh unwind v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libunwind)
       Fresh rustc-std-workspace-alloc v1.99.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/tools/rustc-std-workspace-alloc)
       Fresh rustc_asan v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_asan)
       Fresh rustc_lsan v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_lsan)
       Fresh rustc_tsan v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_tsan)
       Fresh rustc_msan v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_msan)
       Fresh backtrace v0.3.37
       Fresh hashbrown v0.5.0
       Fresh panic_unwind v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libpanic_unwind)
       Fresh std v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd)
       Fresh rustc-std-workspace-std v1.99.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/tools/rustc-std-workspace-std)
       Fresh term v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libterm)
       Fresh proc_macro v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libproc_macro)
       Fresh unicode-width v0.1.6
       Fresh getopts v0.2.21
       Fresh test v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libtest)
    Finished release [optimized + debuginfo] target(s) in 0.51s
      c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      > StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
        c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc-1d90997e0cafecfe.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-1d90997e0cafecfe.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libbacktrace-9cb4ef20cbd0cb8b.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace-9cb4ef20cbd0cb8b.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libbacktrace_sys-9d739383e9fe3d39.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace_sys-9d739383e9fe3d39.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcfg_if-a842f723e620137e.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-a842f723e620137e.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-4de663d1225185ac.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-4de663d1225185ac.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-eaed0e18262cddc7.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-eaed0e18262cddc7.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libgetopts-8e20ef4b774ea61b.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-8e20ef4b774ea61b.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libhashbrown-16cde5b51d5e8916.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-16cde5b51d5e8916.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liblibc-72bcf69787a6b1ce.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-72bcf69787a6b1ce.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-03ba585fd63c664b.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_abort-03ba585fd63c664b.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-c76bb6ca7cb2c324.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-c76bb6ca7cb2c324.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libproc_macro-45bbd8ac3fbd9aef.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-45bbd8ac3fbd9aef.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-e033fc4c2d7c4314.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_asan-e033fc4c2d7c4314.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-1459a12c20b3e5b8.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-1459a12c20b3e5b8.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-cd6a3a06ffe3ac30.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_lsan-cd6a3a06ffe3ac30.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-4b70661be969cecf.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_msan-4b70661be969cecf.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_std_workspace_alloc-def5055ee751788a.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-def5055ee751788a.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_std_workspace_core-92af7381cf56af6a.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-92af7381cf56af6a.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_std_workspace_std-88a1c6e4defa4cf1.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_std-88a1c6e4defa4cf1.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-4caef8abb8cb9121.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_tsan-4caef8abb8cb9121.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libstd-574e0829e26b1bc3.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-574e0829e26b1bc3.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libstd-574e0829e26b1bc3.so" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-574e0829e26b1bc3.so"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libterm-1956e0945f2a2580.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-1956e0945f2a2580.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libtest-45142127b87aca65.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-45142127b87aca65.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libtest-45142127b87aca65.so" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-45142127b87aca65.so"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libunicode_width-f5b408132653f586.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunicode_width-f5b408132653f586.rlib"
Copy "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libunwind-698740435d11a20c.rlib" to "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-698740435d11a20c.rlib"
      < StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } } -- 0.559
    < Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "4" "-v" "--release" "--features" "" "--manifest-path" "/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
       Fresh cfg-if v0.1.8
       Fresh lazy_static v1.3.0
       Fresh semver-parser v0.7.0
       Fresh nodrop v0.1.12
       Fresh smallvec v0.6.10
       Fresh scopeguard v0.3.3
       Fresh memoffset v0.2.1
       Fresh unicode-xid v0.1.0
       Fresh indexmap v1.0.2
       Fresh scopeguard v1.0.0
       Fresh graphviz v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libgraphviz)
       Fresh either v1.5.0
       Fresh stable_deref_trait v1.1.0
       Fresh scoped-tls v1.0.0
       Fresh unicode-width v0.1.6
       Fresh unicode-xid v0.2.0
       Fresh termcolor v1.0.4
       Fresh annotate-snippets v0.6.1
       Fresh lazy_static v0.2.11
       Fresh rustc-demangle v0.1.16
       Fresh datafrog v2.0.1
       Fresh rustc_fs_util v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_fs_util)
       Fresh ppv-lite86 v0.2.5
       Fresh punycode v0.4.0
       Fresh remove_dir_all v0.5.2
       Fresh cc v1.0.35
       Fresh itoa v0.4.4
       Fresh once_cell v1.1.0
       Fresh semver v0.9.0
       Fresh crossbeam-utils v0.2.2
       Fresh crossbeam-utils v0.6.5
       Fresh log_settings v0.1.2
       Fresh arrayvec v0.4.7
       Fresh serialize v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libserialize)
       Fresh lock_api v0.3.1
       Fresh itertools v0.8.0
       Fresh rustc_lexer v0.1.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_lexer)
       Fresh chalk-macros v0.1.0
       Fresh c2-chacha v0.2.2
       Fresh libc v0.2.62
       Fresh rustc_version v0.2.3
       Fresh log v0.4.8
       Fresh byteorder v1.3.2
       Fresh crossbeam-epoch v0.3.1
       Fresh proc-macro2 v0.4.30
       Fresh rustc_index v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_index)
       Fresh bitflags v1.1.0
       Fresh crc32fast v1.1.2
       Fresh serde v1.0.99
       Fresh ryu v1.0.0
       Fresh rustc-hash v1.0.1
       Fresh num_cpus v1.8.0
       Fresh jobserver v0.1.16
       Fresh ena v0.13.1
       Fresh atty v0.2.11
       Fresh term_size v0.3.1
       Fresh memmap v0.6.2
       Fresh getrandom v0.1.12
       Fresh env_logger v0.7.0
       Fresh crossbeam-deque v0.2.0
       Fresh quote v0.6.12
       Fresh backtrace-sys v0.1.30
       Fresh rustc_apfloat v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_apfloat)
       Fresh miniz-sys v0.1.11
       Fresh rls-span v0.5.1
       Fresh polonius-engine v0.10.0
       Fresh chalk-engine v0.9.0
       Fresh serde_json v1.0.40
       Fresh measureme v0.3.0
       Fresh rand_core v0.5.0
       Fresh rustc-rayon-core v0.2.0
       Fresh syn v0.15.35
       Fresh backtrace v0.3.37
       Fresh parking_lot_core v0.6.2
       Fresh flate2 v1.0.6
       Fresh rls-data v0.19.0
       Fresh rustc-rayon v0.2.0
       Fresh synstructure v0.10.2
       Fresh rand_chacha v0.2.1
       Fresh parking_lot v0.9.0
       Fresh rustc_data_structures v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_data_structures)
       Fresh rustc_macros v0.1.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_macros)
       Fresh rand v0.7.0
       Fresh arena v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libarena)
       Fresh tempfile v3.1.0
       Fresh syntax_pos v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libsyntax_pos)
       Fresh rustc_target v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_target)
       Fresh rustc_errors v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_errors)
       Fresh fmt_macros v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libfmt_macros)
       Fresh syntax v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libsyntax)
       Fresh rustc v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc)
       Fresh syntax_ext v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libsyntax_ext)
       Fresh rustc_metadata v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_metadata)
       Fresh rustc_typeck v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_typeck)
       Fresh rustc_incremental v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_incremental)
       Fresh rustc_passes v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_passes)
       Fresh rustc_traits v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_traits)
       Fresh rustc_lint v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_lint)
       Fresh rustc_mir v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_mir)
       Fresh rustc_codegen_utils v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_utils)
       Fresh rustc_plugin_impl v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_plugin)
       Fresh rustc_privacy v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_privacy)
       Fresh rustc_resolve v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_resolve)
       Fresh rustc_codegen_ssa v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_ssa)
       Fresh rustc_save_analysis v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_save_analysis)
   Compiling rustc_interface v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_interface)
       Fresh rustc_plugin v0.0.0 (/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_plugin/deprecated)
     Running `/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/rustc --edition=2018 --crate-name rustc_interface src/librustc_interface/lib.rs --error-format json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C opt-level=2 -C codegen-units=1 -C debuginfo=2 -C metadata=0d169ea062d01b70 -C extra-filename=-0d169ea062d01b70 --out-dir /home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C incremental=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental -L dependency=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern log=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-1022b9df95eebe1a.rmeta --extern once_cell=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libonce_cell-6557f60babc39d8f.rmeta --extern rustc=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-ba8998efb57f0686.rmeta --extern rayon=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-3f14a06398fb8d27.rmeta --extern rustc_codegen_ssa=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_ssa-8ff5b3fb9091fa83.rmeta --extern rustc_codegen_utils=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_utils-9b595da31805269f.rmeta --extern rustc_data_structures=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-985dc1d30c7b19f5.rmeta --extern rustc_errors=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-1f0042c7f0e4a720.rmeta --extern rustc_incremental=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-d8d56e69686884cb.rmeta --extern rustc_lint=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lint-4bf5ab069f3fe1f1.rmeta --extern rustc_metadata=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_metadata-19155a172969d2ff.rmeta --extern rustc_mir=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-e9addb77fa1aa059.rmeta --extern rustc_passes=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_passes-dd83cc2cdbdbe032.rmeta --extern rustc_plugin=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_plugin_impl-89c0defd5d23fbe3.rmeta --extern rustc_privacy=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_privacy-98cbfa24e27cade1.rmeta --extern rustc_resolve=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_resolve-6ef3ba397ea451ec.rmeta --extern rustc_traits=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_traits-d2a356201ce29ff2.rmeta --extern rustc_typeck=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_typeck-e407089fd5307e47.rmeta --extern rustc_serialize=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c62857801cb33412.rmeta --extern smallvec=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-4a539e426cc999c8.rmeta --extern syntax=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-f56e8cee2d0bfce1.rmeta --extern syntax_ext=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_ext-1746e77c97422870.rmeta --extern syntax_pos=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-bd2603bbfec5f360.rmeta --extern tempfile=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-b1326e77fb6c7a44.rmeta -Zexternal-macro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic --cfg=parallel_compiler -Zbinary-dep-depinfo -L native=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-3e31d40a6e460e10/out -L native=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-56b75c4f3d592d4c/out`
rustc command: "LD_LIBRARY_PATH"="/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib:/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps:/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib" "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "--edition=2018" "--crate-name" "rustc_interface" "src/librustc_interface/lib.rs" "--error-format" "json" "--json=diagnostic-rendered-ansi" "--crate-type" "lib" "--emit=dep-info,metadata,link" "-C" "opt-level=2" "-C" "codegen-units=1" "-C" "debuginfo=2" "-C" "metadata=0d169ea062d01b70" "-C" "extra-filename=-0d169ea062d01b70" "--out-dir" "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps" "--target" "x86_64-unknown-linux-gnu" "-C" "incremental=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental" "-L" "dependency=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps" "--extern" "log=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-1022b9df95eebe1a.rmeta" "--extern" "once_cell=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libonce_cell-6557f60babc39d8f.rmeta" "--extern" "rustc=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-ba8998efb57f0686.rmeta" "--extern" "rayon=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-3f14a06398fb8d27.rmeta" "--extern" "rustc_codegen_ssa=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_ssa-8ff5b3fb9091fa83.rmeta" "--extern" "rustc_codegen_utils=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_utils-9b595da31805269f.rmeta" "--extern" "rustc_data_structures=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-985dc1d30c7b19f5.rmeta" "--extern" "rustc_errors=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-1f0042c7f0e4a720.rmeta" "--extern" "rustc_incremental=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-d8d56e69686884cb.rmeta" "--extern" "rustc_lint=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lint-4bf5ab069f3fe1f1.rmeta" "--extern" "rustc_metadata=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_metadata-19155a172969d2ff.rmeta" "--extern" "rustc_mir=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-e9addb77fa1aa059.rmeta" "--extern" "rustc_passes=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_passes-dd83cc2cdbdbe032.rmeta" "--extern" "rustc_plugin=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_plugin_impl-89c0defd5d23fbe3.rmeta" "--extern" "rustc_privacy=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_privacy-98cbfa24e27cade1.rmeta" "--extern" "rustc_resolve=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_resolve-6ef3ba397ea451ec.rmeta" "--extern" "rustc_traits=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_traits-d2a356201ce29ff2.rmeta" "--extern" "rustc_typeck=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_typeck-e407089fd5307e47.rmeta" "--extern" "rustc_serialize=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c62857801cb33412.rmeta" "--extern" "smallvec=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-4a539e426cc999c8.rmeta" "--extern" "syntax=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-f56e8cee2d0bfce1.rmeta" "--extern" "syntax_ext=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_ext-1746e77c97422870.rmeta" "--extern" "syntax_pos=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-bd2603bbfec5f360.rmeta" "--extern" "tempfile=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-b1326e77fb6c7a44.rmeta" "-Zexternal-macro-backtrace" "-Clink-args=-Wl,-rpath,$ORIGIN/../lib" "-Wrust_2018_idioms" "-Wunused_lifetimes" "-Zunstable-options" "-Wrustc::internal" "-Cprefer-dynamic" "--cfg=parallel_compiler" "-Zbinary-dep-depinfo" "-L" "native=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-3e31d40a6e460e10/out" "-L" "native=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-56b75c4f3d592d4c/out" "--sysroot" "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1" "-C" "debug-assertions=n" "-Z" "force-unstable-if-unmarked"
sysroot: "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1"
libdir: "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib"
error: internal compiler error: src/librustc/ich/impls_ty.rs:100: StableHasher: unexpected region '_#4r

thread '<unnamed>' panicked at 'Box<Any>', src/librustc_errors/lib.rs:912:9
stack backtrace:
    Building [====================================================>  ] 154/157: rustc_interface                                                                                               
   0: backtrace::backtrace::libunwind::trace
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:76
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:60
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1028
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1412
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:64
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:196
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:210
  10: rustc_driver::report_ice
             at src/librustc_driver/lib.rs:1187
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:477
  12: std::panicking::begin_panic
             at ./src/libstd/panicking.rs:407
  13: rustc_errors::HandlerInner::bug
             at src/librustc_errors/lib.rs:912
  14: rustc_errors::Handler::bug
             at src/librustc_errors/lib.rs:684
  15: rustc::util::bug::opt_span_bug_fmt::{{closure}}
             at src/librustc/util/bug.rs:36
  16: rustc::ty::context::tls::with_opt::{{closure}}
             at src/librustc/ty/context.rs:1982
  17: rustc::ty::context::tls::with_context_opt
             at src/librustc/ty/context.rs:1932
  18: rustc::ty::context::tls::with_opt
             at src/librustc/ty/context.rs:1982
  19: rustc::util::bug::opt_span_bug_fmt
             at src/librustc/util/bug.rs:32
  20: rustc::util::bug::bug_fmt
             at src/librustc/util/bug.rs:12
  21: rustc::ich::impls_ty::<impl rustc_data_structures::stable_hasher::HashStable<rustc::ich::hcx::StableHashingContext> for rustc::ty::sty::RegionKind>::hash_stable
             at src/librustc/ich/impls_ty.rs:100
  22: <&T as rustc_data_structures::stable_hasher::HashStable<CTX>>::hash_stable
             at ./src/librustc_data_structures/stable_hasher.rs:421
  23: rustc::ty::sty::_DERIVE_rustc_data_structures_stable_hasher_HashStable_rustc_ich_StableHashingContext_ctx_FOR_TyKind::<impl rustc_data_structures::stable_hasher::HashStable<rustc::ich::hcx::StableHashingContext> for rustc::ty::sty::TyKind>::hash_stable
             at src/librustc/ty/sty.rs:89
  24: <&T as rustc_data_structures::stable_hasher::HashStable<CTX>>::hash_stable
             at ./src/librustc_data_structures/stable_hasher.rs:421
  25: rustc::ty::context::_DERIVE_rustc_data_structures_stable_hasher_HashStable_rustc_ich_StableHashingContext_ctx_FOR_GeneratorInteriorTypeCause::<impl rustc_data_structures::stable_hasher::HashStable<rustc::ich::hcx::StableHashingContext> for rustc::ty::context::GeneratorInteriorTypeCause>::hash_stable
             at src/librustc/ty/context.rs:309
  26: <[T] as rustc_data_structures::stable_hasher::HashStable<CTX>>::hash_stable
             at ./src/librustc_data_structures/stable_hasher.rs:289
  27: <alloc::vec::Vec<T> as rustc_data_structures::stable_hasher::HashStable<CTX>>::hash_stable
             at ./src/librustc_data_structures/stable_hasher.rs:297
  28: <rustc::ty::context::TypeckTables as rustc_data_structures::stable_hasher::HashStable<rustc::ich::hcx::StableHashingContext>>::hash_stable::{{closure}}
             at src/librustc/ty/context.rs:810
  29: rustc::ich::hcx::StableHashingContext::with_node_id_hashing_mode
             at src/librustc/ich/hcx.rs:129
  30: <rustc::ty::context::TypeckTables as rustc_data_structures::stable_hasher::HashStable<rustc::ich::hcx::StableHashingContext>>::hash_stable
             at src/librustc/ty/context.rs:769
  31: <&T as rustc_data_structures::stable_hasher::HashStable<CTX>>::hash_stable
             at ./src/librustc_data_structures/stable_hasher.rs:421
  32: <&T as rustc_data_structures::stable_hasher::HashStable<CTX>>::hash_stable
             at ./src/librustc_data_structures/stable_hasher.rs:421
  33: rustc::dep_graph::graph::hash_result
             at src/librustc/dep_graph/graph.rs:88
  34: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::hash_result
             at src/librustc/ty/query/plumbing.rs:1011
  35: core::ops::function::FnOnce::call_once
             at ./src/libcore/ops/function.rs:227
  36: rustc::dep_graph::graph::DepGraph::with_task_impl
             at ./src/librustc/dep_graph/graph.rs:286
  37: rustc::dep_graph::graph::DepGraph::with_task
             at ./src/librustc/dep_graph/graph.rs:202
  38: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:565
  39: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:278
  40: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1854
  41: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
  42: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1761
  43: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1853
  44: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:277
  45: rustc::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc/ty/context.rs:1960
  46: rustc::ty::context::tls::with_context::{{closure}}
             at ./src/librustc/ty/context.rs:1943
  47: rustc::ty::context::tls::with_context_opt
             at ./src/librustc/ty/context.rs:1932
  48: rustc::ty::context::tls::with_context
             at ./src/librustc/ty/context.rs:1943
  49: rustc::ty::context::tls::with_related_context
             at ./src/librustc/ty/context.rs:1956
  50: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at ./src/librustc/ty/query/plumbing.rs:266
  51: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:557
  52: rustc::ty::query::plumbing::with_diagnostics
             at ./src/librustc/ty/query/plumbing.rs:211
  53: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at ./src/librustc/ty/query/plumbing.rs:556
  54: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at ./src/librustc/ty/query/plumbing.rs:434
  55: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:1003
  56: rustc::ty::query::__query_compute::typeck_tables_of
             at ./src/librustc/ty/query/plumbing.rs:954
  57: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
             at ./src/librustc/ty/query/plumbing.rs:995
  58: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}::{{closure}}
             at ./src/librustc/dep_graph/graph.rs:277
  59: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1854
  60: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
  61: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1761
  62: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1853
  63: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}
             at ./src/librustc/dep_graph/graph.rs:276
  64: rustc::ty::context::tls::with_context::{{closure}}
             at ./src/librustc/ty/context.rs:1943
  65: rustc::ty::context::tls::with_context_opt
             at ./src/librustc/ty/context.rs:1932
  66: rustc::ty::context::tls::with_context
             at ./src/librustc/ty/context.rs:1943
  67: rustc::dep_graph::graph::DepGraph::with_task_impl
             at ./src/librustc/dep_graph/graph.rs:270
  68: rustc::dep_graph::graph::DepGraph::with_task
             at ./src/librustc/dep_graph/graph.rs:202
  69: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:565
  70: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:278
  71: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1854
  72: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
  73: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1761
  74: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1853
  75: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:277
  76: rustc::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc/ty/context.rs:1960
  77: rustc::ty::context::tls::with_context::{{closure}}
             at ./src/librustc/ty/context.rs:1943
  78: rustc::ty::context::tls::with_context_opt
             at ./src/librustc/ty/context.rs:1932
  79: rustc::ty::context::tls::with_context
             at ./src/librustc/ty/context.rs:1943
  80: rustc::ty::context::tls::with_related_context
             at ./src/librustc/ty/context.rs:1956
  81: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at ./src/librustc/ty/query/plumbing.rs:266
  82: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:557
  83: rustc::ty::query::plumbing::with_diagnostics
             at ./src/librustc/ty/query/plumbing.rs:211
  84: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at ./src/librustc/ty/query/plumbing.rs:556
  85: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at ./src/librustc/ty/query/plumbing.rs:434
  86: rustc::ty::query::TyCtxtAt::typeck_tables_of
             at ./src/librustc/ty/query/plumbing.rs:1080
  87: rustc::ty::query::<impl rustc::ty::context::TyCtxt>::typeck_tables_of
             at ./src/librustc/ty/query/plumbing.rs:1072
  88: rustc_typeck::collect::checked_type_of
             at src/librustc_typeck/collect.rs:1362
  89: rustc_typeck::collect::type_of
             at src/librustc_typeck/collect.rs:1144
  90: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::type_of>::compute::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:1003
  91: rustc::ty::query::__query_compute::type_of
             at ./src/librustc/ty/query/plumbing.rs:954
  92: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::type_of>::compute
             at ./src/librustc/ty/query/plumbing.rs:995
  93: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}::{{closure}}
             at ./src/librustc/dep_graph/graph.rs:277
  94: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1854
  95: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
  96: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1761
  97: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1853
  98: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}
             at ./src/librustc/dep_graph/graph.rs:276
  99: rustc::ty::context::tls::with_context::{{closure}}
             at ./src/librustc/ty/context.rs:1943
 100: rustc::ty::context::tls::with_context_opt
             at ./src/librustc/ty/context.rs:1932
 101: rustc::ty::context::tls::with_context
             at ./src/librustc/ty/context.rs:1943
 102: rustc::dep_graph::graph::DepGraph::with_task_impl
             at ./src/librustc/dep_graph/graph.rs:270
 103: rustc::dep_graph::graph::DepGraph::with_task
             at ./src/librustc/dep_graph/graph.rs:202
 104: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:565
 105: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:278
 106: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1854
 107: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
 108: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1761
 109: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1853
 110: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:277
 111: rustc::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc/ty/context.rs:1960
 112: rustc::ty::context::tls::with_context::{{closure}}
             at ./src/librustc/ty/context.rs:1943
 113: rustc::ty::context::tls::with_context_opt
             at ./src/librustc/ty/context.rs:1932
 114: rustc::ty::context::tls::with_context
             at ./src/librustc/ty/context.rs:1943
 115: rustc::ty::context::tls::with_related_context
             at ./src/librustc/ty/context.rs:1956
 116: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at ./src/librustc/ty/query/plumbing.rs:266
 117: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:557
 118: rustc::ty::query::plumbing::with_diagnostics
             at ./src/librustc/ty/query/plumbing.rs:211
 119: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at ./src/librustc/ty/query/plumbing.rs:556
 120: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at ./src/librustc/ty/query/plumbing.rs:434
 121: rustc::ty::query::TyCtxtAt::type_of
             at ./src/librustc/ty/query/plumbing.rs:1080
 122: rustc::ty::query::<impl rustc::ty::context::TyCtxt>::type_of
             at ./src/librustc/ty/query/plumbing.rs:1072
 123: <rustc_typeck::collect::CollectItemTypesVisitor as rustc::hir::intravisit::Visitor>::visit_expr
             at src/librustc_typeck/collect.rs:141
 124: rustc::hir::intravisit::walk_expr
             at ./<::syntax::visit::walk_list macros>:2
 125: rustc::hir::intravisit::walk_local
             at ./<::syntax::visit::walk_list macros>:2
 126: rustc::hir::intravisit::walk_block
             at ./<::syntax::visit::walk_list macros>:2
 127: rustc::hir::intravisit::Visitor::visit_fn
             at ./src/librustc/hir/intravisit.rs:293
 128: rustc::hir::intravisit::walk_item
             at ./src/librustc/hir/intravisit.rs:485
 129: <rustc_typeck::collect::CollectItemTypesVisitor as rustc::hir::intravisit::Visitor>::visit_item
             at src/librustc_typeck/collect.rs:114
 130: rustc::hir::map::Map::visit_item_likes_in_module
             at ./src/librustc/hir/map/mod.rs:578
 131: rustc_typeck::collect::collect_mod_item_types
             at src/librustc_typeck/collect.rs:57
 132: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::collect_mod_item_types>::compute::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:1003
 133: rustc::ty::query::__query_compute::collect_mod_item_types
             at ./src/librustc/ty/query/plumbing.rs:954
 134: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::collect_mod_item_types>::compute
             at ./src/librustc/ty/query/plumbing.rs:995
 135: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}::{{closure}}
             at ./src/librustc/dep_graph/graph.rs:277
 136: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1854
 137: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
 138: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1761
 139: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1853
 140: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}
             at ./src/librustc/dep_graph/graph.rs:276
 141: rustc::ty::context::tls::with_context::{{closure}}
             at ./src/librustc/ty/context.rs:1943
 142: rustc::ty::context::tls::with_context_opt
             at ./src/librustc/ty/context.rs:1932
 143: rustc::ty::context::tls::with_context
             at ./src/librustc/ty/context.rs:1943
 144: rustc::dep_graph::graph::DepGraph::with_task_impl
             at ./src/librustc/dep_graph/graph.rs:270
 145: rustc::dep_graph::graph::DepGraph::with_task
             at ./src/librustc/dep_graph/graph.rs:202
 146: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:565
 147: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:278
 148: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1854
 149: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
 150: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1761
 151: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1853
 152: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:277
 153: rustc::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc/ty/context.rs:1960
 154: rustc::ty::context::tls::with_context::{{closure}}
             at ./src/librustc/ty/context.rs:1943
 155: rustc::ty::context::tls::with_context_opt
             at ./src/librustc/ty/context.rs:1932
 156: rustc::ty::context::tls::with_context
             at ./src/librustc/ty/context.rs:1943
 157: rustc::ty::context::tls::with_related_context
             at ./src/librustc/ty/context.rs:1956
 158: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at ./src/librustc/ty/query/plumbing.rs:266
 159: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:557
 160: rustc::ty::query::plumbing::with_diagnostics
             at ./src/librustc/ty/query/plumbing.rs:211
 161: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at ./src/librustc/ty/query/plumbing.rs:556
 162: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at ./src/librustc/ty/query/plumbing.rs:434
 163: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query
             at ./src/librustc/ty/query/plumbing.rs:619
 164: rustc::ty::query::TyCtxtEnsure::collect_mod_item_types
             at ./src/librustc/ty/query/plumbing.rs:1031
 165: rustc_typeck::check_crate::{{closure}}::{{closure}}
             at src/librustc_typeck/lib.rs:306
 166: rustc::util::common::time_ext
             at ./src/librustc/util/common.rs:116
 167: rustc::util::common::time
             at ./src/librustc/util/common.rs:110
 168: rustc_typeck::check_crate::{{closure}}
             at src/librustc_typeck/lib.rs:304
 169: rustc::session::Session::track_errors
             at ./src/librustc/session/mod.rs:334
 170: rustc_typeck::check_crate
             at src/librustc_typeck/lib.rs:303
 171: rustc_interface::passes::analysis
             at src/librustc_interface/passes.rs:915
 172: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::analysis>::compute::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:1003
 173: rustc::ty::query::__query_compute::analysis
             at ./src/librustc/ty/query/plumbing.rs:954
 174: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}::{{closure}}
             at ./src/librustc/dep_graph/graph.rs:277
 175: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1854
 176: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
 177: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1761
 178: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1853
 179: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}
             at ./src/librustc/dep_graph/graph.rs:276
 180: rustc::ty::context::tls::with_context::{{closure}}
             at ./src/librustc/ty/context.rs:1943
 181: rustc::ty::context::tls::with_context_opt
             at ./src/librustc/ty/context.rs:1932
 182: rustc::ty::context::tls::with_context
             at ./src/librustc/ty/context.rs:1943
 183: rustc::dep_graph::graph::DepGraph::with_task_impl
             at ./src/librustc/dep_graph/graph.rs:270
 184: rustc::dep_graph::graph::DepGraph::with_eval_always_task
             at ./src/librustc/dep_graph/graph.rs:387
 185: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:559
 186: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:278
 187: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1854
 188: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
 189: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1761
 190: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1853
 191: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:277
 192: rustc::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc/ty/context.rs:1960
 193: rustc::ty::context::tls::with_context::{{closure}}
             at ./src/librustc/ty/context.rs:1943
 194: rustc::ty::context::tls::with_context_opt
             at ./src/librustc/ty/context.rs:1932
 195: rustc::ty::context::tls::with_context
             at ./src/librustc/ty/context.rs:1943
 196: rustc::ty::context::tls::with_related_context
             at ./src/librustc/ty/context.rs:1956
 197: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at ./src/librustc/ty/query/plumbing.rs:266
 198: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:557
 199: rustc::ty::query::plumbing::with_diagnostics
             at ./src/librustc/ty/query/plumbing.rs:211
 200: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at ./src/librustc/ty/query/plumbing.rs:556
 201: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at ./src/librustc/ty/query/plumbing.rs:434
 202: rustc::ty::query::TyCtxtAt::analysis
             at ./src/librustc/ty/query/plumbing.rs:1080
 203: rustc::ty::query::<impl rustc::ty::context::TyCtxt>::analysis
             at ./src/librustc/ty/query/plumbing.rs:1072
 204: rustc_driver::run_compiler::{{closure}}::{{closure}}
             at src/librustc_driver/lib.rs:377
 205: rustc_interface::passes::BoxedGlobalCtxt::enter::{{closure}}::{{closure}}
             at ./src/librustc_interface/passes.rs:809
 206: rustc::ty::context::tls::enter_global::{{closure}}
             at ./src/librustc/ty/context.rs:1886
 207: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1854
 208: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
 209: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1761
 210: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1853
 211: rustc::ty::context::tls::enter_global
             at ./src/librustc/ty/context.rs:1885
 212: rustc_interface::passes::BoxedGlobalCtxt::enter::{{closure}}
             at ./src/librustc_interface/passes.rs:809
 213: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
             at ./<::rustc_data_structures::box_region::declare_box_region_type macros>:21
 214: rustc_interface::passes::create_global_ctxt::{{closure}}
             at src/librustc_interface/passes.rs:873
 215: rustc_data_structures::box_region::PinnedGenerator<I,A,R>::access
             at ./src/librustc_data_structures/box_region.rs:52
 216: rustc_interface::passes::BoxedGlobalCtxt::access
             at ./<::rustc_data_structures::box_region::declare_box_region_type macros>:24
 217: rustc_interface::passes::BoxedGlobalCtxt::enter
             at ./src/librustc_interface/passes.rs:809
 218: rustc_driver::run_compiler::{{closure}}
             at src/librustc_driver/lib.rs:377
 219: rustc_interface::interface::run_compiler_in_existing_thread_pool
             at ./src/librustc_interface/interface.rs:122
 220: rustc_interface::interface::run_compiler::{{closure}}
             at ./src/librustc_interface/interface.rs:141
 221: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:219
 222: rustc_rayon_core::thread_pool::ThreadPool::install::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/thread_pool/mod.rs:160
 223: rustc_rayon_core::registry::Registry::in_worker_cold::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/registry.rs:395
 224: <rustc_rayon_core::job::StackJob<L,F,R> as rustc_rayon_core::job::Job>::execute::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/job.rs:121
 225: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at ./src/libstd/panic.rs:315
 226: std::panicking::try::do_call
             at ./src/libstd/panicking.rs:292
 227: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:80
 228: std::panicking::try
             at ./src/libstd/panicking.rs:271
 229: std::panic::catch_unwind
             at ./src/libstd/panic.rs:394
 230: rustc_rayon_core::unwind::halt_unwinding
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/unwind.rs:19
 231: <rustc_rayon_core::job::StackJob<L,F,R> as rustc_rayon_core::job::Job>::execute
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/job.rs:121
 232: rustc_rayon_core::job::JobRef::execute
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/job.rs:62
 233: rustc_rayon_core::registry::WorkerThread::execute
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/registry.rs:657
 234: rustc_rayon_core::registry::WorkerThread::wait_until_cold
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/registry.rs:637
 235: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:235
 236: scoped_tls::ScopedKey<T>::set
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 237: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:235
 238: rustc::ty::context::tls::with_thread_locals::{{closure}}::{{closure}}
             at ./src/librustc/ty/context.rs:1842
 239: std::thread::local::LocalKey<T>::try_with
             at ./src/libstd/thread/local.rs:262
 240: std::thread::local::LocalKey<T>::with
             at ./src/libstd/thread/local.rs:239
 241: rustc::ty::context::tls::with_thread_locals::{{closure}}
             at ./src/librustc/ty/context.rs:1834
 242: std::thread::local::LocalKey<T>::try_with
             at ./src/libstd/thread/local.rs:262
 243: std::thread::local::LocalKey<T>::with
             at ./src/libstd/thread/local.rs:239
 244: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:234
 245: scoped_tls::ScopedKey<T>::set
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 246: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:230
 247: scoped_tls::ScopedKey<T>::set
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 248: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:229
 249: rustc_rayon_core::thread_pool::ThreadPool::scoped_pool::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/thread_pool/mod.rs:104
 250: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:80
 251: std::panicking::try
             at ./src/libstd/panicking.rs:271
 252: std::panic::catch_unwind
             at ./src/libstd/panic.rs:394
 253: rustc_rayon_core::unwind::halt_unwinding
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/unwind.rs:19
 254: rustc_rayon_core::registry::main_loop
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/registry.rs:747
 255: rustc_rayon_core::registry::Registry::new::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/registry.rs:145
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0-dev (702b45e40 2019-10-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z external-macro-backtrace -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=2 -C codegen-units=1 -C debuginfo=2 -C incremental -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C debug-assertions=n --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck_tables_of] processing `passes::configure_and_expand`
#1 [typeck_tables_of] processing `passes::configure_and_expand::{{closure}}#0`
#2 [type_of] processing `passes::configure_and_expand::{{closure}}#0`
#3 [collect_mod_item_types] collecting item types in module `passes`
#4 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error

[RUSTC-TIMING] rustc_interface test:false 6.508
error: could not compile `rustc_interface`.

Caused by:
  process didn't exit successfully: `/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/rustc --edition=2018 --crate-name rustc_interface src/librustc_interface/lib.rs --error-format json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C opt-level=2 -C codegen-units=1 -C debuginfo=2 -C metadata=0d169ea062d01b70 -C extra-filename=-0d169ea062d01b70 --out-dir /home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C incremental=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental -L dependency=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern log=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-1022b9df95eebe1a.rmeta --extern once_cell=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libonce_cell-6557f60babc39d8f.rmeta --extern rustc=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-ba8998efb57f0686.rmeta --extern rayon=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-3f14a06398fb8d27.rmeta --extern rustc_codegen_ssa=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_ssa-8ff5b3fb9091fa83.rmeta --extern rustc_codegen_utils=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_utils-9b595da31805269f.rmeta --extern rustc_data_structures=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-985dc1d30c7b19f5.rmeta --extern rustc_errors=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-1f0042c7f0e4a720.rmeta --extern rustc_incremental=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-d8d56e69686884cb.rmeta --extern rustc_lint=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lint-4bf5ab069f3fe1f1.rmeta --extern rustc_metadata=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_metadata-19155a172969d2ff.rmeta --extern rustc_mir=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-e9addb77fa1aa059.rmeta --extern rustc_passes=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_passes-dd83cc2cdbdbe032.rmeta --extern rustc_plugin=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_plugin_impl-89c0defd5d23fbe3.rmeta --extern rustc_privacy=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_privacy-98cbfa24e27cade1.rmeta --extern rustc_resolve=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_resolve-6ef3ba397ea451ec.rmeta --extern rustc_traits=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_traits-d2a356201ce29ff2.rmeta --extern rustc_typeck=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_typeck-e407089fd5307e47.rmeta --extern rustc_serialize=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c62857801cb33412.rmeta --extern smallvec=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-4a539e426cc999c8.rmeta --extern syntax=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-f56e8cee2d0bfce1.rmeta --extern syntax_ext=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_ext-1746e77c97422870.rmeta --extern syntax_pos=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-bd2603bbfec5f360.rmeta --extern tempfile=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-b1326e77fb6c7a44.rmeta -Zexternal-macro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic --cfg=parallel_compiler -Zbinary-dep-depinfo -L native=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-3e31d40a6e460e10/out -L native=/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-56b75c4f3d592d4c/out` (exit code: 101)
command did not execute successfully: "/home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "4" "-v" "--release" "--features" "" "--manifest-path" "/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
Traceback (most recent call last):
  File "./x.py", line 11, in <module>
    bootstrap.main()
  File "/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/bootstrap/bootstrap.py", line 900, in main
    bootstrap(help_triggered)
  File "/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/bootstrap/bootstrap.py", line 886, in bootstrap
    run(args, env=env, verbose=build.verbose)
  File "/home/user/build/2nonpkgs/rust.stuff/rust/rust/src/bootstrap/bootstrap.py", line 141, in run
    raise RuntimeError(err)
RuntimeError: failed to run: /home/user/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/bootstrap build -vv -j 4

real	0m17.336s
user	0m9.125s
sys	0m6.280s

Compile(failed?:1) took 17 secs aka 17 seconds. date='Wed 02 Oct 2019 04:37:21 AM CEST'
Cleaning up

^ nproc='4' /proc/loadavg='3.61 5.39 5.72 4/554 349028' /proc/cmdline='BOOT_IMAGE=/boot/vmlinuz-linux-stable root=UUID=274eb529-31f7-4bd2-8a48-950fb86ae385 rw root_trim=yes rd.luks.allow-discards rd.luks.options=discard ipv6.disable=1 ipv6.disable_ipv6=1 ipv6.autoconf=0 loglevel=15 log_buf_len=16M ignore_loglevel printk.always_kmsg_dump=y printk.time=y printk.devkmsg=on mminit_loglevel=4 memory_corruption_check=1 fbcon=scrollback:4096k fbcon=font:ProFont6x11 net.ifnames=0 nolvm dobtrfs console=tty1 earlyprintk=vga audit=0 systemd.log_target=kmsg systemd.journald.forward_to_console=1 enforcing=0 udev.children-max=1256 rd.udev.children-max=1256 nohz=on oops=panic crashkernel=256M panic=0 print_fatal_signals=1 page_poison=1 psi=1 sysrq_always_enabled random.trust_cpu=off logo.nologo lpj=0 mce=bootlog reboot=force,cold noexec=on nohibernate scsi_mod.use_blk_mq=1 consoleblank=120 mitigations=off nospectre_v1 nospectre_v2 spectre_v2=off nospec_store_bypass_disable kvm-intel.vmentry_l1d_flush=never l1tf=off nopti pti=off no_stf_barrier noibrs noibpb ssbd=force-off spectre_v2_user=off noretpoline mds=off rd.log=all noefi cpuidle.governor=menu zram.num_devices=3 zswap.enabled=0 zswap.same_filled_pages_enabled=1 zswap.compressor=zstd zswap.max_pool_percent=40 zswap.zpool=z3fold vsyscall=none acpi_backlight=vendor CPUunderclocking tsc=unstable radeon.audio=0 radeon.lockup_timeout=999000 radeon.test=0 radeon.agpmode=-1 radeon.benchmark=0 radeon.tv=0 radeon.hard_reset=1 radeon.msi=1 radeon.pcie_gen2=-1 radeon.no_wb=1 radeon.dynclks=0 radeon.r4xx_atom=0 radeonfb radeon.fastfb=1 radeon.dpm=1 radeon.runpm=1 radeon.modeset=1 radeon.aspm=0 pcie_aspm=off rcu_nocbs=1-3'

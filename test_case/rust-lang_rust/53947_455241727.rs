rust
time PATH="${HOME}/build/1packages/4used/chro/4_chromium-dev-git/makepkg_pacman/chromium-dev/src/python-path/:${PATH}" python2 ./x.py install $verbose --incremental -j $threads
./go:65+ PATH=/home/xftroxgpx/build/1packages/4used/chro/4_chromium-dev-git/makepkg_pacman/chromium-dev/src/python-path/:/opt/google-cloud-sdk/bin:/opt/google-cloud-sdk/bin:/usr/local/sbin:/usr/local/bin:/usr/bin:/opt/depot_tools:/usr/bin/site_perl:/usr/bin/vendor_perl:/usr/bin/core_perl:/home/xftroxgpx/bin:/home/xftroxgpx/build/2nonpkgs/rust.stuff/racer/target/release:/home/xftroxgpx/build/2nonpkgs/rust.stuff/chars/target/debug:/home/xftroxgpx/build/2nonpkgs/rust.stuff/chars/target/release:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rustfmt/target/release:/home/xftroxgpx/.cargo/bin:/opt/depot_tools:/usr/bin/site_perl:/usr/bin/vendor_perl:/usr/bin/core_perl:/home/xftroxgpx/bin:/home/xftroxgpx/build/2nonpkgs/rust.stuff/racer/target/release:/home/xftroxgpx/build/2nonpkgs/rust.stuff/chars/target/debug:/home/xftroxgpx/build/2nonpkgs/rust.stuff/chars/target/release:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rustfmt/target/release:/home/xftroxgpx/.cargo/bin
./go:65+ python2 ./x.py install -vv --incremental -j 4
Updating only changed submodules
Submodules updated in 0.10 seconds
running: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/bootstrap/Cargo.toml --verbose
       Fresh unicode-xid v0.1.0
       Fresh cfg-if v0.1.6
       Fresh fixedbitset v0.1.9
       Fresh ordermap v0.3.5
       Fresh itoa v0.4.3
       Fresh cc v1.0.28
       Fresh lazy_static v0.2.11
       Fresh build_helper v0.1.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/build_helper)
       Fresh getopts v0.2.17
       Fresh petgraph v0.4.13
       Fresh cmake v0.1.33
       Fresh proc-macro2 v0.4.24
       Fresh ryu v0.2.7
       Fresh serde v1.0.82
       Fresh libc v0.2.46
       Fresh quote v0.6.10
       Fresh serde_json v1.0.33
       Fresh toml v0.4.10
       Fresh time v0.1.40
       Fresh filetime v0.2.4
       Fresh num_cpus v1.8.0
       Fresh syn v0.15.22
       Fresh serde_derive v1.0.81
       Fresh bootstrap v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/bootstrap)
    Finished dev [unoptimized] target(s) in 1.08s
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
[TIMING] Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 0.135
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
      < RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "unstable-book", src: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/md-doc" }
    < UnstableBook { target: "x86_64-unknown-linux-gnu" }
    c UnstableBookGen { target: "x86_64-unknown-linux-gnu" }
    c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    > TheBook { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", name: "book" }
      > Rustbook { target: "x86_64-unknown-linux-gnu", name: "book" }
        > RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "book", src: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/doc" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        < RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "book", src: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/doc" }
      < Rustbook { target: "x86_64-unknown-linux-gnu", name: "book" }
      > Rustbook { target: "x86_64-unknown-linux-gnu", name: "book/first-edition" }
        > RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "book/first-edition", src: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/doc" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        < RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "book/first-edition", src: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/doc" }
      < Rustbook { target: "x86_64-unknown-linux-gnu", name: "book/first-edition" }
      > Rustbook { target: "x86_64-unknown-linux-gnu", name: "book/second-edition" }
        > RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "book/second-edition", src: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/doc" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        < RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "book/second-edition", src: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/doc" }
      < Rustbook { target: "x86_64-unknown-linux-gnu", name: "book/second-edition" }
      > Rustbook { target: "x86_64-unknown-linux-gnu", name: "book/2018-edition" }
        > RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "book/2018-edition", src: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/doc" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        < RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "book/2018-edition", src: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/doc" }
      < Rustbook { target: "x86_64-unknown-linux-gnu", name: "book/2018-edition" }
      > Standalone { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        > Rustdoc { host: "x86_64-unknown-linux-gnu" }
          c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
          c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
          > Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
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
          c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
          c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
          c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
          c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
          c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
          c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
          c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
        < Rustdoc { host: "x86_64-unknown-linux-gnu" }
        c Rustdoc { host: "x86_64-unknown-linux-gnu" }
        c Rustdoc { host: "x86_64-unknown-linux-gnu" }
        c Rustdoc { host: "x86_64-unknown-linux-gnu" }
        c Rustdoc { host: "x86_64-unknown-linux-gnu" }
        c Rustdoc { host: "x86_64-unknown-linux-gnu" }
        c Rustdoc { host: "x86_64-unknown-linux-gnu" }
        c Rustdoc { host: "x86_64-unknown-linux-gnu" }
        c Rustdoc { host: "x86_64-unknown-linux-gnu" }
        c Rustdoc { host: "x86_64-unknown-linux-gnu" }
        c Rustdoc { host: "x86_64-unknown-linux-gnu" }
        c Rustdoc { host: "x86_64-unknown-linux-gnu" }
        c Rustdoc { host: "x86_64-unknown-linux-gnu" }
        c Rustdoc { host: "x86_64-unknown-linux-gnu" }
        c Rustdoc { host: "x86_64-unknown-linux-gnu" }
        c Rustdoc { host: "x86_64-unknown-linux-gnu" }
        c Rustdoc { host: "x86_64-unknown-linux-gnu" }
        c Rustdoc { host: "x86_64-unknown-linux-gnu" }
        c Rustdoc { host: "x86_64-unknown-linux-gnu" }
        c Rustdoc { host: "x86_64-unknown-linux-gnu" }
        c Rustdoc { host: "x86_64-unknown-linux-gnu" }
        c Rustdoc { host: "x86_64-unknown-linux-gnu" }
        c Rustdoc { host: "x86_64-unknown-linux-gnu" }
        c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      < Standalone { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
    < TheBook { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", name: "book" }
    c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Standalone { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    > Std { stage: 2, target: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    < Std { stage: 2, target: "x86_64-unknown-linux-gnu" }
    > Test { stage: 2, target: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Std { stage: 2, target: "x86_64-unknown-linux-gnu" }
      c Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    < Test { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Test { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Test { stage: 2, target: "x86_64-unknown-linux-gnu" }
    > WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Std { stage: 2, target: "x86_64-unknown-linux-gnu" }
      c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Rustdoc { host: "x86_64-unknown-linux-gnu" }
      c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
      c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    < WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c WhitelistedRustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    > Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    < Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    > Rustdoc { stage: 2, target: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
    < Rustdoc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    c Rustdoc { stage: 2, target: "x86_64-unknown-linux-gnu" }
    > ErrorIndex { target: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      > ErrorIndex { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
        > ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "error_index_generator", path: "src/tools/error_index_generator", mode: ToolRustc, is_optional_tool: false, source_type: InTree, extra_features: [] }
          c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
          c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
          c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
          c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
          c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
          c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
        < ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "error_index_generator", path: "src/tools/error_index_generator", mode: ToolRustc, is_optional_tool: false, source_type: InTree, extra_features: [] }
      < ErrorIndex { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
      c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    < ErrorIndex { target: "x86_64-unknown-linux-gnu" }
    > Nomicon { target: "x86_64-unknown-linux-gnu" }
      > Rustbook { target: "x86_64-unknown-linux-gnu", name: "nomicon" }
        > RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "nomicon", src: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/doc" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        < RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "nomicon", src: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/doc" }
      < Rustbook { target: "x86_64-unknown-linux-gnu", name: "nomicon" }
    < Nomicon { target: "x86_64-unknown-linux-gnu" }
    > Reference { target: "x86_64-unknown-linux-gnu" }
      > Rustbook { target: "x86_64-unknown-linux-gnu", name: "reference" }
        > RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "reference", src: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/doc" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        < RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "reference", src: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/doc" }
      < Rustbook { target: "x86_64-unknown-linux-gnu", name: "reference" }
    < Reference { target: "x86_64-unknown-linux-gnu" }
    > RustdocBook { target: "x86_64-unknown-linux-gnu" }
      > Rustbook { target: "x86_64-unknown-linux-gnu", name: "rustdoc" }
        > RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "rustdoc", src: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/doc" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        < RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "rustdoc", src: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/doc" }
      < Rustbook { target: "x86_64-unknown-linux-gnu", name: "rustdoc" }
    < RustdocBook { target: "x86_64-unknown-linux-gnu" }
    > RustByExample { target: "x86_64-unknown-linux-gnu" }
      > Rustbook { target: "x86_64-unknown-linux-gnu", name: "rust-by-example" }
        > RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "rust-by-example", src: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/doc" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        < RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "rust-by-example", src: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/doc" }
      < Rustbook { target: "x86_64-unknown-linux-gnu", name: "rust-by-example" }
    < RustByExample { target: "x86_64-unknown-linux-gnu" }
    > RustcBook { target: "x86_64-unknown-linux-gnu" }
      > Rustbook { target: "x86_64-unknown-linux-gnu", name: "rustc" }
        > RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "rustc", src: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/doc" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        < RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "rustc", src: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/doc" }
      < Rustbook { target: "x86_64-unknown-linux-gnu", name: "rustc" }
    < RustcBook { target: "x86_64-unknown-linux-gnu" }
    > CargoBook { target: "x86_64-unknown-linux-gnu", name: "cargo" }
      c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
    < CargoBook { target: "x86_64-unknown-linux-gnu", name: "cargo" }
    > EditionGuide { target: "x86_64-unknown-linux-gnu" }
      > Rustbook { target: "x86_64-unknown-linux-gnu", name: "edition-guide" }
        > RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "edition-guide", src: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/doc" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
          c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        < RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "edition-guide", src: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/doc" }
      < Rustbook { target: "x86_64-unknown-linux-gnu", name: "edition-guide" }
    < EditionGuide { target: "x86_64-unknown-linux-gnu" }
    c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
    > RustInstaller { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
      > ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "fabricate", path: "src/tools/rust-installer", mode: ToolBootstrap, is_optional_tool: false, source_type: Submodule, extra_features: [] }
        c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
      < ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "fabricate", path: "src/tools/rust-installer", mode: ToolBootstrap, is_optional_tool: false, source_type: Submodule, extra_features: [] }
    < RustInstaller { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
  < Docs { stage: 2, host: "x86_64-unknown-linux-gnu" }
< Docs { stage: 2, target: "x86_64-unknown-linux-gnu", host: "x86_64-unknown-linux-gnu" }
> Std { stage: 2, target: "x86_64-unknown-linux-gnu", host: "x86_64-unknown-linux-gnu" }
  c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  > Std { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
    c RustInstaller { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
  < Std { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
< Std { stage: 2, target: "x86_64-unknown-linux-gnu", host: "x86_64-unknown-linux-gnu" }
> Rustc { stage: 2, target: "x86_64-unknown-linux-gnu", host: "x86_64-unknown-linux-gnu" }
  c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
  > Rustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
    c Rustdoc { host: "x86_64-unknown-linux-gnu" }
    c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    > DebuggerScripts { sysroot: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/tmp/dist/rustc-1.33.0-dev-x86_64-unknown-linux-gnu-image", host: "x86_64-unknown-linux-gnu" }
    < DebuggerScripts { sysroot: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/tmp/dist/rustc-1.33.0-dev-x86_64-unknown-linux-gnu-image", host: "x86_64-unknown-linux-gnu" }
    c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
    c RustInstaller { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
    c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
  < Rustc { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
< Rustc { stage: 2, target: "x86_64-unknown-linux-gnu", host: "x86_64-unknown-linux-gnu" }
> Docs { stage: 2, target: "x86_64-unknown-linux-gnu", host: "x86_64-unknown-linux-gnu" }
  > Docs { stage: 2, host: "x86_64-unknown-linux-gnu" }
Dist docs (x86_64-unknown-linux-gnu)
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
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "-v" "--release" "--features" "panic-unwind backtrace" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/Cargo.toml" "--message-format" "json"
       Fresh cc v1.0.28
       Fresh core v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libcore)
       Fresh build_helper v0.1.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/build_helper)
       Fresh cmake v0.1.33
       Fresh rustc-std-workspace-core v1.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/tools/rustc-std-workspace-core)
       Fresh libc v0.2.46
       Fresh compiler_builtins v0.1.4
       Fresh unwind v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libunwind)
       Fresh alloc v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/liballoc)
       Fresh rustc-demangle v0.1.10
       Fresh panic_abort v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libpanic_abort)
       Fresh backtrace-sys v0.1.27
       Fresh panic_unwind v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libpanic_unwind)
       Fresh rustc_msan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_msan)
       Fresh rustc_asan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_asan)
       Fresh rustc_tsan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_tsan)
       Fresh rustc_lsan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_lsan)
       Fresh std v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd)
    Finished release [optimized] target(s) in 0.93s
not updating "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/.libstd.stamp"; contents equal and SystemTime { tv_sec: 1547733897, tv_nsec: 647240705 } <= SystemTime { tv_sec: 1547733897, tv_nsec: 801238769 }
                  c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
                  > StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
                    c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
                    c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
                    c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
                  < StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 0.992
                < Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
                c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "-v" "--release" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libtest/Cargo.toml" "--message-format" "json"
       Fresh getopts v0.2.17
       Fresh term v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libterm)
       Fresh proc_macro v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libproc_macro)
       Fresh test v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libtest)
    Finished release [optimized] target(s) in 0.75s
not updating "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/.libtest.stamp"; contents equal and SystemTime { tv_sec: 1547733920, tv_nsec: 136957971 } <= SystemTime { tv_sec: 1547733920, tv_nsec: 188957317 }
                c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
                > TestLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
                  c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
                  c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
                  c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
                < TestLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[TIMING] Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 0.781
              < Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
              c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
              c Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
              c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "-v" "--release" "--features" "" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/rustc/Cargo.toml" "--message-format" "json"
       Fresh cfg-if v0.1.6
       Fresh nodrop v0.1.12
       Fresh void v1.0.2
       Fresh scopeguard v0.3.3
       Fresh memoffset v0.2.1
       Fresh rand_core v0.3.0
       Fresh lazy_static v1.2.0
       Fresh stable_deref_trait v1.1.0
       Fresh semver-parser v0.7.0
       Fresh bitflags v1.0.4
       Fresh byteorder v1.2.7
       Fresh unicode-width v0.1.5
       Fresh either v1.5.0
       Fresh graphviz v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libgraphviz)
       Fresh scoped-tls v0.1.2
       Fresh lazy_static v0.2.11
       Fresh termcolor v1.0.4
       Fresh datafrog v2.0.1
       Fresh rustc-demangle v0.1.10
       Fresh remove_dir_all v0.5.1
       Fresh fmt_macros v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libfmt_macros)
       Fresh rustc_fs_util v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_fs_util)
       Fresh rustc-serialize v0.3.24
       Fresh quick-error v1.2.2
       Fresh cc v1.0.28
       Fresh crossbeam-utils v0.2.2
       Fresh log v0.4.6
       Fresh arrayvec v0.4.7
       Fresh unreachable v1.0.0
       Fresh rand_core v0.2.2
       Fresh rand_isaac v0.1.1
       Fresh rand_hc v0.1.0
       Fresh rand_xorshift v0.1.0
       Fresh log_settings v0.1.2
       Fresh owning_ref v0.3.3
       Fresh semver v0.9.0
       Fresh rustc-hash v1.0.1
       Fresh chalk-macros v0.1.0
       Fresh rls-span v0.4.0
       Fresh humantime v1.2.0
       Fresh ena v0.11.0
       Fresh rustc_cratesio_shim v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_cratesio_shim)
       Fresh crossbeam-epoch v0.3.1
       Fresh libc v0.2.46
       Fresh smallvec v0.6.7
       Fresh lock_api v0.1.3
       Fresh rustc_version v0.2.3
       Fresh polonius-engine v0.6.2
       Fresh crc32fast v1.1.2
       Fresh chalk-engine v0.9.0
       Fresh rls-data v0.18.1
       Fresh crossbeam-deque v0.2.0
       Fresh rand v0.5.5
       Fresh num_cpus v1.8.0
       Fresh rand v0.4.3
       Fresh atty v0.2.11
       Fresh jobserver v0.1.12
       Fresh memmap v0.6.2
       Fresh serialize v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libserialize)
       Fresh rustc_apfloat v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_apfloat)
       Fresh miniz-sys v0.1.11
       Fresh backtrace-sys v0.1.27
       Fresh parking_lot_core v0.3.0
       Fresh rustc-rayon-core v0.1.1
       Fresh env_logger v0.5.13
       Fresh flate2 v1.0.6
       Fresh backtrace v0.3.11
       Fresh parking_lot v0.6.4
       Fresh rustc-rayon v0.1.1
       Fresh rand_pcg v0.1.1
       Fresh rand_chacha v0.1.0
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
       Fresh rustc_incremental v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_incremental)
       Fresh rustc_mir v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_mir)
       Fresh rustc_typeck v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_typeck)
       Fresh rustc_traits v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_traits)
       Fresh rustc_allocator v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_allocator)
       Fresh rustc_lint v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_lint)
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
       Fresh rustc-main v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/rustc)
    Finished release [optimized] target(s) in 0.99s
not updating "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/.librustc.stamp"; contents equal and SystemTime { tv_sec: 1547736456, tv_nsec: 183075554 } <= SystemTime { tv_sec: 1547736457, tv_nsec: 426059927 }
              c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
              > RustcLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
                c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
                c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
                c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
              < RustcLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[TIMING] Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 1.054
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
       Fresh memmap v0.6.2
       Fresh num_cpus v1.8.0
       Fresh rustc_llvm v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_llvm)
       Fresh rustc_codegen_llvm v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_llvm)
    Finished release [optimized] target(s) in 0.74s
not updating "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/.tmp.stamp"; contents equal and SystemTime { tv_sec: 1547736663, tv_nsec: 535468781 } <= SystemTime { tv_sec: 1547736663, tv_nsec: 665467147 }
[TIMING] CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" } -- 0.790
            < CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" }
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
            > Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
            < Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
            c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
            > Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
              c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
            < Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
            c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[TIMING] Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } } -- 0.436
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
       Fresh unwind v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libunwind)
       Fresh rustc-demangle v0.1.10
       Fresh panic_abort v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libpanic_abort)
       Fresh backtrace-sys v0.1.27
       Fresh rustc_tsan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_tsan)
       Fresh rustc_msan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_msan)
       Fresh rustc_asan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_asan)
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
[TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } } -- 0.771
              < Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
              c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
              c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
              c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
              c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
Building stage1 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "-v" "--release" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libtest/Cargo.toml" "--message-format" "json"
       Fresh getopts v0.2.17
       Fresh proc_macro v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libproc_macro)
       Fresh term v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libterm)
       Fresh test v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libtest)
    Finished release [optimized] target(s) in 0.77s
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
[TIMING] Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } } -- 0.801
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
       Fresh lazy_static v1.2.0
       Fresh rand_core v0.3.0
       Fresh void v1.0.2
       Fresh scopeguard v0.3.3
       Fresh memoffset v0.2.1
       Fresh stable_deref_trait v1.1.0
       Fresh semver-parser v0.7.0
       Fresh bitflags v1.0.4
       Fresh byteorder v1.2.7
       Fresh unicode-width v0.1.5
       Fresh either v1.5.0
       Fresh graphviz v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libgraphviz)
       Fresh scoped-tls v0.1.2
       Fresh lazy_static v0.2.11
       Fresh termcolor v1.0.4
       Fresh remove_dir_all v0.5.1
       Fresh rustc-demangle v0.1.10
       Fresh datafrog v2.0.1
       Fresh rustc_fs_util v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_fs_util)
       Fresh fmt_macros v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libfmt_macros)
       Fresh rustc-serialize v0.3.24
       Fresh quick-error v1.2.2
       Fresh cc v1.0.28
       Fresh crossbeam-utils v0.2.2
       Fresh log v0.4.6
       Fresh arrayvec v0.4.7
       Fresh log_settings v0.1.2
       Fresh rand_core v0.2.2
       Fresh rand_xorshift v0.1.0
       Fresh rand_isaac v0.1.1
       Fresh rand_hc v0.1.0
       Fresh unreachable v1.0.0
       Fresh owning_ref v0.3.3
       Fresh semver v0.9.0
       Fresh rustc-hash v1.0.1
       Fresh chalk-macros v0.1.0
       Fresh rls-span v0.4.0
       Fresh humantime v1.2.0
       Fresh libc v0.2.46
       Fresh ena v0.11.0
       Fresh rustc_cratesio_shim v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_cratesio_shim)
       Fresh crossbeam-epoch v0.3.1
       Fresh smallvec v0.6.7
       Fresh lock_api v0.1.3
       Fresh rustc_version v0.2.3
       Fresh polonius-engine v0.6.2
       Fresh crc32fast v1.1.2
       Fresh chalk-engine v0.9.0
       Fresh rls-data v0.18.1
       Fresh rand v0.4.3
       Fresh rand v0.5.5
       Fresh num_cpus v1.8.0
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
       Fresh rand_pcg v0.1.1
       Fresh rand_chacha v0.1.0
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
    Finished release [optimized] target(s) in 1.24s
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
[TIMING] Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } } -- 1.490
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
       Fresh rustc-demangle v0.1.10
       Fresh cc v1.0.28
       Fresh libc v0.2.46
       Fresh num_cpus v1.8.0
       Fresh memmap v0.6.2
       Fresh rustc_llvm v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_llvm)
       Fresh rustc_codegen_llvm v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_llvm)
    Finished release [optimized] target(s) in 0.71s
not updating "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-codegen/x86_64-unknown-linux-gnu/release/.tmp.stamp"; contents equal and SystemTime { tv_sec: 1547740243, tv_nsec: 59468070 } <= SystemTime { tv_sec: 1547740243, tv_nsec: 288465191 }
[TIMING] CodegenBackend { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" } -- 0.755
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
Generating unstable book md files (x86_64-unknown-linux-gnu)
        c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
        > UnstableBookGen { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
          > ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "unstable-book-gen", path: "src/tools/unstable-book-gen", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] }
            c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
Building stage0 tool unstable-book-gen (x86_64-unknown-linux-gnu)
running: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "-v" "--release" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/tools/unstable-book-gen/Cargo.toml" "--message-format" "json"
       Fresh unicode-xid v0.1.0
       Fresh itoa v0.4.3
       Fresh proc-macro2 v0.4.24
       Fresh ryu v0.2.7
       Fresh serde v1.0.82
       Fresh num-traits v0.2.6
       Fresh quote v0.6.10
       Fresh serde_json v1.0.33
       Fresh syn v0.15.22
       Fresh serde_derive v1.0.81
   Compiling tidy v0.1.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/tools/tidy)
     Running `/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/rustc --crate-name tidy src/tools/tidy/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=b87f8481d2fdc0d6 -C extra-filename=-b87f8481d2fdc0d6 --out-dir /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C incremental=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/incremental -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/deps --extern serde=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libserde-2c43ab9f867c1718.rlib --extern serde_derive=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/deps/libserde_derive-a085201d971895af.so --extern serde_json=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libserde_json-44dd882ecb9a69bb.rlib`
rustc command: "LD_LIBRARY_PATH"="/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/deps:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/rustc" "--crate-name" "tidy" "src/tools/tidy/src/lib.rs" "--color" "always" "--crate-type" "lib" "--emit=dep-info,link" "-C" "opt-level=2" "-C" "metadata=b87f8481d2fdc0d6" "-C" "extra-filename=-b87f8481d2fdc0d6" "--out-dir" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps" "--target" "x86_64-unknown-linux-gnu" "-C" "incremental=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/incremental" "-L" "dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/deps" "--extern" "serde=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libserde-2c43ab9f867c1718.rlib" "--extern" "serde_derive=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/deps/libserde_derive-a085201d971895af.so" "--extern" "serde_json=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libserde_json-44dd882ecb9a69bb.rlib" "--cfg" "stage0" "--sysroot" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0" "-C" "debug-assertions=y" "-C" "codegen-units=4" "-C" "link-args=-Wl,-rpath,$ORIGIN/../lib" "-Dwarnings" "-Dbare_trait_objects"
sysroot: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0"
libdir: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/lib"
thread 'main' panicked at 'failed to open bitcode file `/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/incremental/tidy-2rxgu4moab819/s-f8mm8z7roj-13g375o-working/2cz7axt33h95nr6y.pre-thin-lto.bc`: No such file or directory (os error 2)', src/librustc_codegen_ssa/back/write.rs:1821:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.32.0-beta.11 (e64fee6a3 2019-01-04) running on x86_64-unknown-linux-gnu

note: compiler flags: -C opt-level=2 -C incremental -C debug-assertions=y -C codegen-units=4 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib

note: some of the compiler flags provided by cargo are hidden

[RUSTC-TIMING] tidy test:false 2.033
error: Could not compile `tidy`.

Caused by:
  process didn't exit successfully: `/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/rustc --crate-name tidy src/tools/tidy/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=b87f8481d2fdc0d6 -C extra-filename=-b87f8481d2fdc0d6 --out-dir /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C incremental=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/incremental -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/deps --extern serde=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libserde-2c43ab9f867c1718.rlib --extern serde_derive=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/deps/libserde_derive-a085201d971895af.so --extern serde_json=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libserde_json-44dd882ecb9a69bb.rlib` (exit code: 101)
command did not execute successfully: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "-v" "--release" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/tools/unstable-book-gen/Cargo.toml" "--message-format" "json"
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
RuntimeError: failed to run: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/bootstrap install -vv --incremental -j 4

real	0m24.110s
user	0m18.734s
sys	0m2.083s

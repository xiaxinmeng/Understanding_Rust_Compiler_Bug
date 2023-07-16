
$ python x.py test src/test/run-pass --host=x86_64-unknown-linux-gnu --stage=1 --target=x86_64-unknown-linux-gnu --build=x86_64-unknown-linux-gnu --on-fail=bash
    Finished debug [unoptimized] target(s) in 0.0 secs
Synchronizing submodule url for 'src/compiler-rt'
Synchronizing submodule url for 'src/jemalloc'
Synchronizing submodule url for 'src/liblibc'
Synchronizing submodule url for 'src/llvm'
Synchronizing submodule url for 'src/rt/hoedown'
Synchronizing submodule url for 'src/rust-installer'
HEAD is now at d30da544 Merge pull request #30 from japaric/msan
HEAD is now at 11bfb0d Merge pull request #16 from glandium/rust
HEAD is now at 8d8264b96 Auto merge of #512 - dumbbell:support-aarch64-unknown-freebsd, r=alexcrichton
HEAD is now at ceb177eeefa Merge pull request #60 from japaric/gh38406
HEAD is now at da282f1 Merge pull request #8 from GuillaumeGomez/line_information
HEAD is now at 4f99485 Merge pull request #54 from brson/docdir
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.0 secs
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.0 secs
Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling rustc v0.0.0 (file:///home/nagisa/Documents/rust/rust/worktrees/more-cace/src/librustc)
error[E0463]: can't find crate for `banana`
  --> src/librustc/lib.rs:45:1
   |
45 | extern crate banana;
   | ^^^^^^^^^^^^^^^^^^^^ can't find crate

error: aborting due to previous error


Did not run successfully:
"/home/nagisa/Documents/rust/rust/worktrees/more-cace/build/x86_64-unknown-linux-gnu/stage0/bin/rustc" "--crate-name" "rustc" "src/librustc/lib.rs" "--crate-type" "dylib" "-C" "prefer-dynamic" "-C" "opt-level=2" "-C" "metadata=da4efe83a9e913d5" "-C" "extra-filename=-da4efe83a9e913d5" "--out-dir" "/home/nagisa/Documents/rust/rust/worktrees/more-cace/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "--emit=dep-info,link" "--target" "x86_64-unknown-linux-gnu" "-L" "dependency=/home/nagisa/Documents/rust/rust/worktrees/more-cace/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/home/nagisa/Documents/rust/rust/worktrees/more-cace/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps" "--extern" "syntax=/home/nagisa/Documents/rust/rust/worktrees/more-cace/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-27acfd4de2f55616.so" "--extern" "rustc_bitflags=/home/nagisa/Documents/rust/rust/worktrees/more-cace/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_bitflags-fc9d26366eab2200.rlib" "--extern" "graphviz=/home/nagisa/Documents/rust/rust/worktrees/more-cace/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-9e6326f44e01f30d.so" "--extern" "syntax_pos=/home/nagisa/Documents/rust/rust/worktrees/more-cace/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-19a139c1f82f88f2.so" "--extern" "rustc_errors=/home/nagisa/Documents/rust/rust/worktrees/more-cace/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-17caec4ec4736b67.so" "--extern" "arena=/home/nagisa/Documents/rust/rust/worktrees/more-cace/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-95940039e572b298.so" "--extern" "rustc_data_structures=/home/nagisa/Documents/rust/rust/worktrees/more-cace/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-5eeeb6e44141dc85.so" "--extern" "serialize=/home/nagisa/Documents/rust/rust/worktrees/more-cace/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-e8f26cb531d9dfdd.so" "--extern" "serialize=/home/nagisa/Documents/rust/rust/worktrees/more-cace/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-e8f26cb531d9dfdd.rlib" "--extern" "rustc_const_math=/home/nagisa/Documents/rust/rust/worktrees/more-cace/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-34fe24d17ac3499e.so" "--extern" "fmt_macros=/home/nagisa/Documents/rust/rust/worktrees/more-cace/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-62f1962d86cc281d.so" "--extern" "log=/home/nagisa/Documents/rust/rust/worktrees/more-cace/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-45ebcad865d72458.so" "--extern" "log=/home/nagisa/Documents/rust/rust/worktrees/more-cace/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-45ebcad865d72458.rlib" "--extern" "rustc_back=/home/nagisa/Documents/rust/rust/worktrees/more-cace/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_back-389ae9e68756566e.so" "--extern" "rustc_llvm=/home/nagisa/Documents/rust/rust/worktrees/more-cace/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_llvm-dd9956a823ee835e.so" "-L" "native=/home/nagisa/Documents/rust/rust/worktrees/more-cace/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-2751b8d39782e2a1/out" "-L" "native=/home/nagisa/Documents/rust/rust/worktrees/more-cace/build/x86_64-unknown-linux-gnu/llvm/lib" "--cfg" "stage0" "--sysroot" "/home/nagisa/Documents/rust/rust/worktrees/more-cace/build/x86_64-unknown-linux-gnu/stage0-sysroot" "-Cprefer-dynamic" "-C" "debug-assertions=y" "-C" "codegen-units=2" "-C" "link-args=-Wl,-rpath,$ORIGIN/../lib"
-------------
[nagisa@kumabox more-cace]$ # environment rustc ran in (a different shell)

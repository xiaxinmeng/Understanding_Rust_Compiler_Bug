
==> Downloading https://github.com/rust-lang/cargo/commit/27277d966b3cfa454d6dea7f724cb961c036251c.patch?full_index=1
Already downloaded: /Users/francis/Library/Caches/Homebrew/downloads/37d7bab58aa07e080dd782c515558b0a4dbcb4a6a4b271560cbc412e3b4b2277--27277d966b3cfa454d6dea7f724cb961c036251c.patch
==> Cloning https://github.com/rust-lang/cargo.git
Updating /Users/francis/Library/Caches/Homebrew/rust--cargo--git
==> Checking out tag 0.55.0
HEAD is now at 5ae8d74b3 Auto merge of #9611 - ehuss:beta-1.54-backports, r=alexcrichton
HEAD is now at 5ae8d74b3 Auto merge of #9611 - ehuss:beta-1.54-backports, r=alexcrichton
==> Downloading https://static.rust-lang.org/dist/2021-06-17/cargo-1.53.0-x86_64-apple-darwin.tar.gz
Already downloaded: /Users/francis/Library/Caches/Homebrew/downloads/f3e3e9f46257e63b020ce057fe7d8be7b1ab899da38a8d2603e9a933453b25c3--cargo-1.53.0-x86_64-apple-darwin.tar.gz
==> Downloading https://static.rust-lang.org/dist/rustc-1.54.0-src.tar.gz
Already downloaded: /Users/francis/Library/Caches/Homebrew/downloads/1c256d5fdc4ccab76e1d8d332ee8d5d7f6f80fe554fbe061a8f3e7e0d56ee4ad--rustc-1.54.0-src.tar.gz
==> ./configure --prefix=/usr/local/Cellar/rust/1.54.0_1 --release-channel=stable
==> make
Last 15 lines from /Users/francis/Library/Logs/Homebrew/rust/02.make:
[908/2756] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/CmpInstAnalysis.cpp.o
[909/2756] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/CallGraphSCCPass.cpp.o
[910/2756] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/CallPrinter.cpp.o
[911/2756] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/CaptureTracking.cpp.o
[912/2756] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/CGSCCPassManager.cpp.o
ninja: build stopped: subcommand failed.
thread 'main' panicked at '
command did not execute successfully, got: exit status: 1

build script failed, must exit now', /Users/francis/Library/Caches/Homebrew/cargo_cache/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:885:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
	finished in 513.681 seconds
failed to run: /private/tmp/rust-20210905-28496-10niw6/rustc-1.54.0-src/build/bootstrap/debug/bootstrap build --stage 2
Build completed unsuccessfully in 0:11:14
make: *** [all] Error 1


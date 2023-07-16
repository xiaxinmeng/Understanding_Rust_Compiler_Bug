
==> Installing ffmpeg dependency: rust
==> ./configure --prefix=/usr/local/Cellar/rust/1.55.0 --release-channel=stable
==> make
Last 15 lines from /Users/adolphor/Library/Logs/Homebrew/rust/02.make:
using ::isunordered;
      ~~^
13 errors generated.
[4/2756] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Signposts.cpp.o
[5/2756] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/MicrosoftDemangle.cpp.o
[6/2756] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/ItaniumDemangle.cpp.o
ninja: build stopped: subcommand failed.
thread 'main' panicked at '
command did not execute successfully, got: exit status: 1

build script failed, must exit now', /Users/adolphor/Library/Caches/Homebrew/cargo_cache/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:885:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
	finished in 42.952 seconds
Build completed unsuccessfully in 0:09:13
make: *** [all] Error 1

Do not report this issue to Homebrew/brew or Homebrew/core!

These open issues may also help:
gimme-aws-creds: make rust build dep https://github.com/Homebrew/homebrew-core/pull/85081
Rust-dependent formulae on Apple Silicon - upstream issue tracker https://github.com/Homebrew/homebrew-core/issues/68301

Error: You are using macOS 10.13.
We (and Apple) do not provide support for this old version.
You will encounter build failures with some formulae.
Please create pull requests instead of asking for help on Homebrew's GitHub,
Twitter or any other official channels. You are responsible for resolving
any issues you experience while you are running this
old version.

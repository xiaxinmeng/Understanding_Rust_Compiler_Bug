console
> ./x.py setup --verbose
downloading https://static.rust-lang.org/dist/2023-04-20/rust-std-beta-x86_64-unknown-linux-musl.tar.xz
############################################################################################################################## 100.0%
downloading https://static.rust-lang.org/dist/2023-04-20/rustc-beta-x86_64-unknown-linux-musl.tar.xz
############################################################################################################################## 100.0%
downloading https://static.rust-lang.org/dist/2023-04-20/cargo-beta-x86_64-unknown-linux-musl.tar.xz
############################################################################################################################## 100.0%
extracting /home/hugo/src/github.com/rust-lang/rust/build/cache/2023-04-20/rust-std-beta-x86_64-unknown-linux-musl.tar.xz
extracting /home/hugo/src/github.com/rust-lang/rust/build/cache/2023-04-20/rustc-beta-x86_64-unknown-linux-musl.tar.xz
extracting /home/hugo/src/github.com/rust-lang/rust/build/cache/2023-04-20/cargo-beta-x86_64-unknown-linux-musl.tar.xz
Building bootstrap
   Compiling libc v0.2.140
   Compiling proc-macro2 v1.0.56
   Compiling memchr v2.5.0
   Compiling unicode-ident v1.0.0
   Compiling quote v1.0.26
   Compiling cfg-if v1.0.0
   Compiling version_check v0.9.4
   Compiling typenum v1.15.0
   Compiling bitflags v1.3.2
   Compiling io-lifetimes v1.0.9
   Compiling cc v1.0.73
   Compiling regex-automata v0.1.10
   Compiling serde v1.0.137
   Compiling pkg-config v0.3.25
   Compiling log v0.4.17
   Compiling lazy_static v1.4.0
   Compiling rustix v0.37.6
   Compiling once_cell v1.12.0
   Compiling clap_lex v0.4.1
   Compiling syn v1.0.102
   Compiling anstyle v1.0.0
   Compiling linux-raw-sys v0.3.2
   Compiling heck v0.4.1
   Compiling crossbeam-utils v0.8.14
   Compiling regex-syntax v0.6.26
   Compiling serde_json v1.0.81
   Compiling same-file v1.0.6
   Compiling serde_derive v1.0.137
   Compiling fnv v1.0.7
   Compiling clap_builder v4.2.4
   Compiling semver v1.0.17
   Compiling thread_local v1.1.4
   Compiling walkdir v2.3.2
   Compiling ryu v1.0.10
   Compiling cpufeatures v0.2.5
   Compiling bootstrap v0.0.0 (/home/hugo/src/github.com/rust-lang/rust/src/bootstrap)
   Compiling itoa v1.0.2
   Compiling build_helper v0.1.0 (/home/hugo/src/github.com/rust-lang/rust/src/tools/build_helper)
   Compiling termcolor v1.2.0
   Compiling generic-array v0.14.5
   Compiling hex v0.4.3
   Compiling cmake v0.1.48
   Compiling aho-corasick v0.7.18
   Compiling bstr v0.2.17
   Compiling object v0.29.0
   Compiling lzma-sys v0.1.17
   Compiling syn v2.0.15
   Compiling opener v0.5.0
   Compiling block-buffer v0.10.2
   Compiling crypto-common v0.1.3
   Compiling digest v0.10.3
   Compiling regex v1.5.6
   Compiling sha2 v0.10.2
   Compiling filetime v0.2.16
   Compiling xattr v0.2.3
   Compiling xz2 v0.1.6
   Compiling tar v0.4.38
   Compiling globset v0.4.8
   Compiling ignore v0.4.18
   Compiling toml v0.5.9
   Compiling is-terminal v0.4.6
   Compiling fd-lock v3.0.11
   Compiling clap_derive v4.2.0
   Compiling clap v4.2.4
   Compiling clap_complete v4.2.2
    Finished dev [unoptimized] target(s) in 0.03s
running: /home/hugo/src/github.com/rust-lang/rust/build/bootstrap/debug/bootstrap setup --verbose
finding compilers
CC_x86_64-unknown-linux-musl = "cc"
CFLAGS_x86_64-unknown-linux-musl = ["-ffunction-sections", "-fdata-sections", "-fPIC", "-m64", "-static"]
CXX_x86_64-unknown-linux-musl = "c++"
CXXFLAGS_x86_64-unknown-linux-musl = ["-ffunction-sections", "-fdata-sections", "-fPIC", "-m64", "-static"]
AR_x86_64-unknown-linux-musl = "ar"
verifying /home/hugo/src/github.com/rust-lang/rust/build/cache/2023-04-21/rustfmt-nightly-x86_64-unknown-linux-musl.tar.xz
extracting /home/hugo/src/github.com/rust-lang/rust/build/cache/2023-04-21/rustfmt-nightly-x86_64-unknown-linux-musl.tar.xz to /home/hugo/src/github.com/rust-lang/rust/build/x86_64-unknown-linux-musl/rustfmt
Traceback (most recent call last):
  File "/home/hugo/src/github.com/rust-lang/rust/./x.py", line 32, in <module>
    bootstrap.main()
  File "/home/hugo/src/github.com/rust-lang/rust/src/bootstrap/bootstrap.py", line 1084, in main
    bootstrap(args)
  File "/home/hugo/src/github.com/rust-lang/rust/src/bootstrap/bootstrap.py", line 1059, in bootstrap
    run(args, env=env, verbose=build.verbose, is_bootstrap=True)
  File "/home/hugo/src/github.com/rust-lang/rust/src/bootstrap/bootstrap.py", line 184, in run
    raise RuntimeError(err)
RuntimeError: failed to run: /home/hugo/src/github.com/rust-lang/rust/build/bootstrap/debug/bootstrap setup --verbose

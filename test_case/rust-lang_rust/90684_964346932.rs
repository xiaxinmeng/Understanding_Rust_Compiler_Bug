
rg 'run\.path' src/bootstrap/dist.rs  | sed 's/.*"\(.*\)".*/\1/' | sort | uniq -c
      1 build-manifest
      1 cargo
      1 clippy
      1 extended
      1 llvm-tools
      1 miri
      1 reproducible-artifacts
      1 rls
      1 rust-analysis
      1 rust-analyzer
      1 rustc
      1 rustc-dev
      1 rustc-docs
      1 rustc-src
      1 rust-demangler
      1 rust-dev
      1 rust-docs
      1 rustfmt
      1 rust-mingw
      1 rust-src
      1 rust-std
      1 src/lldb_batchmode.py

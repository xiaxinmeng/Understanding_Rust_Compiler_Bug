
error: process didn't exit successfully: `rustc -vV` (exit code: 1)
--- stdout
rustc 1.33.0-nightly (8e2063d02 2019-01-07)
binary: rustc
commit-hash: 8e2063d02062ee9f088274690a97826333847e17
commit-date: 2019-01-07
host: x86_64-unknown-linux-gnu
release: 1.33.0-nightly
--- stderr
error: couldn't load codegen backend "/root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so": "/usr/lib64/libstdc++.so.6: version `GLIBCXX_3.4.9\' not found (required by /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-8svn.so)"

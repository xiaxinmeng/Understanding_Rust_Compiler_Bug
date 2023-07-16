`bash
git clone https://github.com/rust-lang/regex/
cd regex
git checkout fc3e6aa 
RUSTFLAGS="-Zincremental-verify-ich=yes" cargo build
git checkout 0e96af4 
RUSTFLAGS="-Zincremental-verify-ich=yes" cargo build

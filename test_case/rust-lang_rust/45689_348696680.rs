`bash
cargo new --bin crashtest
cd crashtest
echo "
[profile.release]
lto = true
" >> Cargo.toml

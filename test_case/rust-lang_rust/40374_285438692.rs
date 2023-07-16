bash
cd /tmp
git clone https://github.com/kfairmasterz/wtf
cd wtf
cargo build --release
strip ./target/release/wtf
objdump -s -j .rodata ./target/release/wtf

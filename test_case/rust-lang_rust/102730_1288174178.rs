sh
# git clone git@github.com:cybergsus/sawblade.git
# git checkout 8be5ef464ac842b55c2da87f4f0c666f373cdd11
# cd sawblade
RUSTFLAGS="-Zsanitizer=address" cargo +nightly fuzz build -Zbuild-std

sh
#!/bin/sh
set -e

unset CARGO_TARGET_DIR CARGO_REGISTRIES_CRATES_IO_PROTOCOL

git checkout 6874f4e3fc2a16be7c78e702d068bbc1daa90e16
git cherry-pick metadata-ice
rm -rf build/host/stage*
x build # --stage 1 proc-macro-srv-cli
git checkout 999ac5
git cherry-pick metadata-ice
x build # --stage 1 proc-macro-srv-cli

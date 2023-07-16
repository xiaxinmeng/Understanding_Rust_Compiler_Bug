
TRIPLET = x86_64-unknown-linux-gnu

all: libcore.diff

libcore.diff: rust.metadata.bin.1 rust.metadata.bin.2
    diffoscope --html-dir "$@" rust.metadata.bin.1 rust.metadata.bin.2 \
        --max-diff-block-lines 5000 \
        --max-diff-input-lines 10000000 \
        --max-report-size 204800000; true # diffoscope returns 1 for "there were diffs"

rust.metadata.bin.1 rust.metadata.bin.2: Makefile
    rm -f $(TRIPLET)/stage2/lib/rustlib/$(TRIPLET)/lib/libcore-*.rlib
    rm -f $(TRIPLET)/stage2/lib/rustlib/$(TRIPLET)/lib/stamp.core
    $(MAKE) $(TRIPLET)/stage2/lib/rustlib/$(TRIPLET)/lib/stamp.core
    ar x $(TRIPLET)/stage2/lib/rustlib/$(TRIPLET)/lib/libcore-*.rlib rust.metadata.bin
    mv rust.metadata.bin "$@"

.PHONY: all rust.metadata.bin.1 rust.metadata.bin.2

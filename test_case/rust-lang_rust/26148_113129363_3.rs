
RUSTC_CROSS_FLAGS_le32-unknown-nacl=-C cross-path=$(CFG_NACL_CROSS_PATH) --cfg "target_libc=\"newlib\"" -L $(CFG_NACL_CROSS_PATH)/lib/pnacl/Release -L $(CFG_NACL_CROSS_PATH)/toolchain/$(NACL_TOOLCHAIN_OS_PATH)_pnacl/lib/clang/3.6.0/lib/le32-nacl -L $(CFG_NACL_CROSS_PATH)/toolchain/$(NACL_TOOLCHAIN_OS_PATH)_pnacl/le32-nacl/usr/lib

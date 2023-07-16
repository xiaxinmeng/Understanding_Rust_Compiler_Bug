
MUSL_CONFIGURE_ARGS = \
  --set=target.$(RUSTC_TARGET_ARCH).ar=$(TARGET_AR) \
  --set=target.$(RUSTC_TARGET_ARCH).cc=$(TARGET_CC_NOCACHE) \
  --set=target.$(RUSTC_TARGET_ARCH).cxx=$(TARGET_CXX_NOCACHE) \
  --set=target.$(RUSTC_TARGET_ARCH).linker=$(TARGET_CC_NOCACHE) \
  --set=target.$(RUSTC_TARGET_ARCH).musl-root=$(TOOLCHAIN_DIR) \
  --set=target.$(RUSTC_TARGET_ARCH).ranlib=$(TARGET_RANLIB)

HOST_CONFIGURE_OPTS += CARGO_HOME=$(CARGO_HOME)
RUST_COMMOM_ARGS := \
  --build=$(RUSTC_HOST_ARCH) \
  --dist-compression-formats=xz \
  --enable-full-tools \
  --enable-llvm-link-shared \
  --enable-llvm-plugins \
  --enable-missing-tools \
  --enable-ninja \
  --enable-optimize \
  --enable-optimize-llvm \
  --enable-parallel-compiler \
  --enable-sanitizers \
  --release-channel=nightly \
  ${MUSL_CONFIGURE_ARGS}

HOST_CONFIGURE_ARGS = \
  --target=$(RUSTC_TARGET_ARCH) \
  --host=$(RUSTC_HOST_ARCH) \
  --prefix=$(STAGING_DIR_HOST) \
  --bindir=$(STAGING_DIR_HOST)/bin \
  --libdir=$(STAGING_DIR_HOST)/lib \
  --sysconfdir=$(STAGING_DIR_HOST)/etc \
  --datadir=$(STAGING_DIR_HOST)/share \
  --mandir=$(STAGING_DIR_HOST)/man \
  ${RUST_COMMOM_ARGS}

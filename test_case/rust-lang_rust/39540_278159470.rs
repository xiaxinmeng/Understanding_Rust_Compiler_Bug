cmake
# old setup, uses an external bootstrap package
define HOST_RUST_BUILDROOT_CONFIGURE_CMDS
	(cd $(@D); $(HOST_CONFIGURE_OPTS) \
		$(HOST_RUST_BUILDROOT_CONF_ENV) \
		./configure \
		--target=$(GNU_TARGET_NAME) \
		--prefix="$(HOST_DIR)/usr" \
		--jemalloc-root="$(HOST_DIR)/usr/lib" \
		--enable-local-rust \
		--local-rust-root="$(HOST_RUST_BOOTSTRAP_DIR)/rustc" \
		--disable-docs \
		--disable-manage-submodules \
		--sysconfdir="$(HOST_DIR)/etc" \
		--localstatedir="$(HOST_DIR)/var/lib" \
		--datadir="$(HOST_DIR)/usr/share" \
		--infodir="$(HOST_DIR)/usr/share/info" \
		$(HOST_RUST_BUILDROOT_CONF_OPTS) \
	)
endef

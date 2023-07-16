cmake
define HOST_RUST_BUILDROOT_CONFIGURE_CMDS
	(cd $(@D); $(HOST_CONFIGURE_OPTS) \
		$(HOST_RUST_BUILDROOT_CONF_ENV) \
		./configure \
		--target=$(GNU_TARGET_NAME) \
		--prefix="$(HOST_DIR)/usr" \
		--disable-docs \
		--disable-manage-submodules \
		--sysconfdir="$(HOST_DIR)/etc" \
		--localstatedir="$(HOST_DIR)/var/lib" \
		--datadir="$(HOST_DIR)/usr/share" \
		--infodir="$(HOST_DIR)/usr/share/info" \
		$(HOST_RUST_BUILDROOT_CONF_OPTS) \
	)
        # The following is to get mips64 working
	(cd $(@D); \
		echo > config.toml; \
		echo "[rust]" >> config.toml; \
		echo "backtrace = false" >> config.toml; \
		echo "[target.$(GNU_TARGET_NAME)]" >> config.toml; \
		echo "cc = \"$(GNU_TARGET_NAME)-gcc\"" >> config.toml; \
		echo "cxx = \"$(GNU_TARGET_NAME)-g++\"" >> config.toml; \
	)
endef

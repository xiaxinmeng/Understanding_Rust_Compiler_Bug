cmake
define HOST_RUST_BUILDROOT_BUILD_CMDS
	(cd $(@D); \
	 $(HOST_MAKE_ENV) $(HOST_RUST_BUILDROOT_MAKE_ENV) $(HOST_DIR)/usr/bin/python \
		src/bootstrap/bootstrap.py build $(if $(VERBOSE),-v); \
	 $(HOST_MAKE_ENV) $(HOST_RUST_BUILDROOT_MAKE_ENV) $(HOST_DIR)/usr/bin/python \
		src/bootstrap/bootstrap.py build --keep-stage 1 --target $(GNU_TARGET_NAME) $(if $(VERBOSE),-v); \
	)
endef

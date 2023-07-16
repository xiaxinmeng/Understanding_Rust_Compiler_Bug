
    $$(Q)$$(MAKE) -C $$(S)src/libuv/ \
        CFLAGS="$$(CFG_GCCISH_CFLAGS) $$(LIBUV_FLAGS_$$(HOST_$(1))) $$(SNAP_DEFINES)" \
        LDFLAGS="$$(CFG_GCCISH_LINK_FLAGS) $$(LIBUV_FLAGS_$$(HOST_$(1)))" \
        CC="$$(CC_$(1))" \
        CXX="$$(CXX_$(1))" \
        AR="$$(AR_$(1))" \
        builddir_name="$$(CFG_BUILD_DIR)/rt/$(1)/stage$(2)/libuv" \
        V=$$(VERBOSE)

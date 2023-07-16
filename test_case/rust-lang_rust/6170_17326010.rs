
# XXX: Shouldn't need platform-specific conditions here
ifdef CFG_WINDOWSY_$(1)
$$(LIBUV_LIB_$(1)): $$(LIBUV_DEPS)
    $$(Q)$$(MAKE) -C $$(S)src/libuv/ \
        builddir_name="$$(CFG_BUILD_DIR)/rt/$(1)/libuv" \
        OS=mingw \
        V=$$(VERBOSE)
else ifeq ($(OSTYPE_$(1)), linux-androideabi)
$$(LIBUV_LIB_$(1)): $$(LIBUV_DEPS)
    $$(Q)$$(MAKE) -C $$(S)src/libuv/ \
        CFLAGS="$$(LIBUV_FLAGS_$$(HOST_$(1))) $$(SNAP_DEFINES)" \
        LDFLAGS="$$(LIBUV_FLAGS_$$(HOST_$(1)))" \
        CC="$$(CC_$(1))" \
        CXX="$$(CXX_$(1))" \
        AR="$$(AR_$(1))" \
        BUILDTYPE=Release \
        builddir_name="$$(CFG_BUILD_DIR)/rt/$(1)/libuv" \
        host=android OS=linux \
        V=$$(VERBOSE)
## MY HACKS START HERE >>>>>>>>>
else ifeq ($(OSTYPE_$(1)), apple-darwin)
$$(LIBUV_LIB_$(1)): $$(LIBUV_DEPS)
    echo @COMPILING LIBUV FOR ARM-APPLE-DARWIN NOT INTENDED FOR MAC , JUST IOS
    echo $$(CC_$(1))  $$(CXX_$(1)) $$(AR_$(1))
    CFG_IOS_CF=$$(CFG_IOS_DEV)/SDKs/iPhone6.1sdk/System/Library/Frameworks/CoreFoundation.framework
    #exit(1)
    $$(Q)$$(MAKE) -C $$(S)src/libuv/ \
        CFLAGS="$$(CFG_IOS_FLAGS) $$(CFG_IOS_CF)  $$(LIBUV_FLAGS_$$(HOST_$(1))) $$(SNAP_DEFINES)" \
        LDFLAGS="$$(CFG_GCCISH_LINK_FLAGS_arm-apple-darwin) $$(LIBUV_FLAGS_$$(HOST_$(1)))" \
        CC="$$(CC_$(1))" \
        CXX="$$(CXX_$(1))" \
        AR="$$(AR_$(1))" \
        BUILDTYPE=Release \
        builddir_name="$$(CFG_BUILD_DIR)/rt/$(1)/libuv" \
        host=darwin OS=darwin \
        V=$$(VERBOSE)
## <<<<<<< MY HACKS END HERE
else
$$(LIBUV_LIB_$(1)): $$(LIBUV_DEPS)
      ####>>>>>> I dont think this was getting the cross-compile tools passed before..
    $$(Q)$$(MAKE) -C $$(S)src/libuv/ \
        CFLAGS="$$(LIBUV_FLAGS_$$(HOST_$(1))) $$(SNAP_DEFINES)" \
        builddir_name="$$(CFG_BUILD_DIR)/rt/$(1)/libuv" \
        V=$$(VERBOSE)
endif

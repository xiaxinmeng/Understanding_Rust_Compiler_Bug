
mk/cfg/arm-unknown-linux-gnueabihf.mk:CFG_JEMALLOC_CFLAGS_arm-unknown-linux-gnueabihf := -D__arm__ $(CFLAGS) -march=armv6 -marm
mk/cfg/arm-unknown-linux-gnueabihf.mk:CFG_GCCISH_CFLAGS_arm-unknown-linux-gnueabihf := -Wall -g -fPIC -D__arm__ $(CFLAGS) -march=armv6 -marm
mk/cfg/arm-unknown-linux-gnueabihf.mk:RUSTC_FLAGS_arm-unknown-linux-gnueabihf := -C target-feature=+v6,+vfp2

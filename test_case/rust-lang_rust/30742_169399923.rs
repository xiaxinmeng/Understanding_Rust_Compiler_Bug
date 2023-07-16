 mk
# arm-unknown-linux-musl configuration
CROSS_PREFIX_arm-unknown-linux-musl=arm-linux-musl-
CC_arm-unknown-linux-musl=gcc
CXX_arm-unknown-linux-musl=g++
CPP_arm-unknown-linux-musl=gcc -E
AR_arm-unknown-linux-musl=ar
CFG_INSTALL_ONLY_RLIB_arm-unknown-linux-musl = 1
CFG_LIB_NAME_arm-unknown-linux-musl=lib$(1).so
CFG_STATIC_LIB_NAME_arm-unknown-linux-musl=lib$(1).a
CFG_LIB_GLOB_arm-unknown-linux-musl=lib$(1)-*.so
CFG_LIB_DSYM_GLOB_arm-unknown-linux-musl=lib$(1)-*.dylib.dSYM
CFG_JEMALLOC_CFLAGS_arm-unknown-linux-musl := -D__arm__ -msoft-float $(CFLAGS)
CFG_GCCISH_CFLAGS_arm-unknown-linux-musl := -Wall -g -fPIC -D__arm__ -msoft-float $(CFLAGS)
CFG_GCCISH_CXXFLAGS_arm-unknown-linux-musl := -fno-rtti $(CXXFLAGS)
CFG_GCCISH_LINK_FLAGS_arm-unknown-linux-musl := -shared -fPIC -g -msoft-float
CFG_GCCISH_DEF_FLAG_arm-unknown-linux-musl := -Wl,--export-dynamic,--dynamic-list=
CFG_LLC_FLAGS_arm-unknown-linux-musl :=
CFG_INSTALL_NAME_arm-unknown-linux-musl =
CFG_EXE_SUFFIX_arm-unknown-linux-musl :=
CFG_WINDOWSY_arm-unknown-linux-musl :=
CFG_UNIXY_arm-unknown-linux-musl := 1
CFG_LDPATH_arm-unknown-linux-musl :=
CFG_RUN_arm-unknown-linux-musl=$(2)
CFG_RUN_TARG_arm-unknown-linux-musl=$(call CFG_RUN_arm-unknown-linux-musl,,$(2))
RUSTC_FLAGS_arm-unknown-linux-musl :=
RUSTC_CROSS_FLAGS_arm-unknown-linux-musl := -C target-cpu=arm926ej-s -C target-feature="+v5te" -C soft-float
CFG_GNU_TRIPLE_arm-unknown-linux-musl := arm-unknown-linux-musl
CFG_THIRD_PARTY_OBJECTS_arm-unknown-linux-musl := crt1.o crti.o crtn.o
CFG_INSTALLED_OBJECTS_arm-unknown-linux-musl := crt1.o crti.o crtn.o

NATIVE_DEPS_libc_T_arm-unknown-linux-musl += libc.a
NATIVE_DEPS_std_T_arm-unknown-linux-musl += libunwind.a crt1.o crti.o crtn.o

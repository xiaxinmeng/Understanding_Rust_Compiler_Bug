
#arm-apple-darwin configuration for iOS
CFG_IOS_SDK=/Developer-3.2.6/Platforms/iPhoneOS.platform/Developer/usr
CFG_IOS_TOOLS=/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/usr/bin
CFG_IOS_FLAGS = -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS5.1.sdk/ -I/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS6.1.sdk/usr/include -I/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS6.1.sdk/usr/include/c++/4.2.1 -I/usr/include
CC_arm-apple-darwin= $(CFG_IOS_TOOLS)/arm-apple-darwin10-llvm-gcc-4.2
CXX_arm-apple-darwin= $(CFG_IOS_TOOLS)/arm-apple-darwin10-llvm-g++-4.2
CPP_arm-apple-darwin= $(CFG_IOS_TOOLS)/arm-apple-darwin10-llvm-g++-4.2
AR_arm-apple-darwin= $(CFG_IOS_TOOLS)/ar
CFG_LIB_NAME_arm-apple-darwin=lib$(1).dylib
CFG_LIB_GLOB_arm-apple-darwin=lib$(1)-*.dylib
CFG_LIB_DSYM_GLOB_arm-apple-darwin=lib$(1)-*.dylib.dSYM
CFG_GCCISH_CFLAGS_arm-apple-darwin := -Wall -Werror -g -fPIC $(CFG_IOS_FLAGS)
CFG_GCCISH_CXXFLAGS_arm-apple-darwin := -fno-rtti $(CFG_IOS_FLAGS)
CFG_GCCISH_LINK_FLAGS_arm-apple-darwin := -dynamiclib -lpthread -framework CoreServices -Wl,-no_compact_unwind 
CFG_GCCISH_DEF_FLAG_arm-apple-darwin := -Wl,-exported_symbols_list,
CFG_GCCISH_PRE_LIB_FLAGS_arm-apple-darwin :=
CFG_GCCISH_POST_LIB_FLAGS_arm-apple-darwin :=
CFG_DEF_SUFFIX_arm-apple-darwin := .darwin.def
CFG_INSTALL_NAME_arm-apple-darwin = -Wl,-install_name,@rpath/$(1)
CFG_LIBUV_LINK_FLAGS_arm-apple-darwin =
CFG_EXE_SUFFIX_arm-apple-darwin :=
CFG_WINDOWSY_arm-apple-darwin :=
CFG_UNIXY_arm-apple-darwin := 1
CFG_PATH_MUNGE_arm-apple-darwin := true
CFG_LDPATH_arm-apple-darwin :=
CFG_RUN_arm-apple-darwin=$(2)
CFG_RUN_TARG_arm-apple-darwin=$(call CFG_RUN_arm-apple-darwin,,$(2))

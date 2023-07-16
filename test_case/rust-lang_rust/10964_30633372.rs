 sh
# OS X 10.9, gcc is actually clang. This can cause some confusion in the build
# system, so if we find that gcc is clang, we should just use clang directly.
if [ $CFG_OSTYPE = apple-darwin -a -z "$CFG_ENABLE_CLANG" ]
then
    CFG_OSX_GCC_VERSION=$("$CFG_GCC" --version 2>&1 | grep "Apple LLVM version")
    if [ $? -eq 0 ]
    then
        step_msg "on OS X 10.9, forcing use of clang"
        CFG_ENABLE_CLANG=1
        putvar CFG_ENABLE_CLANG
    fi
fi

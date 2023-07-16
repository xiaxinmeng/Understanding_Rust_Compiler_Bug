 sh
    case $CFG_CLANG_VERSION in
        (3.0svn | 3.0 | 3.1* | 3.2* | 3.3* | 3.4* )
        step_msg "found ok version of CLANG: $CFG_CLANG_VERSION"
        CFG_C_COMPILER="clang"
        ;;
        (*)
        err "bad CLANG version: $CFG_CLANG_VERSION, need >=3.0svn"
        ;;
    esac

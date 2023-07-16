
check_dispatch() {
    if [ "$1" = submodule_changed ]; then
        # ignore $2 (branch id)
        verify_status $3 $4
    elif [ "$2" = beta ]; then
        echo "Requiring test passing for $3..."
        if check_tool_failed "$3"; then
            exit 4
        fi
    fi
}

status_check() {
    check_dispatch $1 beta book src/doc/book
    check_dispatch $1 beta nomicon src/doc/nomicon
    check_dispatch $1 beta reference src/doc/reference
    check_dispatch $1 beta rust-by-example src/doc/rust-by-example
    check_dispatch $1 beta edition-guide src/doc/edition-guide
    check_dispatch $1 beta rls src/tools/rls
    check_dispatch $1 beta rustfmt src/tools/rustfmt
    check_dispatch $1 beta clippy-driver src/tools/clippy
    # these tools are not required for beta to successfully branch
    check_dispatch $1 nightly miri src/tools/miri
    check_dispatch $1 nightly embedded-book src/doc/embedded-book
    check_dispatch $1 nightly rustc-guide src/doc/rustc-guide
}

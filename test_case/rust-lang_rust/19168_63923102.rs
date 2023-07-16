 rust
f() {
    rm -Rf "${TMP_DIR}"
    need_ok "failed to remove temporary installation directory"

    mkdir -p "${TMP_DIR}"
    need_ok "failed to create create temporary installation directory"

    ...
    rm -Rf "${TMP_DIR}"
    need_ok "couldn't rm temporary installation directory"
}

f

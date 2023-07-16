bash
if [[ -z "${INITIAL_RUST_CONFIGURE_ARGS+x}" ]]; then
    INITIAL_RUST_CONFIG=""
    echo "No initial Rust Configure Args set"
else
    INITIAL_RUST_CONFIG="${INITIAL_RUST_CONFIGURE_ARGS}"
    ciCommandSetEnv RUST_CONFIGURE_ARGS "${INITIAL_RUST_CONFIG}"
fi

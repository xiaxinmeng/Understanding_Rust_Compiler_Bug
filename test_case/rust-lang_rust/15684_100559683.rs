 sh
_RUST_LIB_PATH=/usr/local/lib
if ! echo "$LD_LIBRARY_PATH" | grep -q -E '(^|:)'"$_RUST_LIB_PATH"'(:|$)'; then
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$_RUST_LIB_PATH"
fi
unset _RUST_LIB_PATH

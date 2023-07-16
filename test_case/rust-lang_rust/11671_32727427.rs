
if [[ "$CFG_LIBDIR" != "$CFG_PREFIX"* ]]
then
    err "libdir must begin with the prefix. Use --prefix to set it accordingly."
fi

valopt libdir-relative "${CFG_LIBDIR:${#CFG_PREFIX}+1}" "install libraries (relative to prefix"

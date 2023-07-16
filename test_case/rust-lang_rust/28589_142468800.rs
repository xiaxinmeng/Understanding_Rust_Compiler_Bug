 bash
# /bin/sh on Solaris is not a POSIX compatible shell, but bash is.
if [ `uname -s` = 'SunOS' -a "${POSIX_SHELL}" != "true" ]; then
    POSIX_SHELL="true"
    export POSIX_SHELL
    exec /usr/bin/env bash $0 "$@"
fi
unset POSIX_SHELL # clear it so if we invoke other scripts, they run as bash as well

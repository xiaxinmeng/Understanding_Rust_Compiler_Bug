
#! /bin/sh

PORT="${PORTSDIR}/lang/rust-nightly"
OSVERSION=$(awk '/\#define __FreeBSD_version/ { print $3 }' "/usr/include/sys/param.h")
BOOTSTRAPS_DATE="$(make -C ${PORT} -V BOOTSTRAPS_DATE)" RUSTC_BOOTSTRAP="$(make -C ${PORT} -V RUSTC_BOOTSTRAP)"
FILESDIR="$(make -C ${PORT} -V FILESDIR)"
RUST_STD_BOOTSTRAP="$(make -C ${PORT} -V RUST_STD_BOOTSTRAP)"
CARGO_BOOTSTRAP="$(make -C ${PORT} -V CARGO_BOOTSTRAP)"
RUST_TARGET="$(make -C ${PORT} -V RUST_TARGET)"
DISTDIR="$(make -C ${PORT} -V '${DISTDIR}/${DIST_SUBDIR}')"
rm -rf build
mkdir -p "build/cache/${BOOTSTRAPS_DATE}"
ln -sf "${DISTDIR}/${RUSTC_BOOTSTRAP}" "build/cache/${BOOTSTRAPS_DATE}"
ln -sf "${DISTDIR}/${CARGO_BOOTSTRAP}" "build/cache/${BOOTSTRAPS_DATE}"
if [ ${OSVERSION} -lt 1200031 ]; then
        ln -sf "${DISTDIR}/${RUST_STD_BOOTSTRAP}" "build/cache/${BOOTSTRAPS_DATE}"
else
        TMP=$(mktemp -d)
        (
                set -ex
                cd "${TMP}"
                cc -O2 -pipe  -fstack-protector -fno-strict-aliasing \
                    -fPIC -c -o old_fstat.o "${FILESDIR}/old_fstat.c"
                tar -xf "${DISTDIR}/${RUST_STD_BOOTSTRAP}"
                RUST_STD_DIR="${RUST_STD_BOOTSTRAP##*/}"
                RUST_STD_DIR="${RUST_STD_DIR%%.*}"
                libstd="$(echo "${RUST_STD_DIR}/rust-std-${RUST_TARGET}/lib/rustlib/${RUST_TARGET}/lib/"libstd-*.rlib)"
                echo "libstd='${libstd}'"
                hash="${libstd##*/libstd-}"
                hash="${hash%.rlib}"
                std_o="$(ar t "${libstd}" | grep -E "^std-${hash}.*\.o$" | head -n 1)"
                ar x "${libstd}" "${std_o}"
                ld -r -o std.xx.o "${std_o}" old_fstat.o
                mv -f std.xx.o "${std_o}"
                ar r "${libstd}" "${std_o}"
                tar -c --format=ustar -f "${RUST_STD_BOOTSTRAP##*/}" \
                    "${RUST_STD_DIR}"
                mv -f "${RUST_STD_BOOTSTRAP##*/}" \
                    "${OLDPWD}/build/cache/${BOOTSTRAPS_DATE}/"
        )
        rm -rf "${TMP}"
fi

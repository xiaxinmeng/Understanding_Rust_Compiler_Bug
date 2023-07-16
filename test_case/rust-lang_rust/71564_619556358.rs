
#!/bin/sh

die() { printf '%s: %s\n' "$0" "$*" >&2; exit 1; }

if [ $# -eq 0 ] || [ x-h = "x$1" ] || [ x--help = "x$1" ] || [ xrustc = "x$1" ]; then
  printf 'Usage: %s "$(rustc --version)"\n' "$0" >&2
  exit 1
fi

VERSION="${1#rustc }"
VERSION="${VERSION% (*}"

SCRIPTPATH="src/ci/docker/scripts/musl.sh"
URL="https://raw.githubusercontent.com/rust-lang/rust/$VERSION/$SCRIPTPATH"

SCRIPT=$(curl -sf "$URL") || die "couldn't fetch $SCRIPTPATH from rust-lang/rust"

MUSL=$(sed -n 's/^MUSL=//p' <<< "$SCRIPT")
[ -n "$MUSL" ] || die "couldn't find MUSL variable"

echo "${MUSL#musl-}"

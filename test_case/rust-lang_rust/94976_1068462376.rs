bash
#!/bin/bash

set -o errexit
set -o pipefail

REPO=https://apt.dilos.org/dilos
A=dilos2
B=main

printf 'loading package manifest...\n'
pkgs=$(curl -sSf -L "$REPO/dists/$A/$B/binary-solaris-i386/Packages.gz" |
    gunzip)

function file_for {
         awk -v "p=$1" '
                $1 == "Package:" && $2 == p { q = 1; }
                /^$/ { q = 0; }
                q && $1 == "Filename:" { print $2; }' <<< "$pkgs"
}

files=(
        $(file_for libsendfile)
        $(file_for libsendfile-dev)
        $(file_for liblgrp)
        $(file_for liblgrp-dev)
)

td="$(mktemp -d)"

for f in "${files[@]}"; do
        b=$(basename "$f")
        printf 'package: %s\n' "$b"
        curl -sSfL -o "$td/$b" "$REPO/$f"
        7z -so x "$td/$b" 2>/dev/null | tar tvf - | sed 's/^/  /'
        printf '\n'
done

rm -rf "$td"

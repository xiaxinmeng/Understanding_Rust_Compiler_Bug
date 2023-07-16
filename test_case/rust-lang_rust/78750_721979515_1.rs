sh
#!/bin/sh

set -eu

if ! [ $# = 3 ]; then
	echo "usage: $0 <before> <after> <file>"
	exit 1
fi

before=$1
after=$2
file="$3"

rustdoc +$before "$file"
tidy-rustdoc doc doc-$before

rustdoc +$after "$file"
tidy-rustdoc doc doc-$after

diff -ur doc-$before doc-$after | delta

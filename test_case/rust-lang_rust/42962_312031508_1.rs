bash
#!/bin/bash

from=$1
to=$2

echo "from=$1 to=$2"

function ren () {
    echo "renaming $1"
    for i in $1/src/*.rs; do
	sed -i "s/$from/$to/g" $i
    done
    for i in $1/tests/*.rs; do
	sed -i "s/$from/$to/g" $i
    done;
    return 0
}

ren abstract-interp
ren amd64
ren analysis
ren avr
ren cli
ren core
ren data-flow
ren glue
ren graph-algos
ren mos6502
ren qt

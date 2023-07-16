 sh
#! /bin/sh

for COMMAND in \
    ./x86_64-unknown-linux-gnu/test/bench/msgsend-ring-mutex-arcs.stage1-x86_64-unknown-linux-gnu \
    ./x86_64-unknown-linux-gnu/test/bench/msgsend-ring-rw-arcs.stage1-x86_64-unknown-linux-gnu \
    ./x86_64-unknown-linux-gnu/test/bench/graph500-bfs.stage1-x86_64-unknown-linux-gnu
do
    echo ${COMMAND}
    for II in `seq 0 5`
    do
    ${COMMAND}
    done
    echo
done

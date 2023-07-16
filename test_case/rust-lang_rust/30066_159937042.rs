 shell
$ git clone -b nostd https://github.com/adjivas/io.git io && cd io # Nostd-library
$ rustc src/lib.rs --crate-name io_synesthesist \
                 --crate-type lib -g \
                 --emit=dep-info,link,llvm-bc \
                 -L dependency=$PWD/target/debug \
                 -L dependency=$PWD/io/target/debug/deps ; # Success
$ rustc src/main.rs --crate-type bin -g \
                  --emit=dep-info,link,llvm-bc \
                  -L dependency=$PWD/target/debug \
                  -L dependency=$PWD/io/target/debug/deps \
                  --extern io_synesthesist=libio_synesthesist.rlib ; # Success
$ llc -march=cpp main.bc -o main.cpp # Work, we progress!
$ llc -march=c main.bc -o main.c # Fail
llc: error: invalid target 'c'

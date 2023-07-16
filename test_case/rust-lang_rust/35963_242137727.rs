
sh: 1: cd: can't cd to src/src/tools/linkchecker
The command "docker run -v `pwd`:/build rust sh -c " ./configure --llvm-root=/usr/lib/llvm-3.7 && make tidy && make check-notidy -j4 && (cd src/src/tools/linkchecker && cargo run ../../lib*) "" exited with 2.

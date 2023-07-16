
test result: ok. 2445 passed; 0 failed; 60 ignored; 0 measured
The command "docker run -v `pwd`:/build rust sh -c " ./configure --llvm-root=/usr/lib/llvm-3.7 && make tidy && make check-notidy -j4 "" exited with 2.
Done. Your build exited with 1.

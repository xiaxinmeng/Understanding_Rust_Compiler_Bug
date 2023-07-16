 bash
$ pwd
/home/emacs/build/rust/x86_64-unknown-linux-gnu/stage2/bin
$ ./compiletest
./compiletest: symbol lookup error: ./compiletest: undefined symbol: _ZN11collections4hash3map36HashMap$LT$K$C$$u{20}V$C$$u{20}S$GT$6resize10_FILE_LINE20ha626c175c55fc0ddEmoE

$ ./rustc
./rustc: symbol lookup error: ./rustc: undefined symbol: _ZN2rt10lang_start20hc5e53532c7982aa4P7zE

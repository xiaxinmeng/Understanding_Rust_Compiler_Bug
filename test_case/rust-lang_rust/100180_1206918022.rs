sh
for dir in "${HOME}"/.rustup/toolchains/* ; do { for libs in "$dir"/lib/* ; do file "$libs"; ldd "$libs"; done; }; done

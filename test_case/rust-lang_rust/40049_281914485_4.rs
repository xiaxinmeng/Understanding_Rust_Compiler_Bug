
patchelf --set-interpreter /lib/ld-musl-x86_64.so.1 $BINARY
LD_LIBRARY_PATH=/path/to/libseccomp/src/.libs ./$BINARY
# success!

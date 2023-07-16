
cd /usr/local/bin; for i in *; do echo "BINARY $i"; otool -l $i | grep -A 3 LC_LOAD_DYLIB; done

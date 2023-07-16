
cd /usr/local/lib; for i in *.dylib; do otool -l $i | grep -A 3 LC_ID_DYLIB; done

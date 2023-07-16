
   ld -static /tmp/$EXECUTABLE_CRATE.o -Bdynamic -lfoo -Bstatic $PATH_TO_LIBSTD -lc -lunwind
   
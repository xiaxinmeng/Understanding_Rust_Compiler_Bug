
# Compile an object file with a symbol in it
$ echo 'void foo() {}' > foo.c
$ cc -g foo.c -o foo.o -c

# Link that object to a shared library, take a look at the output
$ cc foo.o -m64 -dynamiclib -o libfoo.dylib -Wl,-dylib
$ md5 libfoo.dylib
MD5 (libfoo.dylib) = e60e735b7c919c19259daddd04a625c8

# update the timestamp on the object file
$ sleep 1
$ touch foo.o

# now link the same way we did above
$ cc foo.o -m64 -dynamiclib -o libfoo.dylib -Wl,-dylib
$ md5 libfoo.dylib
MD5 (libfoo.dylib) = 9754a78562696bbe5912efd9fc892a83

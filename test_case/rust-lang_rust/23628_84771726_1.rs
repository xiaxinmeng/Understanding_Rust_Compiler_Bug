 console
$ gcc -Wall minstack.c -ldl -lpthread -o minstack
$ ./minstack
24640
$ dpkg-shlibdeps minstack
dpkg-shlibdeps: warning: binaries to analyze should already be installed in their package's directory
$ cat debian/substvars
shlibs:Depends=libc6 (>= 2.4)

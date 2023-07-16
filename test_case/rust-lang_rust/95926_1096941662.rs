
~$ cat /usr/bin/x86_64-linux-musl-gcc
#!/bin/sh
exec "${REALGCC:-x86_64-linux-gnu-gcc}" "$@" -specs "/usr/lib/x86_64-linux-musl/musl-gcc.specs"
~$ cat /usr/lib/x86_64-linux-musl/musl-gcc.specs
%rename cpp_options old_cpp_options

*cpp_options:
-nostdinc -isystem /usr/include/x86_64-linux-musl -isystem include%s %(old_cpp_options)

*cc1:
%(cc1_cpu) -nostdinc -isystem /usr/include/x86_64-linux-musl -isystem include%s

*link_libgcc:
-L/usr/lib/x86_64-linux-musl -L .%s

*libgcc:
libgcc.a%s %:if-exists(libgcc_eh.a%s)

*startfile:
%{!shared: /usr/lib/x86_64-linux-musl/Scrt1.o} /usr/lib/x86_64-linux-musl/crti.o crtbeginS.o%s

*endfile:
crtendS.o%s /usr/lib/x86_64-linux-musl/crtn.o

*link:
-dynamic-linker /lib/ld-musl-x86_64.so.1 -nostdlib %{shared:-shared} %{static:-static} %{rdynamic:-export-dynamic}

*esp_link:


*esp_options:


*esp_cpp_options:




mbp:cargo achin$ ls -l /opt//local/lib/libz.* 
-rwxr-xr-x  1 root  admin  157800 Nov 10  2013 /opt//local/lib/libz.1.2.8.dylib
lrwxr-xr-x  1 root  admin      16 Nov 10  2013 /opt//local/lib/libz.1.dylib -> libz.1.2.8.dylib
-rw-r--r--  1 root  admin  176424 Nov 10  2013 /opt//local/lib/libz.a
lrwxr-xr-x  1 root  admin      16 Nov 10  2013 /opt//local/lib/libz.dylib -> libz.1.2.8.dylib
mbp:cargo achin$ file /opt/local/lib/libz.*
/opt/local/lib/libz.1.2.8.dylib: Mach-O universal binary with 2 architectures
/opt/local/lib/libz.1.2.8.dylib (for architecture x86_64):      Mach-O 64-bit dynamically linked shared library x86_64
/opt/local/lib/libz.1.2.8.dylib (for architecture i386):        Mach-O dynamically linked shared library i386
/opt/local/lib/libz.1.dylib:     Mach-O universal binary with 2 architectures
/opt/local/lib/libz.1.dylib (for architecture x86_64):  Mach-O 64-bit dynamically linked shared library x86_64
/opt/local/lib/libz.1.dylib (for architecture i386):    Mach-O dynamically linked shared library i386
/opt/local/lib/libz.a:           Mach-O universal binary with 2 architectures
/opt/local/lib/libz.a (for architecture x86_64):        current ar archive random library
/opt/local/lib/libz.a (for architecture i386):  current ar archive random library
/opt/local/lib/libz.dylib:       Mach-O universal binary with 2 architectures
/opt/local/lib/libz.dylib (for architecture x86_64):    Mach-O 64-bit dynamically linked shared library x86_64
/opt/local/lib/libz.dylib (for architecture i386):      Mach-O dynamically linked shared library i386


[19:01:11]/tmp> ltrace -e malloc+free ./foo 
foo->malloc(784)                                                                                  = 0x21f0010
foo->malloc(96)                                                                                   = 0x21f03c0
foo->malloc(96)                                                                                   = 0x21f0430
libpthread.so.0->free(0)                                                                          = <void>
foo->malloc(784)                                                                                  = 0x21f05d0
foo->malloc(96)                                                                                   = 0x21f0980
foo->malloc(96)                                                                                   = 0x21f09f0
libpthread.so.0->free(0)                                                                          = <void>
foo->malloc(784)                                                                                  = 0x21f0b90
foo->malloc(96)                                                                                   = 0x21f0fc0
foo->malloc(96)                                                                                   = 0x21f1030
libpthread.so.0->free(0)                                                                          = <void>
foo->malloc(784)                                                                                  = 0x21f11d0
foo->malloc(96)                                                                                   = 0x21f1600
foo->malloc(96)                                                                                   = 0x21f1670
libpthread.so.0->free(0)                                                                          = <void>
foo->malloc(784)                                                                                  = 0x21f1810
foo->malloc(96)                                                                                   = 0x21f1c40
foo->malloc(96)                                                                                   = 0x21f1cb0
libpthread.so.0->free(0)                                                                          = <void>
foo->malloc(784)                                                                                  = 0x21f1e50
foo->malloc(96)                                                                                   = 0x21f2280
foo->malloc(96)                                                                                   = 0x21f22f0
libpthread.so.0->free(0)                                                                          = <void>
foo->malloc(784)                                                                                  = 0x21f2490
foo->malloc(96)                                                                                   = 0x21f29c0
foo->malloc(96)                                                                                   = 0x21f2a30
libpthread.so.0->free(0)                                                                          = <void>
foo->malloc(784)                                                                                  = 0x21f2bd0
foo->malloc(96)                                                                                   = 0x21f3100
foo->malloc(96)                                                                                   = 0x21f3170
libpthread.so.0->free(0)                                                                          = <void>
hello world
+++ exited (status 0) +++

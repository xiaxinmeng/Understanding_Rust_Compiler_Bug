
$ strace -s 0 ./read_to_end large.txt
...
openat(AT_FDCWD, "large.txt", O_RDONLY|O_CLOEXEC) = 3
read(3, ""..., 32)                      = 32
read(3, ""..., 32)                      = 32
read(3, ""..., 64)                      = 64
read(3, ""..., 128)                     = 128
read(3, ""..., 256)                     = 256
read(3, ""..., 512)                     = 512
read(3, ""..., 1024)                    = 1024
read(3, ""..., 2048)                    = 2048
read(3, ""..., 4096)                    = 4096
read(3, ""..., 8192)                    = 8192
read(3, ""..., 16384)                   = 16384
read(3, ""..., 32768)                   = 32768
read(3, ""..., 65536)                   = 65536
mmap(NULL, 266240, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7fbd9ad0b000
read(3, ""..., 131072)                  = 131072
mremap(0x7fbd9ad0b000, 266240, 528384, MREMAP_MAYMOVE) = 0x7fbd9ac8a000
read(3, ""..., 262144)                  = 262144
mremap(0x7fbd9ac8a000, 528384, 1052672, MREMAP_MAYMOVE) = 0x7fbd9ab89000
read(3, ""..., 524288)                  = 524288
mremap(0x7fbd9ab89000, 1052672, 2101248, MREMAP_MAYMOVE) = 0x7fbd9a988000
read(3, ""..., 1048576)                 = 1048576
mremap(0x7fbd9a988000, 2101248, 4198400, MREMAP_MAYMOVE) = 0x7fbd9a587000
read(3, ""..., 2097152)                 = 2097152
mremap(0x7fbd9a587000, 4198400, 8392704, MREMAP_MAYMOVE) = 0x7fbd99d86000
read(3, ""..., 4194304)                 = 4194304
mremap(0x7fbd99d86000, 8392704, 16781312, MREMAP_MAYMOVE) = 0x7fbd98d85000
read(3, ""..., 8388608)                 = 1711392
read(3, "", 6677216)                    = 0
munmap(0x7fbd98d85000, 16781312)        = 0
close(3)                                = 0
...

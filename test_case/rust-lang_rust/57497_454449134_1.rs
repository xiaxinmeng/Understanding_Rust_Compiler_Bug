
$ readelf -Ws main | grep GLIBC | grep -v GLIBC_2.[23]
    22: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND __stack_chk_fail@GLIBC_2.4 (8)
    48: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND memcpy@GLIBC_2.14 (12)
   693: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND __stack_chk_fail@@GLIBC_2.4
   804: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND memcpy@@GLIBC_2.14
$ docker run --rm -it -v $(pwd):/io centos:7 /io/main
Hello, world!

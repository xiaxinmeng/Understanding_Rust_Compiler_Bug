
$ gcc -fPIC -o main main.c
$ file main
main: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=348339ee88b63ec5fd57c7ebcee0ec4667053cc7, for GNU/Linux 3.2.0, not stripped
$ gcc -o main main.c
$ file main
main: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=348339ee88b63ec5fd57c7ebcee0ec4667053cc7, for GNU/Linux 3.2.0, not stripped
$ gcc -o main main.c  -static
$ file main
main: ELF 64-bit LSB executable, x86-64, version 1 (GNU/Linux), statically linked, BuildID[sha1]=abc1157e27c07d6849a0f9f2f574f35f3d0b934a, for GNU/Linux 3.2.0, not stripped
$ gcc -o main main.c  -fpie
$ file main
main: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=348339ee88b63ec5fd57c7ebcee0ec4667053cc7, for GNU/Linux 3.2.0, not stripped

dkm@arrakis:~$ gcc -o main main.c  -fpie
dkm@arrakis:~$ file main
main: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=348339ee88b63ec5fd57c7ebcee0ec4667053cc7, for GNU/Linux 3.2.0, not stripped
$ readelf -h main
ELF Header:
  Type:                              DYN (Shared object file)
$ gcc -o main main.c  -static
$ readelf -h main
ELF Header:
  Type:                              EXEC (Executable file)

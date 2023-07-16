
$ arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/examples/minimal minimal.bin

ls -l minimal.bin
-rwxr-xr-x 1 japaric japaric 2340 minimal.bin

$ arm-none-eabi-strings minimal.bin
pGpGZ
FjFO
|@aF
5UE
5EE
XF=F
;ZEO
[       d
03jF
[       d
*The answer is
00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899

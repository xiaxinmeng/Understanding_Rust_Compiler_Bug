
root@2303509b5596:/#` mips64el-linux-gnuabi64-objdump -d hello-world | grep lwc1
 b74:   c4400d98        lwc1    $f0,3480(v0)
 b80:   c4400d9c        lwc1    $f0,3484(v0)
 b88:   c7c10000        lwc1    $f1,0(s8)
 b8c:   c7c00004        lwc1    $f0,4(s8)


$ grep -R BN_is_zero /usr/include  
/usr/include/openssl/bn.h:#define BN_is_zero(a)       ((a)->top == 0)

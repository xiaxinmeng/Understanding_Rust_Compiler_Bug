
$ objdump -t  /usr/local/opt/openssl/lib/libssl.1.0.0.dylib|grep '_X509_verify_cert$'
0000000000000000         *UND*	_X509_verify_cert
$ objdump -t  /usr/local/opt/openssl/lib/libcrypto.dylib|grep '_X509_verify_cert$'
00000000000c8200 g     F __TEXT,__text	_X509_verify_cert
$ otool -L  /usr/local/opt/openssl/lib/libssl.1.0.0.dylib
/usr/local/opt/openssl/lib/libssl.1.0.0.dylib:
	/usr/local/opt/openssl/lib/libssl.1.0.0.dylib (compatibility version 1.0.0, current version 1.0.0)
	/usr/local/Cellar/openssl/1.0.2s/lib/libcrypto.1.0.0.dylib (compatibility version 1.0.0, current version 1.0.0)
	/usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1252.250.1)

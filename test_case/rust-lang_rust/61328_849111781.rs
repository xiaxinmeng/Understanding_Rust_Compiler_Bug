
  = note: /usr/lib/gcc/aarch64-alpine-linux-musl/10.2.1/../../../../aarch64-alpine-linux-musl/bin/ld: /root/sccache/src/target/aarch64-unknown-linux-musl/release/deps/libopenssl_sys-fcc8125410f43229.rlib(ssl_cert.o): in function `ssl_cert_free':
          ssl_cert.c:(.text.ssl_cert_free+0x1c): undefined reference to `__aarch64_ldadd4_relax'
          /usr/lib/gcc/aarch64-alpine-linux-musl/10.2.1/../../../../aarch64-alpine-linux-musl/bin/ld: /root/sccache/src/target/aarch64-unknown-linux-musl/release/deps/libopenssl_sys-fcc8125410f43229.rlib(ssl_lib.o): in function `SSL_CTX_up_ref':
          ssl_lib.c:(.text.SSL_CTX_up_ref+0x14): undefined reference to `__aarch64_ldadd4_relax'
          /usr/lib/gcc/aarch64-alpine-linux-musl/10.2.1/../../../../aarch64-alpine-linux-musl/bin/ld: /root/sccache/src/target/aarch64-unknown-linux-musl/release/deps/libopenssl_sys-fcc8125410f43229.rlib(ssl_lib.o): in function `SSL_CTX_free':
          ssl_lib.c:(.text.SSL_CTX_free+0x1c): undefined reference to `__aarch64_ldadd4_relax'
          /usr/lib/gcc/aarch64-alpine-linux-musl/10.2.1/../../../../aarch64-alpine-linux-musl/bin/ld: /root/sccache/src/target/aarch64-unknown-linux-musl/release/deps/libopenssl_sys-fcc8125410f43229.rlib(ssl_lib.o): in function `SSL_free':
          ssl_lib.c:(.text.SSL_free+0x1c): undefined reference to `__aarch64_ldadd4_relax'
          /usr/lib/gcc/aarch64-alpine-linux-musl/10.2.1/../../../../aarch64-alpine-linux-musl/bin/ld: /root/sccache/src/target/aarch64-unknown-linux-musl/release/deps/libopenssl_sys-fcc8125410f43229.rlib(ssl_sess.o): in function `SSL_SESSION_free':
          ssl_sess.c:(.text.SSL_SESSION_free+0x1c): undefined reference to `__aarch64_ldadd4_relax'
          /usr/lib/gcc/aarch64-alpine-linux-musl/10.2.1/../../../../aarch64-alpine-linux-musl/bin/ld: /root/sccache/src/target/aarch64-unknown-linux-musl/release/deps/libopenssl_sys-fcc8125410f43229.rlib(ssl_sess.o):ssl_sess.c:(.text.SSL_SESSION_up_ref+0x14): more undefined references to `__aarch64_ldadd4_relax' follow
          /usr/lib/gcc/aarch64-alpine-linux-musl/10.2.1/../../../../aarch64-alpine-linux-musl/bin/ld: /root/sccache/src/target/aarch64-unknown-linux-musl/release/deps/libopenssl_sys-fcc8125410f43229.rlib(lhash.o): in function `getrn':
          lhash.c:(.text.getrn+0x3c): undefined reference to `__aarch64_ldadd8_relax'
          /usr/lib/gcc/aarch64-alpine-linux-musl/10.2.1/../../../../aarch64-alpine-linux-musl/bin/ld: lhash.c:(.text.getrn+0x94): undefined reference to `__aarch64_ldadd8_relax'
          /usr/lib/gcc/aarch64-alpine-linux-musl/10.2.1/../../../../aarch64-alpine-linux-musl/bin/ld: lhash.c:(.text.getrn+0xac): undefined reference to `__aarch64_ldadd8_relax'
          /usr/lib/gcc/aarch64-alpine-linux-musl/10.2.1/../../../../aarch64-alpine-linux-musl/bin/ld: /root/sccache/src/target/aarch64-unknown-linux-musl/release/deps/libopenssl_sys-fcc8125410f43229.rlib(lhash.o): in function `OPENSSL_LH_retrieve':
          lhash.c:(.text.OPENSSL_LH_retrieve+0x48): undefined reference to `__aarch64_ldadd8_relax'
          /usr/lib/gcc/aarch64-alpine-linux-musl/10.2.1/../../../../aarch64-alpine-linux-musl/bin/ld: lhash.c:(.text.OPENSSL_LH_retrieve+0x80): undefined reference to `__aarch64_ldadd8_relax'
          /usr/lib/gcc/aarch64-alpine-linux-musl/10.2.1/../../../../aarch64-alpine-linux-musl/bin/ld: /root/sccache/src/target/aarch64-unknown-linux-musl/release/deps/libopenssl_sys-fcc8125410f43229.rlib(drbg_lib.o): in function `RAND_DRBG_instantiate':
          drbg_lib.c:(.text.RAND_DRBG_instantiate+0x2d8): undefined reference to `__aarch64_ldadd4_relax'
          /usr/lib/gcc/aarch64-alpine-linux-musl/10.2.1/../../../../aarch64-alpine-linux-musl/bin/ld: /root/sccache/src/target/aarch64-unknown-linux-musl/release/deps/libopenssl_sys-fcc8125410f43229.rlib(drbg_lib.o): in function `RAND_DRBG_reseed':
          drbg_lib.c:(.text.RAND_DRBG_reseed+0x20c): undefined reference to `__aarch64_ldadd4_relax'
          /usr/lib/gcc/aarch64-alpine-linux-musl/10.2.1/../../../../aarch64-alpine-linux-musl/bin/ld: /root/sccache/src/target/aarch64-unknown-linux-musl/release/deps/libopenssl_sys-fcc8125410f43229.rlib(rsa_lib.o): in function `RSA_free':
          rsa_lib.c:(.text.RSA_free+0x1c): undefined reference to `__aarch64_ldadd4_relax'
          /usr/lib/gcc/aarch64-alpine-linux-musl/10.2.1/../../../../aarch64-alpine-linux-musl/bin/ld: /root/sccache/src/target/aarch64-unknown-linux-musl/release/deps/libopenssl_sys-fcc8125410f43229.rlib(threads_pthread.o): in function `CRYPTO_atomic_add':
          threads_pthread.c:(.text.CRYPTO_atomic_add+0x1c): undefined reference to `__aarch64_ldadd4_acq_rel'
          /usr/lib/gcc/aarch64-alpine-linux-musl/10.2.1/../../../../aarch64-alpine-linux-musl/bin/ld: /root/sccache/src/target/aarch64-unknown-linux-musl/release/deps/libopenssl_sys-fcc8125410f43229.rlib(x509_lu.o): in function `X509_STORE_free':
          x509_lu.c:(.text.X509_STORE_free+0x1c): undefined reference to `__aarch64_ldadd4_relax'
          /usr/lib/gcc/aarch64-alpine-linux-musl/10.2.1/../../../../aarch64-alpine-linux-musl/bin/ld: /root/sccache/src/target/aarch64-unknown-linux-musl/release/deps/libopenssl_sys-fcc8125410f43229.rlib(x509_lu.o): in function `X509_STORE_up_ref':
          x509_lu.c:(.text.X509_STORE_up_ref+0x14): undefined reference to `__aarch64_ldadd4_relax'
          /usr/lib/gcc/aarch64-alpine-linux-musl/10.2.1/../../../../aarch64-alpine-linux-musl/bin/ld: /root/sccache/src/target/aarch64-unknown-linux-musl/release/deps/libopenssl_sys-fcc8125410f43229.rlib(x509_set.o): in function `X509_up_ref':
          x509_set.c:(.text.X509_up_ref+0x14): undefined reference to `__aarch64_ldadd4_relax'
          /usr/lib/gcc/aarch64-alpine-linux-musl/10.2.1/../../../../aarch64-alpine-linux-musl/bin/ld: /root/sccache/src/target/aarch64-unknown-linux-musl/release/deps/libopenssl_sys-fcc8125410f43229.rlib(x509cset.o): in function `X509_CRL_up_ref':
          x509cset.c:(.text.X509_CRL_up_ref+0x14): undefined reference to `__aarch64_ldadd4_relax'
          collect2: error: ld returned 1 exit status

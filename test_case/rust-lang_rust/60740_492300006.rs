plain
voltdb
wry
zxing-cpp
==> Downloading https://homebrew.bintray.com/bottles/xz-5.2.4.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/e7/e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880?__gda__=exp=1557840298~hmac=0e80a56da1ece48e8e125e9685be522f71e364f3562b141eafbab621e4a1204c&response-content-disposition=attachment%3Bfilename%3D%22xz-5.2.4.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX1_5h2H-0cv7QD3B3Bjq61jV2i2nceinYc0HipAxpDS2Vzv6WgxkIbdkfNjuu-ubzsJBP1xbxius0vc_82dtDdEqz-PE421ZgEAQ8rRDXO6XX_8GPZubeuuB5jkD77S5lYrw6F3-nCTI8A&response-X-Checksum-Sha1=32dc0b28e61f32b40c20e2993418aa8cb6e746d5&response-X-Checksum-Sha2=e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880
🍺  /usr/local/Cellar/xz/5.2.4: 92 files, 1MB
==> `brew cleanup` has not been run in 30 days, running now...
Removing: /Users/travis/Library/Caches/Homebrew/boost-1.66.0.high_sierra.bottle.tar.gz... (84.6MB)
Removing: /Users/travis/Library/Caches/Homebrew/carthage-0.28.0.high_sierra.bottle.tar.gz... (8.3MB)
---
Pruned 0 symbolic links and 5 directories from /usr/local
==> Installing dependencies for swig: pcre
==> Installing swig dependency: pcre
==> Downloading https://homebrew.bintray.com/bottles/pcre-8.43.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/03/0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861?__gda__=exp=1557840314~hmac=5cf802d97dd4fc8e53f0bba74378e13e0367b4f8831d47997a531d5ed1845691&response-content-disposition=attachment%3Bfilename%3D%22pcre-8.43.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX18kvtb5JBYfr-OTAM1zZUWEKzaug_d1xCNE6U7lbXoRIEw4abaYQJu66f-ITpOhDEDJca5nonRHDbgUzKx37arvRFhhfiBIYMb_KWvMlw0vxAeCnDH-z-Pam0sNJFjlt3qxFyDGA0I1HA&response-X-Checksum-Sha1=c67d4b99bb245f0ea56b34118dd6325b06a7250c&response-X-Checksum-Sha2=0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861
🍺  /usr/local/Cellar/pcre/8.43: 204 files, 5.5MB
==> Installing swig
==> Downloading https://homebrew.bintray.com/bottles/swig-4.0.0.high_sierra.bottle.tar.gz
==> Downloading https://homebrew.bintray.com/bottles/swig-4.0.0.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/ae/aed79cb436b3a0ac5812c4085e3121ffd62866397b8c7eaa06815ed8ec1e22b7?__gda__=exp=1557840316~hmac=f82bdb3bf7a3f93be2e102eb64b0736e353c40b373cfb74ccbd4a38782b21fae&response-content-disposition=attachment%3Bfilename%3D%22swig-4.0.0.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX18JIecMcr2dYa1dVe_VbvOZKH2u5Ncx8fTO4NtckCvZ-3n_pKT3RL7uWqff4rgqLiOoTPMoUh8CzNwMljjBCocnvSbERHIr_nJNtXgdkwsV_sIEQwjX7FVoYkDX6sL_WDX3HA9Wy0Ocxw&response-X-Checksum-Sha1=a9c428aee4337d91061a69c02d7ae508b627d03f&response-X-Checksum-Sha2=aed79cb436b3a0ac5812c4085e3121ffd62866397b8c7eaa06815ed8ec1e22b7
🍺  /usr/local/Cellar/swig/4.0.0: 722 files, 5.4MB
travis_time:end:05590041:start=1557839238160767000,finish=1557839636599136000,duration=398438369000
travis_fold:end:install
travis_fold:start:before_script.1
---
[00:03:34]       Memory: 8 GB
[00:03:34]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:34]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:34]       SMC Version (system): 2.8f0
[00:03:34]       Serial Number (system): VMM7TjG73MCb
[00:03:34] 
[00:03:34] hw.ncpu: 4
[00:03:34] hw.byteorder: 1234
[00:03:34] hw.memsize: 8589934592
---
upload: deploy/56043b7b6108e7f7d187e2e6bad36f79e51848fa/rust-analysis-nightly-armv7-apple-ios.tar.xz to s3://rust-lang-ci2/rustc-builds/56043b7b6108e7f7d187e2e6bad36f79e51848fa/rust-analysis-nightly-armv7-apple-ios.tar.xz
upload: deploy/56043b7b6108e7f7d187e2e6bad36f79e51848fa/llvm-tools-nightly-x86_64-apple-darwin.tar.gz to s3://rust-lang-ci2/rustc-builds/56043b7b6108e7f7d187e2e6bad36f79e51848fa/llvm-tools-nightly-x86_64-apple-darwin.tar.gz
upload: deploy/56043b7b6108e7f7d187e2e6bad36f79e51848fa/rust-analysis-nightly-i386-apple-ios.tar.xz to s3://rust-lang-ci2/rustc-builds/56043b7b6108e7f7d187e2e6bad36f79e51848fa/rust-analysis-nightly-i386-apple-ios.tar.xz
upload: deploy/56043b7b6108e7f7d187e2e6bad36f79e51848fa/rust-analysis-ni
The job exceeded the maximum log length, and has been terminated.

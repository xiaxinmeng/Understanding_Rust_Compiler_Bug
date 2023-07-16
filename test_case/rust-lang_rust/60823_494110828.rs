plain
Updated 1 tap (homebrew/core).
==> New Formulae
atlantis
==> Downloading https://homebrew.bintray.com/bottles/xz-5.2.4.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/e7/e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880?__gda__=exp=1558376869~hmac=5b1902ac8f3e2730fc6fd5789bbb23763456f76547b8d71c76b9f225577bd08f&response-content-disposition=attachment%3Bfilename%3D%22xz-5.2.4.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX19CjG7DdB46_X5nXdVeptumOkjRqnRgjWvklrsXmoLgH7d1PfHRLUg8u4GNdbiFrydz5UBCvhXrmeXxTYbORlCqTmU8SWhngsx85ORDAEpJ39cZZhGzdf8Y57kFbZ_jHPqVl9IRe0nSkg&response-X-Checksum-Sha1=32dc0b28e61f32b40c20e2993418aa8cb6e746d5&response-X-Checksum-Sha2=e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880
🍺  /usr/local/Cellar/xz/5.2.4: 92 files, 1MB
==> `brew cleanup` has not been run in 30 days, running now...
Removing: /Users/travis/Library/Caches/Homebrew/boost-1.66.0.high_sierra.bottle.tar.gz... (84.6MB)
Removing: /Users/travis/Library/Caches/Homebrew/carthage-0.28.0.high_sierra.bottle.tar.gz... (8.3MB)
---
Pruned 0 symbolic links and 5 directories from /usr/local
==> Installing dependencies for swig: pcre
==> Installing swig dependency: pcre
==> Downloading https://homebrew.bintray.com/bottles/pcre-8.43.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/03/0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861?__gda__=exp=1558376884~hmac=d00b4f6f6a60601d66eba15c851475df85eea8413fdf4599da8a16c2100dc50b&response-content-disposition=attachment%3Bfilename%3D%22pcre-8.43.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX18IiJ7hx5XMPr5kG7HKOcwCYoP8WIjANEEDnycmiDFs9TLZVPYpYNjNhMi6q3pjkfV4_g0oc0LhN5e-lv-04D2mvum5xE8YDBLUOQ7Sts5gAm08wr3_j6mu9HMHEuJL-zdsH5bOW_bjJw&response-X-Checksum-Sha1=c67d4b99bb245f0ea56b34118dd6325b06a7250c&response-X-Checksum-Sha2=0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861
🍺  /usr/local/Cellar/pcre/8.43: 204 files, 5.5MB
==> Installing swig
==> Downloading https://homebrew.bintray.com/bottles/swig-4.0.0.high_sierra.bottle.tar.gz
==> Downloading https://homebrew.bintray.com/bottles/swig-4.0.0.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/ae/aed79cb436b3a0ac5812c4085e3121ffd62866397b8c7eaa06815ed8ec1e22b7?__gda__=exp=1558376887~hmac=5f1f4cc52614c9e84e7a2429d740e343e057ad3a8590591a5b012ae52999704c&response-content-disposition=attachment%3Bfilename%3D%22swig-4.0.0.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX1-fzI3C-irX9EcgTqbbSi3d8xcn-_cqQC0r3Dm0HlqXR9xU5_g5yyyG6SL0_ocg6V6nOFkx2rqoxls13ZE0E8EHIYPINVMkQq75SL0cku-3HeiWeFJZdF82crN1XnQT_-u6wD9sg5RlvQ&response-X-Checksum-Sha1=a9c428aee4337d91061a69c02d7ae508b627d03f&response-X-Checksum-Sha2=aed79cb436b3a0ac5812c4085e3121ffd62866397b8c7eaa06815ed8ec1e22b7
🍺  /usr/local/Cellar/swig/4.0.0: 722 files, 5.4MB
travis_time:end:292f8bae:start=1558375802120114000,finish=1558376205914679000,duration=403794565000
travis_fold:end:install
travis_fold:start:before_script.1
---
[00:03:37]       Memory: 8 GB
[00:03:37]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:37]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:37]       SMC Version (system): 2.8f0
[00:03:37]       Serial Number (system): VMqvU5WxsmcO
[00:03:37] 
[00:03:37] hw.ncpu: 4
[00:03:37] hw.byteorder: 1234
[00:03:37] hw.memsize: 8589934592
---
[00:11:01]    Compiling rustc_errors v0.0.0 (/Users/travis/build/rust-lang/rust/src/librustc_errors)
[00:11:11] [RUSTC-TIMING] rustc_errors test:false 10.170
[00:12:52] [RUSTC-TIMING] syntax test:false 101.702
[00:12:52]    Compiling syntax_ext v0.0.0 (/Users/travis/build/rust-lang/rust/src/libsyntax_ext)
No output has been received in the last 30m0s, this potentially indicates a stalled build or something wrong with the build itself.
Check the details on how to adjust your build configuration on: https://docs.travis-ci.com/user/common-build-problems/#Build-times-out-because-no-output-was-received
The build has been terminated

plain
==> Auto-updated Homebrew!
Updated 1 tap (caskroom/cask).
No changes to formulae.
==> Downloading https://homebrew.bintray.com/bottles/xz-5.2.4.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/e7/e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880?__gda__=exp=1557814770~hmac=215501b9307d74b35fd94ac54d2f45e5894a73db9a000d4fad6c4f190aeb95a1&response-content-disposition=attachment%3Bfilename%3D%22xz-5.2.4.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX19RCvPcqH2qgMQzgMaoeDFru_ZOEwmw0gi01_Ofe8i80vZwAVHamYjzFniAppKhb6t8Jgmk7knuACaYAVxMMtjmipT-m4Xsq6K94_utcMkSw7hbnJBMw5lzgRMs6srCaVTub8en88KBBw&response-X-Checksum-Sha1=32dc0b28e61f32b40c20e2993418aa8cb6e746d5&response-X-Checksum-Sha2=e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880
🍺  /usr/local/Cellar/xz/5.2.4: 92 files, 1MB
==> `brew cleanup` has not been run in 30 days, running now...
Removing: /Users/travis/Library/Caches/Homebrew/boost-1.66.0.high_sierra.bottle.tar.gz... (84.6MB)
Removing: /Users/travis/Library/Caches/Homebrew/carthage-0.28.0.high_sierra.bottle.tar.gz... (8.3MB)
---
Pruned 0 symbolic links and 5 directories from /usr/local
==> Installing dependencies for swig: pcre
==> Installing swig dependency: pcre
==> Downloading https://homebrew.bintray.com/bottles/pcre-8.43.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/03/0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861?__gda__=exp=1557814781~hmac=d5e2b5ce405e9c595b44a81ed37a8be5a47e9301b159be33d9af38ce9f488959&response-content-disposition=attachment%3Bfilename%3D%22pcre-8.43.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX1-zuN6zkH2L_KZ2Nip7D_Aus32N031GolM0bfLR9q7T2gtEZkj8aLneCodQkuWW_N_QZFpMkB4pCM7-dJR2MFa_JiQ_2MFrknaT0CjLZIWqZNCEUiMEP0gi2_WkfaZpb_ss7UZzqHtohg&response-X-Checksum-Sha1=c67d4b99bb245f0ea56b34118dd6325b06a7250c&response-X-Checksum-Sha2=0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861
🍺  /usr/local/Cellar/pcre/8.43: 204 files, 5.5MB
==> Installing swig
==> Downloading https://homebrew.bintray.com/bottles/swig-4.0.0.high_sierra.bottle.tar.gz
==> Downloading https://homebrew.bintray.com/bottles/swig-4.0.0.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/ae/aed79cb436b3a0ac5812c4085e3121ffd62866397b8c7eaa06815ed8ec1e22b7?__gda__=exp=1557814783~hmac=7a2f74c253784b1655c164fed2edb507bd15208e6abde72d7dbcda51769c29d0&response-content-disposition=attachment%3Bfilename%3D%22swig-4.0.0.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX1-VqhiyjMS6HQca43PvjgCRcAlon1JgZYkYeQxPC8nWGPd9_-o2D3kONHj-LiOBv2QOXf3Fov5gWBo5TBCeOkQ4Sn0l4wHvpwNvVL6Dt0hvywmPDjD_0kVA4O9iDVF3Yx7RdbcnCXW_WA&response-X-Checksum-Sha1=a9c428aee4337d91061a69c02d7ae508b627d03f&response-X-Checksum-Sha2=aed79cb436b3a0ac5812c4085e3121ffd62866397b8c7eaa06815ed8ec1e22b7
🍺  /usr/local/Cellar/swig/4.0.0: 722 files, 5.4MB
travis_time:end:061bf342:start=1557813706998465000,finish=1557814100831783000,duration=393833318000
travis_fold:end:install
travis_fold:start:before_script.1
---
[00:03:21]       Memory: 8 GB
[00:03:21]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:21]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:21]       SMC Version (system): 2.8f0
[00:03:21]       Serial Number (system): VMqURky+qS4w
[00:03:21] 
[00:03:21] hw.ncpu: 4
[00:03:21] hw.byteorder: 1234
[00:03:21] hw.memsize: 8589934592
---
[01:27:31]    Compiling clippy_lints v0.0.212 (/Users/travis/build/rust-lang/rust/src/tools/clippy/clippy_lints)
[01:27:34] error[E0432]: unresolved import `syntax::symbol::keywords`
[01:27:34]   --> src/tools/clippy/clippy_lints/src/utils/mod.rs:45:22
[01:27:34]    |
[01:27:34] 45 | use syntax::symbol::{keywords, Symbol};
[01:27:34]    |                      ^^^^^^^^ no `keywords` in `symbol`
[01:27:34] error[E0432]: unresolved import `syntax::symbol::keywords`
[01:27:34]  --> src/tools/clippy/clippy_lints/src/lifetimes.rs:9:5
[01:27:34]   |
[01:27:34] 9 | use syntax::symbol::keywords;
---
[01:27:45] 
[01:27:45] error[E0432]: unresolved import `syntax::symbol::keywords`
[01:27:45]   --> src/tools/clippy/clippy_lints/src/utils/mod.rs:45:22
[01:27:45]    |
[01:27:45] 45 | use syntax::symbol::{keywords, Symbol};
[01:27:45]    |                      ^^^^^^^^ no `keywords` in `symbol`
[01:27:45] error[E0432]: unresolved import `syntax::symbol::keywords`
[01:27:45]  --> src/tools/clippy/clippy_lints/src/lifetimes.rs:9:5
[01:27:45]   |
[01:27:45] 9 | use syntax::symbol::keywords;
---
[01:29:04]    Compiling rayon v1.0.1
[01:29:07] error[E0432]: unresolved import `syntax::symbol::keywords`
[01:29:07]   --> src/tools/clippy/clippy_lints/src/utils/mod.rs:45:22
[01:29:07]    |
[01:29:07] 45 | use syntax::symbol::{keywords, Symbol};
[01:29:07]    |                      ^^^^^^^^ no `keywords` in `symbol`
[01:29:07] error[E0432]: unresolved import `syntax::symbol::keywords`
[01:29:07]  --> src/tools/clippy/clippy_lints/src/lifetimes.rs:9:5
[01:29:07]   |
[01:29:07] 9 | use syntax::symbol::keywords;
---
[02:23:43]    Compiling clippy_lints v0.0.212 (/Users/travis/build/rust-lang/rust/src/tools/clippy/clippy_lints)
[02:23:47] error[E0432]: unresolved import `syntax::symbol::keywords`
[02:23:47]   --> src/tools/clippy/clippy_lints/src/utils/mod.rs:45:22
[02:23:47]    |
[02:23:47] 45 | use syntax::symbol::{keywords, Symbol};
[02:23:47]    |                      ^^^^^^^^ no `keywords` in `symbol`
[02:23:47] error[E0432]: unresolved import `syntax::symbol::keywords`
[02:23:47]   --> src/tools/clippy/clippy_lints/src/utils/mod.rs:45:22
[02:23:47]    |
[02:23:47]    |
[02:23:47] 45 | use syntax::symbol::{keywords, Symbol};
[02:23:47]    |                      ^^^^^^^^ no `keywords` in `symbol`
[02:23:47] error[E0432]: unresolved import `syntax::symbol::keywords`
[02:23:47]  --> src/tools/clippy/clippy_lints/src/lifetimes.rs:9:5
[02:23:47]   |
[02:23:47] 9 | use syntax::symbol::keywords;
---
[02:24:14]    Compiling racer v2.1.22
[02:24:17] error[E0432]: unresolved import `syntax::symbol::keywords`
[02:24:17]   --> src/tools/clippy/clippy_lints/src/utils/mod.rs:45:22
[02:24:17]    |
[02:24:17] 45 | use syntax::symbol::{keywords, Symbol};
[02:24:17]    |                      ^^^^^^^^ no `keywords` in `symbol`
[02:24:17] error[E0432]: unresolved import `syntax::symbol::keywords`
[02:24:17]  --> src/tools/clippy/clippy_lints/src/lifetimes.rs:9:5
[02:24:17]   |
[02:24:17] 9 | use syntax::symbol::keywords;
---
upload: deploy/e598681d9a59feedcd638666232dcac1d8826314/rust-analysis-nightly-armv7-apple-ios.tar.xz to s3://rust-lang-ci2/rustc-builds/e598681d9a59feedcd638666232dcac1d8826314/rust-analysis-nightly-armv7-apple-ios.tar.xz
upload: deploy/e598681d9a59feedcd638666232dcac1d8826314/rust-analysis-nightly-armv7s-apple-ios.tar.gz to s3://rust-lang-ci2/rustc-builds/e598681d9a59feedcd638666232dcac1d8826314/rust-analysis-nightly-armv7s-apple-ios.tar.gz
upload: deploy/e598681d9a59feedcd638666232dcac1d8826314/rust-analysis-nightly-i386-apple-ios.tar.xz to s3://rust-lang-ci2/rustc-builds/e598681d9a59feedcd638666232dcac1d8826314/rust-analysis-nightly-i386-apple-ios.tar.xz
upload: deploy/e598681d9a59feedcd638666232dcac1d8826314/rust-analysis-nightly-x8
The job exceeded the maximum log length, and has been terminated.

plain
voltdb
wry
zxing-cpp
==> Downloading https://homebrew.bintray.com/bottles/xz-5.2.4.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/e7/e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880?__gda__=exp=1554838829~hmac=e03c02d3b989d37683f6a173e6fa70c13bb756b03f25fae2fd72056c1bc8a6e4&response-content-disposition=attachment%3Bfilename%3D%22xz-5.2.4.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX19fHX-ZsJ70XvkQA2EUQbRX95_I1DGQYHp4O8aECRkMFtj6WU4t7bMoHyzJD2VUqKBEj1yJXlJoxD2aDmJB7bEv791dvRtWtNMa4CkwBmVWmmMrdaqirgKCLr6VTMl7bsmAY4jXgoGl4g&response-X-Checksum-Sha1=32dc0b28e61f32b40c20e2993418aa8cb6e746d5&response-X-Checksum-Sha2=e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880
🍺  /usr/local/Cellar/xz/5.2.4: 92 files, 1MB
==> `brew cleanup` has not been run in 30 days, running now...
Removing: /Users/travis/Library/Caches/Homebrew/boost-1.66.0.high_sierra.bottle.tar.gz... (84.6MB)
Removing: /Users/travis/Library/Caches/Homebrew/carthage-0.28.0.high_sierra.bottle.tar.gz... (8.3MB)
---
Pruned 0 symbolic links and 5 directories from /usr/local
==> Installing dependencies for swig: pcre
==> Installing swig dependency: pcre
==> Downloading https://homebrew.bintray.com/bottles/pcre-8.43.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/03/0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861?__gda__=exp=1554838844~hmac=fd4b16d0276a7565b7a7b3b9e171614a626ec4168ef6032a17dd7e030828b9bf&response-content-disposition=attachment%3Bfilename%3D%22pcre-8.43.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX1_NF-dJKSmWdT3_n8TVMN3MrjIY-w5MMEFMZYYxtodI02-dHRilZu-HorFwgWckLOB1c5GjEB6Y-68CtO9qJU7tOJ-87VjORxvt8kyYiEJp78b0LjbiXjRUvu-JeHFjHHkTa00VM8Sj0w&response-X-Checksum-Sha1=c67d4b99bb245f0ea56b34118dd6325b06a7250c&response-X-Checksum-Sha2=0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861
🍺  /usr/local/Cellar/pcre/8.43: 204 files, 5.5MB
==> Installing swig
==> Downloading https://homebrew.bintray.com/bottles/swig-3.0.12.high_sierra.bottle.tar.gz
==> Downloading https://homebrew.bintray.com/bottles/swig-3.0.12.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/c0/c0e2656fd10d57281280d20ce8bf9a060cf8714f4283dd1dfde383b3688d9ed1?__gda__=exp=1554838847~hmac=c5ff332639cbc2838f3ec5ee68e3f203fa2bbe56946418d84cc9bdc36de3c20d&response-content-disposition=attachment%3Bfilename%3D%22swig-3.0.12.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX18rayXParDQA8Rd13o0VO6ZO5OlgYarQOGFdqdfsceDtoHH5odfwmrnu7_hPn8ucr2qLNdXXXP7iU1GlZNP5xkdkpASdARWxp1hS-QwwW0l9gZTsK-i2r23gkXN5J-kC00CyCNj7TDEFQ&response-X-Checksum-Sha1=db6e6ed21965214d5f9fba1b180517bb2587ef59&response-X-Checksum-Sha2=c0e2656fd10d57281280d20ce8bf9a060cf8714f4283dd1dfde383b3688d9ed1
🍺  /usr/local/Cellar/swig/3.0.12: 755 files, 5.5MB
travis_time:end:01349262:start=1554837731090870000,finish=1554838169996663000,duration=438905793000
travis_fold:end:install
travis_fold:start:before_script.1
---
[00:03:45]       Memory: 8 GB
[00:03:45]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:45]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:45]       SMC Version (system): 2.8f0
[00:03:45]       Serial Number (system): VMwFVoWLEdXS
[00:03:45] 
[00:03:45] hw.ncpu: 4
[00:03:45] hw.byteorder: 1234
[00:03:45] hw.memsize: 8589934592
---
[02:57:54] travis_time:end:stage2-cargo-miri:start=1554848845079977000,finish=1554848845774378000,duration=694401000

[02:57:54] [TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", tool: "cargo-miri", path: "src/tools/miri", mode: ToolRustc, is_optional_tool: true, source_type: Submodule, extra_features: [] } -- 0.725
[02:57:56] [TIMING] Miri { stage: 2, target: "x86_64-apple-darwin" } -- 1.507
The job exceeded the maximum time limit for jobs, and has been terminated.

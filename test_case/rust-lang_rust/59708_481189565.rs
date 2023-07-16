plain
travis_fold:start:worker_info
Worker information
hostname: c7b69a8a-0430-4fdd-8dc5-52b7d2147819@1.worker-custom-2-7846647cfb-d9k8w.macstadium-prod-2
instance: 95ae0ce2-3541-4cff-ba8c-ee93b6da55b2 moar-travis-ci-macos10.13-xcode9.3-1516902186 (via amqp)
startup: 59.412573022s
travis_fold:end:worker_info
Ignoring bigdecimal-1.3.4 because its extensions are not built.  Try: gem pristine bigdecimal --version 1.3.4
---
voltdb
wry
zxing-cpp
==> Downloading https://homebrew.bintray.com/bottles/xz-5.2.4.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/e7/e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880?__gda__=exp=1554794954~hmac=e1303dc9cff03502bb0500f5fd6d953a18d2d2e02a7a80e8d0512bad239ef1ba&response-content-disposition=attachment%3Bfilename%3D%22xz-5.2.4.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX18NkzrA9cp5bfeBRkjkxskgkasFz6KrJ7MeZO9jzfTPMv5fE4NV_9kCAFPdPKYduEGZ63deHbpDs9Pded7eFFCgZUujLZilZQ-vdVO4lwvDb7ire5y_FDLyMpMIBnMuQJIEgjDDSFEFKg&response-X-Checksum-Sha1=32dc0b28e61f32b40c20e2993418aa8cb6e746d5&response-X-Checksum-Sha2=e7be50f4ee00e35887f3957263334eb3baba59e8c061919060f9259351be6880
🍺  /usr/local/Cellar/xz/5.2.4: 92 files, 1MB
==> `brew cleanup` has not been run in 30 days, running now...
Removing: /Users/travis/Library/Caches/Homebrew/boost-1.66.0.high_sierra.bottle.tar.gz... (84.6MB)
Removing: /Users/travis/Library/Caches/Homebrew/carthage-0.28.0.high_sierra.bottle.tar.gz... (8.3MB)
---
Pruned 0 symbolic links and 5 directories from /usr/local
==> Installing dependencies for swig: pcre
==> Installing swig dependency: pcre
==> Downloading https://homebrew.bintray.com/bottles/pcre-8.43.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/03/0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861?__gda__=exp=1554794967~hmac=2af26079751e7cb081b8ec6bad58145d531c29f8a894f7cad945774e8c6947b4&response-content-disposition=attachment%3Bfilename%3D%22pcre-8.43.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX1-25GYG1jTpiBMqQAzyGph4Qrsd2NX7Iy-Te-gehs4VrbMGoSob01ItLPF2UfUVMgDk8YtIhdjNtWY9knLZz-99pCOVk-hH8IOI7qYeybgLMfqo6OTJdLM1orjqdJ_azJEp4NLYYwFf1w&response-X-Checksum-Sha1=c67d4b99bb245f0ea56b34118dd6325b06a7250c&response-X-Checksum-Sha2=0389911a93a88efd4a69b52dea8ecb872fdb55bcfff45d2f7313be5f79730861
🍺  /usr/local/Cellar/pcre/8.43: 204 files, 5.5MB
==> Installing swig
==> Downloading https://homebrew.bintray.com/bottles/swig-3.0.12.high_sierra.bottle.tar.gz
==> Downloading https://homebrew.bintray.com/bottles/swig-3.0.12.high_sierra.bottle.tar.gz
==> Downloading from https://akamai.bintray.com/c0/c0e2656fd10d57281280d20ce8bf9a060cf8714f4283dd1dfde383b3688d9ed1?__gda__=exp=1554794970~hmac=17d4dcd9214d20978c9c630a2be84439edee2b2a26acf4370f01b5e47eb37b3e&response-content-disposition=attachment%3Bfilename%3D%22swig-3.0.12.high_sierra.bottle.tar.gz%22&response-content-type=application%2Fgzip&requestInfo=U2FsdGVkX19FQiM6JRNX3FnwvQ1IrexElgviM2uX_K1JCJ1jUDhV6FBaN1TsNMHDbqPE4mOWPl1YCI622IV5qpV0Lyt5oalEksSfMxvpdCcoUvwJxeQByqF5_xaKsl53FAHn3UykRdyd3mm5Vo9Xnw&response-X-Checksum-Sha1=db6e6ed21965214d5f9fba1b180517bb2587ef59&response-X-Checksum-Sha2=c0e2656fd10d57281280d20ce8bf9a060cf8714f4283dd1dfde383b3688d9ed1
🍺  /usr/local/Cellar/swig/3.0.12: 755 files, 5.5MB
travis_time:end:0180d123:start=1554793851302252000,finish=1554794291983964000,duration=440681712000
travis_fold:end:install
travis_fold:start:before_script.1
---
[00:03:35]       Memory: 8 GB
[00:03:35]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:35]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:35]       SMC Version (system): 2.8f0
[00:03:35]       Serial Number (system): VMtWWh7rwhGH
[00:03:35] 
[00:03:35] hw.ncpu: 4
[00:03:35] hw.byteorder: 1234
[00:03:35] hw.memsize: 8589934592
---

[02:49:47] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", tool: "fabricate", path: "src/tools/rust-installer", mode: ToolBootstrap, is_optional_tool: false, source_type: Submodule, extra_features: [] } -- 62.641
[02:51:08] [TIMING] Docs { stage: 2, host: "x86_64-apple-darwin" } -- 85.531
[02:51:08] Dist docs (aarch64-apple-ios)
The job exceeded the maximum time limit for jobs, and has been terminated.

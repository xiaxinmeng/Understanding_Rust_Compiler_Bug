plain
Removing intermediate container 785ef9e9d3bc
 ---> 0fb7965f0b50
Step 5/10 : RUN npm install es-check -g
 ---> Running in 233e6a72fcdf
/node-v14.4.0-linux-x64/bin/es-check -> /node-v14.4.0-linux-x64/lib/node_modules/es-check/index.js

> spawn-sync@1.0.15 postinstall /node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/spawn-sync
> node postinstall
+ es-check@5.2.1
added 95 packages from 44 contributors in 3.627s
Removing intermediate container 233e6a72fcdf
 ---> 8d0968f7bb21
---
Successfully built a6bec2649292
Successfully tagged rust-ci:latest
Built container sha256:a6bec2649292265266d7066b61ad8e9afaa2facb34a79c5d9b18c6e4cd420e1f
Uploading finished image to https://ci-caches.rust-lang.org/docker/9f2a38e35a8211f9c2c342213b6dabbf1ce1e957c3f9f4a6874af054aa93d446d1c6f8252277cb4118d76ddf7862eec7c972b385df9acf97d8b518b20c0181e6
upload failed: - to s3://rust-lang-ci-sccache2/docker/9f2a38e35a8211f9c2c342213b6dabbf1ce1e957c3f9f4a6874af054aa93d446d1c6f8252277cb4118d76ddf7862eec7c972b385df9acf97d8b518b20c0181e6 Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
Cloning into 'rust-toolstate'...
<Nothing changed>
+ es-check es5 ../src/librustdoc/html/static/main.js ../src/librustdoc/html/static/settings.js ../src/librustdoc/html/static/source-script.js ../src/librustdoc/html/static/storage.js

Cannot read property 'includes' of undefined

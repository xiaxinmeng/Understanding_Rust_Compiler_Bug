plain
Step 3/10 : RUN curl -sL https://nodejs.org/dist/v14.4.0/node-v14.4.0-linux-x64.tar.xz | tar -xJ
 ---> Running in 827c1ae0c4f8
Removing intermediate container 827c1ae0c4f8
 ---> 1718e9d7f955
Step 4/10 : ENV PATH="/node-v14.4.0-linux-x64/bin:${PATH}"
Removing intermediate container cc4f63777fff
 ---> 96abb0cd5c9e
 ---> 96abb0cd5c9e
Step 5/10 : RUN npm install es-check -g
 ---> Running in 97bebed46c72
/node-v14.4.0-linux-x64/bin/es-check -> /node-v14.4.0-linux-x64/lib/node_modules/es-check/index.js

> spawn-sync@1.0.15 postinstall /node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/spawn-sync
> node postinstall

+ es-check@5.2.0
added 95 packages from 44 contributors in 4.817s
 ---> 10d89e5f8d12
Step 6/10 : COPY scripts/sccache.sh /scripts/
 ---> b3ac6c642dde
Step 7/10 : RUN sh /scripts/sccache.sh
---
Step 9/10 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Running in 48dd01e59f1c
Removing intermediate container 48dd01e59f1c
 ---> 1c532f56e792
Step 10/10 : ENV SCRIPT python3 ../x.py --stage 2 test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test --stage 2 src/tools/tidy &&            python3 ../x.py doc --stage 0 library/std &&            /scripts/validate-toolstate.sh &&            es-check es5 ../src/librustdoc/html/static/*.js
Removing intermediate container 5fad28be588c
 ---> 8b4021b5f897
Successfully built 8b4021b5f897
Successfully tagged rust-ci:latest
Successfully tagged rust-ci:latest
Built container sha256:8b4021b5f897d87ba96e71c0b6cd28de54846767cb31152fc19d6a09a8fae290
Uploading finished image to https://ci-caches.rust-lang.org/docker/acc7ba3cbf029aab335712a84f663acc46a82f9b19653908a00b5480c746a3294801c6eaf57e269e9434eaf22aad72efad56f3f8c9b3898334b28b87caa8fea1
upload failed: - to s3://rust-lang-ci-sccache2/docker/acc7ba3cbf029aab335712a84f663acc46a82f9b19653908a00b5480c746a3294801c6eaf57e269e9434eaf22aad72efad56f3f8c9b3898334b28b87caa8fea1 Unable to locate credentials
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
Build completed successfully in 0:00:51
+ /scripts/validate-toolstate.sh
Cloning into 'rust-toolstate'...
<Nothing changed>
+ es-check es5 ../src/librustdoc/html/static/main.js ../src/librustdoc/html/static/settings.js ../src/librustdoc/html/static/source-script.js ../src/librustdoc/html/static/storage.js
ES-Check: there were 2 ES version matching errors.
          ES-Check Error:
          ----
          · erroring file: ../src/librustdoc/html/static/main.js
          · erroring file: ../src/librustdoc/html/static/main.js
          · error: SyntaxError: Unexpected token (2911:17)
          · see the printed err.stack below for context


          SyntaxError: Unexpected token (2911:17)
    at Parser.pp$4.raise (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2825:15)
    at Parser.pp.unexpected (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:689:10)
    at Parser.pp$3.parseExprAtom (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2270:12)
    at Parser.pp$3.parseExprSubscripts (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2089:21)
    at Parser.pp$3.parseMaybeUnary (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2066:19)
    at Parser.pp$3.parseExprOps (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2010:21)
    at Parser.pp$3.parseMaybeConditional (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1993:21)
    at Parser.pp$3.parseMaybeAssign (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1968:21)
    at Parser.pp$3.parseMaybeAssign (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1978:25)
    at Parser.pp$3.parseExprList (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2728:20)
    at Parser.pp$3.parseSubscript (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2124:27)
    at Parser.pp$3.parseSubscripts (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2104:26)
    at Parser.pp$3.parseExprSubscripts (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2092:23)
    at Parser.pp$3.parseMaybeUnary (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2066:19)
    at Parser.pp$3.parseExprOps (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2010:21)
    at Parser.pp$3.parseMaybeConditional (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1993:21)

          ES-Check Error:
          ----
          · erroring file: ../src/librustdoc/html/static/storage.js
          · erroring file: ../src/librustdoc/html/static/storage.js
          · error: SyntaxError: Unexpected token (160:16)
          · see the printed err.stack below for context


          SyntaxError: Unexpected token (160:16)
    at Parser.pp$4.raise (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2825:15)
    at Parser.pp.unexpected (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:689:10)
    at Parser.pp.semicolon (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:666:66)
    at Parser.pp$1.parseExpressionStatement (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1145:10)
    at Parser.pp$1.parseStatement (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:880:26)
    at Parser.pp$1.parseBlock (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1161:23)
    at Parser.pp$3.parseFunctionBody (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2671:24)
    at Parser.pp$1.parseFunction (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1283:10)
    at Parser.pp$3.parseExprAtom (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2251:19)
    at Parser.pp$3.parseExprSubscripts (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2089:21)
    at Parser.pp$3.parseMaybeUnary (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2066:19)
    at Parser.pp$3.parseExprOps (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2010:21)
    at Parser.pp$3.parseMaybeConditional (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1993:21)
    at Parser.pp$3.parseMaybeAssign (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1968:21)
    at Parser.pp$3.parseExpression (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1933:21)
    at Parser.pp$1.parseReturnStatement (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1005:33)

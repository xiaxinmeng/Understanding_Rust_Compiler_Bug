plain
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
ES-Check: there were 1 ES version matching errors.

          ES-Check Error:
          ----
          · erroring file: ../src/librustdoc/html/static/js/search.js
          · error: SyntaxError: Unexpected token (422:24)
          · see the printed err.stack below for context


          SyntaxError: Unexpected token (422:24)
    at Parser.pp$4.raise (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2825:15)
    at Parser.pp.unexpected (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:689:10)
    at Parser.pp$3.parseExprAtom (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2270:12)
    at Parser.pp$3.parseExprSubscripts (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2089:21)
    at Parser.pp$3.parseMaybeUnary (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2066:19)
    at Parser.pp$3.parseExprOps (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2010:21)
    at Parser.pp$3.parseMaybeConditional (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1993:21)
    at Parser.pp$3.parseMaybeAssign (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1968:21)
    at Parser.pp$3.parseExprList (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2728:20)
    at Parser.pp$3.parseSubscript (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2124:27)
    at Parser.pp$3.parseSubscripts (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2104:26)
    at Parser.pp$3.parseExprSubscripts (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2092:23)
    at Parser.pp$3.parseMaybeUnary (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2066:19)
    at Parser.pp$3.parseExprOps (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2010:21)
    at Parser.pp$3.parseMaybeConditional (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1993:21)
    at Parser.pp$3.parseMaybeAssign (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1968:21)

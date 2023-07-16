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
          · erroring file: ../src/librustdoc/html/static/js/main.js
          · error: SyntaxError: The keyword 'let' is reserved (1115:8)
          · see the printed err.stack below for context


          SyntaxError: The keyword 'let' is reserved (1115:8)
    at Parser.pp$4.raise (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2825:15)
    at Parser.pp$3.checkUnreserved (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2752:12)
    at Parser.pp$3.parseIdent (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2781:12)
    at Parser.pp$3.parseExprAtom (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2198:21)
    at Parser.pp$3.parseExprSubscripts (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2089:21)
    at Parser.pp$3.parseMaybeUnary (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2066:19)
    at Parser.pp$3.parseExprOps (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2010:21)
    at Parser.pp$3.parseMaybeConditional (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1993:21)
    at Parser.pp$3.parseMaybeAssign (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1968:21)
    at Parser.pp$3.parseExpression (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1933:21)
    at Parser.pp$1.parseStatement (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:877:47)
    at Parser.pp$1.parseBlock (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1161:23)
    at Parser.pp$3.parseFunctionBody (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2671:24)
    at Parser.pp$1.parseFunction (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1283:10)
    at Parser.pp$1.parseFunctionStatement (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:983:17)
    at Parser.pp$1.parseStatement (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:830:19)

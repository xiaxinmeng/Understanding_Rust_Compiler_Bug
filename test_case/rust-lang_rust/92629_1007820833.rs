plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
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
          · erroring file: ../src/librustdoc/html/static/js/storage.js
          · error: SyntaxError: Unexpected token (217:12)
          · see the printed err.stack below for context


          SyntaxError: Unexpected token (217:12)
    at Parser.pp$4.raise (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2825:15)
    at Parser.pp.unexpected (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:689:10)
    at Parser.pp.semicolon (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:666:66)
    at Parser.pp$1.parseExpressionStatement (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1145:10)
    at Parser.pp$1.parseStatement (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:880:26)
    at Parser.pp$1.parseBlock (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1161:23)
    at Parser.pp$3.parseFunctionBody (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2671:24)
    at Parser.pp$1.parseFunction (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1283:10)
    at Parser.pp$1.parseFunctionStatement (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:983:17)
    at Parser.pp$1.parseStatement (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:830:19)
    at Parser.pp$1.parseBlock (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1161:23)
    at Parser.pp$3.parseFunctionBody (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2671:24)
    at Parser.pp$1.parseFunction (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1283:10)
    at Parser.pp$3.parseExprAtom (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2251:19)
    at Parser.pp$3.parseExprSubscripts (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2089:21)
    at Parser.pp$3.parseMaybeUnary (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2066:19)

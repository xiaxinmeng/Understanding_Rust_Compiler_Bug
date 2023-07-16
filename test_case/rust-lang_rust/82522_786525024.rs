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
          · erroring file: ../src/librustdoc/html/static/main.js
          · error: SyntaxError: Unexpected character '`' (2916:20)
          · see the printed err.stack below for context


          SyntaxError: Unexpected character '`' (2916:20)
    at Parser.pp$4.raise (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2825:15)
    at Parser.pp$9.getTokenFromCode (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:4560:10)
    at Parser.pp$9.readToken (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:4288:17)
    at Parser.pp$9.nextToken (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:4279:17)
    at Parser.pp$9.next (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:4236:10)
    at Parser.pp.eat (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:619:12)
    at Parser.pp.expect (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:683:10)
    at Parser.pp$3.parseExprList (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2716:14)
    at Parser.pp$3.parseExprAtom (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2242:28)
    at Parser.pp$3.parseExprSubscripts (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2089:21)
    at Parser.pp$3.parseMaybeUnary (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2066:19)
    at Parser.pp$3.parseExprOps (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2010:21)
    at Parser.pp$3.parseMaybeConditional (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1993:21)
    at Parser.pp$3.parseMaybeAssign (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:1968:21)
    at Parser.pp$3.parseExprList (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2728:20)
    at Parser.pp$3.parseExprAtom (/node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/acorn/dist/acorn.js:2242:28)

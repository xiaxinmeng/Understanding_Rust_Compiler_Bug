plain
2020-03-09T11:03:57.7990160Z test [ui] ui/hygiene/unpretty-debug.rs ... ok
2020-03-09T11:03:57.8526980Z test [ui] ui/hygiene/specialization.rs ... ok
2020-03-09T11:03:57.9265080Z test [ui] ui/if-attrs/bad-cfg.rs ... ok
2020-03-09T11:03:57.9418350Z test [ui] ui/hygiene/transparent-basic.rs ... ok
2020-03-09T11:03:58.0300960Z test [ui] ui/if-attrs/builtin-if-attr.rs ... ok
2020-03-09T11:03:58.0403520Z test [ui] ui/if-attrs/cfg-false-if-attr.rs ... ok
2020-03-09T11:03:58.0958720Z test [ui] ui/if-attrs/else-attrs.rs ... ok
2020-03-09T11:03:58.1782390Z test [ui] ui/if-attrs/let-chains-attr.rs ... ok
2020-03-09T11:03:58.2576450Z test [ui] ui/if-attrs/stmt-expr-gated.rs ... ok
2020-03-09T11:03:58.2779270Z test [ui] ui/hygiene/wrap_unhygienic_example.rs ... ok
2020-03-09T11:03:58.4504060Z test [ui] ui/if-attrs/gate-whole-expr.rs ... ok
2020-03-09T11:03:58.6051760Z test [ui] ui/if-bot.rs ... ok
2020-03-09T11:03:58.6532310Z test [ui] ui/if-else-type-mismatch.rs ... ok
2020-03-09T11:03:58.7072800Z test [ui] ui/if-check.rs ... ok
2020-03-09T11:03:58.7226600Z test [ui] ui/if/if-branch-types.rs ... ok
---
2020-03-09T11:09:42.7898220Z test [ui] ui/sanitize/cfg.rs#thread ... ignored
2020-03-09T11:09:42.8021510Z test [ui] ui/safe-extern-statics.rs ... ok
2020-03-09T11:09:42.8123930Z test [ui] ui/sanitize/memory.rs ... ignored
2020-03-09T11:09:44.2133240Z test [ui] ui/sanitize/address.rs ... ok
2020-03-09T11:09:44.2843640Z test [ui] ui/sanitize/badfree.rs ... ok
2020-03-09T11:09:44.3312710Z test [ui] ui/sanitize/unsupported-target.rs ... ok
2020-03-09T11:09:44.4972050Z test [ui] ui/save-analysis/emit-notifications.rs ... ok
2020-03-09T11:09:44.5876400Z test [ui] ui/save-analysis/issue-59134-0.rs ... ok
2020-03-09T11:09:44.6679360Z test [ui] ui/save-analysis/issue-59134-1.rs ... ok
---
2020-03-09T11:22:02.5036600Z test [pretty] pretty/fn-variadic.rs ... ok
2020-03-09T11:22:02.5142620Z test [pretty] pretty/fn-types.rs ... ok
2020-03-09T11:22:02.6052330Z test [pretty] pretty/gat-bounds.rs ... ok
2020-03-09T11:22:02.6254810Z test [pretty] pretty/for-comment.rs ... ok
2020-03-09T11:22:02.6657920Z test [pretty] pretty/if-attr.rs ... ok
2020-03-09T11:22:02.7279420Z test [pretty] pretty/issue-12590-c.rs ... ok
2020-03-09T11:22:02.7482280Z test [pretty] pretty/issue-12590-a.rs ... ok
2020-03-09T11:22:02.7986310Z test [pretty] pretty/issue-30731.rs ... ok
2020-03-09T11:22:02.8290310Z test [pretty] pretty/issue-19077.rs ... ok
---
2020-03-09T11:50:15.3863420Z test result: ok. 1 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out
2020-03-09T11:50:15.3863690Z 
2020-03-09T11:50:15.3863880Z  finished in 4.230
2020-03-09T11:50:15.3864880Z [TIMING] BookTest { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" }, path: "src/doc/rustc", name: "rustc", is_ext_doc: false } -- 4.230
2020-03-09T11:50:15.5299910Z x86_64-apple-darwin
2020-03-09T11:50:15.7063620Z tmp.js:9
2020-03-09T11:50:15.7067740Z var itemTypes=["mod","externcrate","import","struct","enum","fn","type","static","trait","impl","tymethod","method","structfield","variant","macro","primitive","associatedtype","constant","associatedconstant","union","foreigntype","keyword","existential","attr","derive","traitalias"];exports.itemTypes = itemTypes;var MAX_LEV_DISTANCE=3;exports.MAX_LEV_DISTANCE = MAX_LEV_DISTANCE;var MAX_RESULTS=200;exports.MAX_RESULTS = MAX_RESULTS;var NO_TYPE_FILTER=-1;exports.NO_TYPE_FILTER = NO_TYPE_FILTER;var GENERICS_DATA=1;exports.GENERICS_DATA = GENERICS_DATA;var NAME=0;exports.NAME = NAME;var INPUTS_DATA=0;exports.INPUTS_DATA = INPUTS_DATA;var OUTPUT_DATA=1;exports.OUTPUT_DATA = OUTPUT_DATA;var TY_PRIMITIVE=itemTypes.indexOf("primitive");exports.TY_PRIMITIVE = TY_PRIMITIVE;var TY_KEYWORD=itemTypes.indexOf("keyword");exports.TY_KEYWORD = TY_KEYWORD;var levenshtein_row2=[];exports.levenshtein_row2 = levenshtein_row2;function buildHrefAndPath(item){var displayPath;var href;var type=itemTypes[item.ty];var name=
2020-03-09T11:50:15.7070180Z ReferenceError: r is not defined
2020-03-09T11:50:15.7070180Z ReferenceError: r is not defined
2020-03-09T11:50:15.7070530Z     at checkReturned (tmp.js:9:10009)
2020-03-09T11:50:15.7070850Z     at execQuery (tmp.js:9:14562)
2020-03-09T11:50:15.7071170Z     at Object.execSearch (tmp.js:9:17052)
2020-03-09T11:50:15.7072090Z     at /Users/runner/runners/2.165.0/work/1/s/src/tools/rustdoc-js-std/tester.js:300:30
2020-03-09T11:50:15.7072500Z     at Array.forEach (<anonymous>)
2020-03-09T11:50:15.7073330Z     at main (/Users/runner/runners/2.165.0/work/1/s/src/tools/rustdoc-js-std/tester.js:291:33)
2020-03-09T11:50:15.7074320Z     at Object.<anonymous> (/Users/runner/runners/2.165.0/work/1/s/src/tools/rustdoc-js-std/tester.js:345:14)
2020-03-09T11:50:15.7074830Z     at Module._compile (internal/modules/cjs/loader.js:1158:30)
2020-03-09T11:50:15.7075280Z     at Object.Module._extensions..js (internal/modules/cjs/loader.js:1178:10)
2020-03-09T11:50:15.7075710Z     at Module.load (internal/modules/cjs/loader.js:1002:32)
2020-03-09T11:50:15.7131610Z 
2020-03-09T11:50:15.7131610Z 
2020-03-09T11:50:15.7132860Z command did not execute successfully: "/usr/local/bin/node" "src/tools/rustdoc-js-std/tester.js" "x86_64-apple-darwin"
2020-03-09T11:50:15.7133580Z 
2020-03-09T11:50:15.7134300Z 
2020-03-09T11:50:15.7149010Z failed to run: /Users/runner/runners/2.165.0/work/1/s/build/bootstrap/debug/bootstrap test
2020-03-09T11:50:15.7149500Z Build completed unsuccessfully in 1:35:15
2020-03-09T11:50:15.7149500Z Build completed unsuccessfully in 1:35:15
2020-03-09T11:50:15.7210740Z == clock drift check ==
2020-03-09T11:50:15.7277540Z   local time: Mon Mar  9 11:50:15 UTC 2020
2020-03-09T11:50:15.7704470Z   network time: Mon, 09 Mar 2020 11:50:15 GMT
2020-03-09T11:50:15.7706290Z == end clock drift check ==
2020-03-09T11:50:15.7753780Z 
2020-03-09T11:50:15.7838290Z ##[error]Bash exited with code '1'.
2020-03-09T11:50:15.7926990Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-09T11:50:15.7935300Z ==============================================================================
2020-03-09T11:50:15.7935700Z Task         : Get sources
2020-03-09T11:50:15.7936100Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

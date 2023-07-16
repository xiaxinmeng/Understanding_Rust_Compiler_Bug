
2020-07-16T22:46:45.0248894Z [0m[0m[1m[32m    Finished[0m release [optimized] target(s) in 25.05s
2020-07-16T22:46:45.0355607Z [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "tidy", path: "src/tools/tidy", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 25.065
2020-07-16T22:46:45.0357013Z tidy check
2020-07-16T22:46:45.1959111Z tidy error: /checkout/src/librustdoc/html/static/rustdoc.css:1128: CSS files use tabs for indent
2020-07-16T22:46:45.1959986Z tidy error: /checkout/src/librustdoc/html/static/rustdoc.css:1136: CSS files use tabs for indent
2020-07-16T22:46:45.1960232Z tidy error: /checkout/src/librustdoc/html/static/rustdoc.css:1137: CSS files use tabs for indent
2020-07-16T22:46:45.1960769Z tidy error: /checkout/src/librustdoc/html/static/rustdoc.css:1138: CSS files use tabs for indent
2020-07-16T22:46:45.1961134Z tidy error: /checkout/src/librustdoc/html/static/rustdoc.css:1142: CSS files use tabs for indent
2020-07-16T22:46:45.1973850Z tidy error: /checkout/src/librustdoc/html/static/themes/ayu.css:394: CSS files use tabs for indent
2020-07-16T22:46:45.1975089Z tidy error: /checkout/src/librustdoc/html/static/themes/ayu.css:395: CSS files use tabs for indent
2020-07-16T22:46:49.3754271Z Checking which error codes lack tests...
2020-07-16T22:46:49.5504782Z some tidy checks failed

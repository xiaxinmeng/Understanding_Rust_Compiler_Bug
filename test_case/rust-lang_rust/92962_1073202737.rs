plain
.......... (60/61)
          (61/61)


/checkout/src/test/rustdoc-gui/escape-key.goml escape-key... FAILED
[ERROR] (line 6) TimeoutError: waiting for selector "#search h1" failed: timeout 30000ms exceeded: for command `wait-for: "#search h1" // The search element is empty before the first search `
[ERROR] (line 7) Error: Evaluation failed: expected `content` for attribute `class` for selector `#search`, found `content hidden`: for command `assert-attribute: ("#search", {"class": "content"})`
[ERROR] (line 8) Error: Evaluation failed: expected `content hidden` for attribute `class` for selector `#main-content`, found `content`: for command `assert-attribute: ("#main-content", {"class": "content hidden"})`
[ERROR] (line 9) Error: Evaluation failed: Property `URL` (`file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/test_docs/index.html`) does not end with `index.html?search=test`: for command `assert-document-property: ({"URL": "index.html?search=test"}, ENDS_WITH)`
[ERROR] (line 17) Error: Evaluation failed: expected `content` for attribute `class` for selector `#search`, found `content hidden`: for command `assert-attribute: ("#search", {"class": "content"})`
[ERROR] (line 18) Error: Evaluation failed: expected `content hidden` for attribute `class` for selector `#main-content`, found `content`: for command `assert-attribute: ("#main-content", {"class": "content hidden"})`
[ERROR] (line 19) Error: Evaluation failed: Property `URL` (`file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/test_docs/index.html`) does not end with `index.html?search=test`: for command `assert-document-property: ({"URL": "index.html?search=test"}, ENDS_WITH)`
[ERROR] (line 24) Error: Evaluation failed: Property `URL` (`file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/test_docs/index.html`) does not end with `index.html?search=test`: for command `assert-document-property: ({"URL": "index.html?search=test"}, [ENDS_WITH])`
[ERROR] (line 28) Error: Evaluation failed: expected `content` for attribute `class` for selector `#search`, found `content hidden`: for command `assert-attribute: ("#search", {"class": "content"})`
[ERROR] (line 29) Error: Evaluation failed: expected `content hidden` for attribute `class` for selector `#main-content`, found `content`: for command `assert-attribute: ("#main-content", {"class": "content hidden"})`
[ERROR] (line 30) Error: Evaluation failed: Property `URL` (`file:///checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc/test_docs/index.html`) does not end with `index.html?search=test`: for command `assert-document-property: ({"URL": "index.html?search=test"}, [ENDS_WITH])`
[ERROR] (line 36) assert didn't fail: for command `assert-false: ".search-input:focus"`
[ERROR] (line 37) "#results a:focus" not found: for command `assert: "#results a:focus"`
Build completed unsuccessfully in 0:00:45

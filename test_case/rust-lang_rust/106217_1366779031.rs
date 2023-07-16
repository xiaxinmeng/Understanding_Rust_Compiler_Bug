plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 6a20f7df5755d8c6b68110d2d0391a7b03268e77 and 07d83640dbaa1fe180806b008d1ba26d7f429f68
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (90/96)
......     (96/96)


/checkout/src/test/rustdoc-gui/codeblock-tooltip.goml codeblock-tooltip... FAILED
[ERROR] (line 25 from 135) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.compile_fail .tooltip`): [expected `center` for key `text-align`, found `start`]: for command `assert-css: (
            ".docblock .example-wrap.compile_fail .tooltip::after",
            {
                "content": '"This example deliberately fails to compile"',
                "text-align": "center",
                "padding": "5px 3px 3px",
                "background-color": |background|,
                "color": |color|,
                "border": "1px solid " + |border|,
        )`
        )`
[ERROR] (line 65 from 135) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.should_panic .tooltip`): [expected `center` for key `text-align`, found `start`]: for command `assert-css: (
            ".docblock .example-wrap.should_panic .tooltip::after",
            {
                "content": '"This example panics"',
                "text-align": "center",
                "padding": "5px 3px 3px",
                "background-color": |background|,
                "color": |color|,
                "border": "1px solid " + |border|,
        )`
        )`
[ERROR] (line 105 from 135) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.ignore .tooltip`): [expected `center` for key `text-align`, found `start`]: for command `assert-css: (
            ".docblock .example-wrap.ignore .tooltip::after",
            {
                "content": '"This example is not tested"',
                "text-align": "center",
                "padding": "5px 3px 3px",
                "background-color": |background|,
                "color": |color|,
                "border": "1px solid " + |border|,
        )`
        )`
[ERROR] (line 25 from 141) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.compile_fail .tooltip`): [expected `center` for key `text-align`, found `start`]: for command `assert-css: (
            ".docblock .example-wrap.compile_fail .tooltip::after",
            {
                "content": '"This example deliberately fails to compile"',
                "text-align": "center",
                "padding": "5px 3px 3px",
                "background-color": |background|,
                "color": |color|,
                "border": "1px solid " + |border|,
        )`
        )`
[ERROR] (line 65 from 141) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.should_panic .tooltip`): [expected `center` for key `text-align`, found `start`]: for command `assert-css: (
            ".docblock .example-wrap.should_panic .tooltip::after",
            {
                "content": '"This example panics"',
                "text-align": "center",
                "padding": "5px 3px 3px",
                "background-color": |background|,
                "color": |color|,
                "border": "1px solid " + |border|,
        )`
        )`
[ERROR] (line 105 from 141) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.ignore .tooltip`): [expected `center` for key `text-align`, found `start`]: for command `assert-css: (
            ".docblock .example-wrap.ignore .tooltip::after",
            {
                "content": '"This example is not tested"',
                "text-align": "center",
                "padding": "5px 3px 3px",
                "background-color": |background|,
                "color": |color|,
                "border": "1px solid " + |border|,
        )`
        )`
[ERROR] (line 25 from 147) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.compile_fail .tooltip`): [expected `center` for key `text-align`, found `start`]: for command `assert-css: (
            ".docblock .example-wrap.compile_fail .tooltip::after",
            {
                "content": '"This example deliberately fails to compile"',
                "text-align": "center",
                "padding": "5px 3px 3px",
                "background-color": |background|,
                "color": |color|,
                "border": "1px solid " + |border|,
        )`
        )`
[ERROR] (line 65 from 147) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.should_panic .tooltip`): [expected `center` for key `text-align`, found `start`]: for command `assert-css: (
            ".docblock .example-wrap.should_panic .tooltip::after",
            {
                "content": '"This example panics"',
                "text-align": "center",
                "padding": "5px 3px 3px",
                "background-color": |background|,
                "color": |color|,
                "border": "1px solid " + |border|,
        )`
        )`
[ERROR] (line 105 from 147) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.ignore .tooltip`): [expected `center` for key `text-align`, found `start`]: for command `assert-css: (
            ".docblock .example-wrap.ignore .tooltip::after",
            {
                "content": '"This example is not tested"',
                "text-align": "center",
                "padding": "5px 3px 3px",
                "background-color": |background|,
                "color": |color|,
                "border": "1px solid " + |border|,
        )`

Build completed unsuccessfully in 0:02:34

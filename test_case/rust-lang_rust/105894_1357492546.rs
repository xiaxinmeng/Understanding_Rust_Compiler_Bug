plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 10723378900ba2d25fc5d8baf785e1082f385832 and d8d46ab217aff005b1ce7c515f82e86d75a5476e
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (90/93)
...        (93/93)


/checkout/src/test/rustdoc-gui/codeblock-tooltip.goml codeblock-tooltip... FAILED
[ERROR] (line 17 from 135) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.compile_fail .tooltip`): [expected `rgb(255, 0, 0)` for key `color`, found `rgba(255, 0, 0, 0.5)`]: for command `assert-css: (
            ".docblock .example-wrap.compile_fail .tooltip",
            {"color": "rgb(255, 0, 0)"},
        )`
[ERROR] (line 21 from 135) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.compile_fail`): [expected `2px solid rgb(255, 0, 0)` for key `border-left`, found `2px solid rgba(255, 0, 0, 0.5)`]: for command `assert-css: (
            ".docblock .example-wrap.compile_fail",
            {"border-left": "2px solid rgb(255, 0, 0)"},
        )`
[ERROR] (line 25 from 135) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.compile_fail .tooltip`): [expected `"This example deliberately fails to compile"` for key `content`, found `none`, expected `center` for key `text-align`, found `start`, expected `5px 3px 3px` for key `padding`, found `0px`, expected `rgb(49, 69, 89)` for key `background-color`, found `rgba(0, 0, 0, 0)`, expected `rgb(197, 197, 197)` for key `color`, found `rgba(255, 0, 0, 0.5)`, expected `1px solid rgb(92, 103, 115)` for key `border`, found `0px none rgba(255, 0, 0, 0.5)`]: for command `assert-css: (
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
[ERROR] (line 36 from 135) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.compile_fail .tooltip`): [expected `5px` for key `border-width`, found `0px`, expected `solid` for key `border-style`, found `none`, expected `rgba(0, 0, 0, 0) rgb(49, 69, 89) rgba(0, 0, 0, 0) rgba(0, 0, 0, 0)` for key `border-color`, found `rgba(255, 0, 0, 0.5)`]: for command `assert-css: (
            ".docblock .example-wrap.compile_fail .tooltip::before",
            {
                "border-width": "5px",
                "border-style": "solid",
                "border-color": "rgba(0, 0, 0, 0) " + |background| + " rgba(0, 0, 0, 0) rgba(0, 0, 0, 0)",
        )`
        )`
[ERROR] (line 57 from 135) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.should_panic .tooltip`): [expected `rgb(255, 0, 0)` for key `color`, found `rgba(255, 0, 0, 0.5)`]: for command `assert-css: (
            ".docblock .example-wrap.should_panic .tooltip",
            {"color": "rgb(255, 0, 0)"},
        )`
[ERROR] (line 61 from 135) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.should_panic`): [expected `2px solid rgb(255, 0, 0)` for key `border-left`, found `2px solid rgba(255, 0, 0, 0.5)`]: for command `assert-css: (
            ".docblock .example-wrap.should_panic",
            {"border-left": "2px solid rgb(255, 0, 0)"},
        )`
[ERROR] (line 65 from 135) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.should_panic .tooltip`): [expected `"This example panics"` for key `content`, found `none`, expected `center` for key `text-align`, found `start`, expected `5px 3px 3px` for key `padding`, found `0px`, expected `rgb(49, 69, 89)` for key `background-color`, found `rgba(0, 0, 0, 0)`, expected `rgb(197, 197, 197)` for key `color`, found `rgba(255, 0, 0, 0.5)`, expected `1px solid rgb(92, 103, 115)` for key `border`, found `0px none rgba(255, 0, 0, 0.5)`]: for command `assert-css: (
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
[ERROR] (line 76 from 135) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.should_panic .tooltip`): [expected `5px` for key `border-width`, found `0px`, expected `solid` for key `border-style`, found `none`, expected `rgba(0, 0, 0, 0) rgb(49, 69, 89) rgba(0, 0, 0, 0) rgba(0, 0, 0, 0)` for key `border-color`, found `rgba(255, 0, 0, 0.5)`]: for command `assert-css: (
            ".docblock .example-wrap.should_panic .tooltip::before",
            {
                "border-width": "5px",
                "border-style": "solid",
                "border-color": "rgba(0, 0, 0, 0) " + |background| + " rgba(0, 0, 0, 0) rgba(0, 0, 0, 0)",
        )`
        )`
[ERROR] (line 97 from 135) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.ignore .tooltip`): [expected `rgb(255, 142, 0)` for key `color`, found `rgba(255, 142, 0, 0.6)`]: for command `assert-css: (
            ".docblock .example-wrap.ignore .tooltip",
            {"color": "rgb(255, 142, 0)"},
        )`
[ERROR] (line 101 from 135) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.ignore`): [expected `2px solid rgb(255, 142, 0)` for key `border-left`, found `2px solid rgba(255, 142, 0, 0.6)`]: for command `assert-css: (
            ".docblock .example-wrap.ignore",
            {"border-left": "2px solid rgb(255, 142, 0)"},
        )`
[ERROR] (line 105 from 135) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.ignore .tooltip`): [expected `"This example is not tested"` for key `content`, found `none`, expected `center` for key `text-align`, found `start`, expected `5px 3px 3px` for key `padding`, found `0px`, expected `rgb(49, 69, 89)` for key `background-color`, found `rgba(0, 0, 0, 0)`, expected `rgb(197, 197, 197)` for key `color`, found `rgba(255, 142, 0, 0.6)`, expected `1px solid rgb(92, 103, 115)` for key `border`, found `0px none rgba(255, 142, 0, 0.6)`]: for command `assert-css: (
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
[ERROR] (line 116 from 135) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.ignore .tooltip`): [expected `5px` for key `border-width`, found `0px`, expected `solid` for key `border-style`, found `none`, expected `rgba(0, 0, 0, 0) rgb(49, 69, 89) rgba(0, 0, 0, 0) rgba(0, 0, 0, 0)` for key `border-color`, found `rgba(255, 142, 0, 0.6)`]: for command `assert-css: (
            ".docblock .example-wrap.ignore .tooltip::before",
            {
                "border-width": "5px",
                "border-style": "solid",
                "border-color": "rgba(0, 0, 0, 0) " + |background| + " rgba(0, 0, 0, 0) rgba(0, 0, 0, 0)",
        )`
        )`
[ERROR] (line 17 from 141) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.compile_fail .tooltip`): [expected `rgb(255, 0, 0)` for key `color`, found `rgba(255, 0, 0, 0.5)`]: for command `assert-css: (
            ".docblock .example-wrap.compile_fail .tooltip",
            {"color": "rgb(255, 0, 0)"},
        )`
[ERROR] (line 21 from 141) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.compile_fail`): [expected `2px solid rgb(255, 0, 0)` for key `border-left`, found `2px solid rgba(255, 0, 0, 0.5)`]: for command `assert-css: (
            ".docblock .example-wrap.compile_fail",
            {"border-left": "2px solid rgb(255, 0, 0)"},
        )`
[ERROR] (line 25 from 141) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.compile_fail .tooltip`): [expected `"This example deliberately fails to compile"` for key `content`, found `none`, expected `center` for key `text-align`, found `start`, expected `5px 3px 3px` for key `padding`, found `0px`, expected `rgb(0, 0, 0)` for key `background-color`, found `rgba(0, 0, 0, 0)`, expected `rgb(255, 255, 255)` for key `color`, found `rgba(255, 0, 0, 0.5)`, expected `1px solid rgb(224, 224, 224)` for key `border`, found `0px none rgba(255, 0, 0, 0.5)`]: for command `assert-css: (
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
[ERROR] (line 36 from 141) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.compile_fail .tooltip`): [expected `5px` for key `border-width`, found `0px`, expected `solid` for key `border-style`, found `none`, expected `rgba(0, 0, 0, 0) rgb(0, 0, 0) rgba(0, 0, 0, 0) rgba(0, 0, 0, 0)` for key `border-color`, found `rgba(255, 0, 0, 0.5)`]: for command `assert-css: (
            ".docblock .example-wrap.compile_fail .tooltip::before",
            {
                "border-width": "5px",
                "border-style": "solid",
                "border-color": "rgba(0, 0, 0, 0) " + |background| + " rgba(0, 0, 0, 0) rgba(0, 0, 0, 0)",
        )`
        )`
[ERROR] (line 57 from 141) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.should_panic .tooltip`): [expected `rgb(255, 0, 0)` for key `color`, found `rgba(255, 0, 0, 0.5)`]: for command `assert-css: (
            ".docblock .example-wrap.should_panic .tooltip",
            {"color": "rgb(255, 0, 0)"},
        )`
[ERROR] (line 61 from 141) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.should_panic`): [expected `2px solid rgb(255, 0, 0)` for key `border-left`, found `2px solid rgba(255, 0, 0, 0.5)`]: for command `assert-css: (
            ".docblock .example-wrap.should_panic",
            {"border-left": "2px solid rgb(255, 0, 0)"},
        )`
[ERROR] (line 65 from 141) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.should_panic .tooltip`): [expected `"This example panics"` for key `content`, found `none`, expected `center` for key `text-align`, found `start`, expected `5px 3px 3px` for key `padding`, found `0px`, expected `rgb(0, 0, 0)` for key `background-color`, found `rgba(0, 0, 0, 0)`, expected `rgb(255, 255, 255)` for key `color`, found `rgba(255, 0, 0, 0.5)`, expected `1px solid rgb(224, 224, 224)` for key `border`, found `0px none rgba(255, 0, 0, 0.5)`]: for command `assert-css: (
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
[ERROR] (line 76 from 141) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.should_panic .tooltip`): [expected `5px` for key `border-width`, found `0px`, expected `solid` for key `border-style`, found `none`, expected `rgba(0, 0, 0, 0) rgb(0, 0, 0) rgba(0, 0, 0, 0) rgba(0, 0, 0, 0)` for key `border-color`, found `rgba(255, 0, 0, 0.5)`]: for command `assert-css: (
            ".docblock .example-wrap.should_panic .tooltip::before",
            {
                "border-width": "5px",
                "border-style": "solid",
                "border-color": "rgba(0, 0, 0, 0) " + |background| + " rgba(0, 0, 0, 0) rgba(0, 0, 0, 0)",
        )`
        )`
[ERROR] (line 97 from 141) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.ignore .tooltip`): [expected `rgb(255, 142, 0)` for key `color`, found `rgba(255, 142, 0, 0.6)`]: for command `assert-css: (
            ".docblock .example-wrap.ignore .tooltip",
            {"color": "rgb(255, 142, 0)"},
        )`
[ERROR] (line 101 from 141) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.ignore`): [expected `2px solid rgb(255, 142, 0)` for key `border-left`, found `2px solid rgba(255, 142, 0, 0.6)`]: for command `assert-css: (
            ".docblock .example-wrap.ignore",
            {"border-left": "2px solid rgb(255, 142, 0)"},
        )`
[ERROR] (line 105 from 141) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.ignore .tooltip`): [expected `"This example is not tested"` for key `content`, found `none`, expected `center` for key `text-align`, found `start`, expected `5px 3px 3px` for key `padding`, found `0px`, expected `rgb(0, 0, 0)` for key `background-color`, found `rgba(0, 0, 0, 0)`, expected `rgb(255, 255, 255)` for key `color`, found `rgba(255, 142, 0, 0.6)`, expected `1px solid rgb(224, 224, 224)` for key `border`, found `0px none rgba(255, 142, 0, 0.6)`]: for command `assert-css: (
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
[ERROR] (line 116 from 141) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.ignore .tooltip`): [expected `5px` for key `border-width`, found `0px`, expected `solid` for key `border-style`, found `none`, expected `rgba(0, 0, 0, 0) rgb(0, 0, 0) rgba(0, 0, 0, 0) rgba(0, 0, 0, 0)` for key `border-color`, found `rgba(255, 142, 0, 0.6)`]: for command `assert-css: (
            ".docblock .example-wrap.ignore .tooltip::before",
            {
                "border-width": "5px",
                "border-style": "solid",
                "border-color": "rgba(0, 0, 0, 0) " + |background| + " rgba(0, 0, 0, 0) rgba(0, 0, 0, 0)",
        )`
        )`
[ERROR] (line 17 from 147) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.compile_fail .tooltip`): [expected `rgb(255, 0, 0)` for key `color`, found `rgba(255, 0, 0, 0.5)`]: for command `assert-css: (
            ".docblock .example-wrap.compile_fail .tooltip",
            {"color": "rgb(255, 0, 0)"},
        )`
[ERROR] (line 21 from 147) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.compile_fail`): [expected `2px solid rgb(255, 0, 0)` for key `border-left`, found `2px solid rgba(255, 0, 0, 0.5)`]: for command `assert-css: (
            ".docblock .example-wrap.compile_fail",
            {"border-left": "2px solid rgb(255, 0, 0)"},
        )`
[ERROR] (line 25 from 147) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.compile_fail .tooltip`): [expected `"This example deliberately fails to compile"` for key `content`, found `none`, expected `center` for key `text-align`, found `start`, expected `5px 3px 3px` for key `padding`, found `0px`, expected `rgb(0, 0, 0)` for key `background-color`, found `rgba(0, 0, 0, 0)`, expected `rgb(255, 255, 255)` for key `color`, found `rgba(255, 0, 0, 0.5)`, expected `1px solid rgb(224, 224, 224)` for key `border`, found `0px none rgba(255, 0, 0, 0.5)`]: for command `assert-css: (
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
[ERROR] (line 36 from 147) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.compile_fail .tooltip`): [expected `5px` for key `border-width`, found `0px`, expected `solid` for key `border-style`, found `none`, expected `rgba(0, 0, 0, 0) rgb(0, 0, 0) rgba(0, 0, 0, 0) rgba(0, 0, 0, 0)` for key `border-color`, found `rgba(255, 0, 0, 0.5)`]: for command `assert-css: (
            ".docblock .example-wrap.compile_fail .tooltip::before",
            {
                "border-width": "5px",
                "border-style": "solid",
                "border-color": "rgba(0, 0, 0, 0) " + |background| + " rgba(0, 0, 0, 0) rgba(0, 0, 0, 0)",
        )`
        )`
[ERROR] (line 57 from 147) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.should_panic .tooltip`): [expected `rgb(255, 0, 0)` for key `color`, found `rgba(255, 0, 0, 0.5)`]: for command `assert-css: (
            ".docblock .example-wrap.should_panic .tooltip",
            {"color": "rgb(255, 0, 0)"},
        )`
[ERROR] (line 61 from 147) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.should_panic`): [expected `2px solid rgb(255, 0, 0)` for key `border-left`, found `2px solid rgba(255, 0, 0, 0.5)`]: for command `assert-css: (
            ".docblock .example-wrap.should_panic",
            {"border-left": "2px solid rgb(255, 0, 0)"},
        )`
[ERROR] (line 65 from 147) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.should_panic .tooltip`): [expected `"This example panics"` for key `content`, found `none`, expected `center` for key `text-align`, found `start`, expected `5px 3px 3px` for key `padding`, found `0px`, expected `rgb(0, 0, 0)` for key `background-color`, found `rgba(0, 0, 0, 0)`, expected `rgb(255, 255, 255)` for key `color`, found `rgba(255, 0, 0, 0.5)`, expected `1px solid rgb(224, 224, 224)` for key `border`, found `0px none rgba(255, 0, 0, 0.5)`]: for command `assert-css: (
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
[ERROR] (line 76 from 147) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.should_panic .tooltip`): [expected `5px` for key `border-width`, found `0px`, expected `solid` for key `border-style`, found `none`, expected `rgba(0, 0, 0, 0) rgb(0, 0, 0) rgba(0, 0, 0, 0) rgba(0, 0, 0, 0)` for key `border-color`, found `rgba(255, 0, 0, 0.5)`]: for command `assert-css: (
            ".docblock .example-wrap.should_panic .tooltip::before",
            {
                "border-width": "5px",
                "border-style": "solid",
                "border-color": "rgba(0, 0, 0, 0) " + |background| + " rgba(0, 0, 0, 0) rgba(0, 0, 0, 0)",
        )`
        )`
[ERROR] (line 97 from 147) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.ignore .tooltip`): [expected `rgb(255, 142, 0)` for key `color`, found `rgba(255, 142, 0, 0.6)`]: for command `assert-css: (
            ".docblock .example-wrap.ignore .tooltip",
            {"color": "rgb(255, 142, 0)"},
        )`
[ERROR] (line 101 from 147) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.ignore`): [expected `2px solid rgb(255, 142, 0)` for key `border-left`, found `2px solid rgba(255, 142, 0, 0.6)`]: for command `assert-css: (
            ".docblock .example-wrap.ignore",
            {"border-left": "2px solid rgb(255, 142, 0)"},
        )`
[ERROR] (line 105 from 147) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.ignore .tooltip`): [expected `"This example is not tested"` for key `content`, found `none`, expected `center` for key `text-align`, found `start`, expected `5px 3px 3px` for key `padding`, found `0px`, expected `rgb(0, 0, 0)` for key `background-color`, found `rgba(0, 0, 0, 0)`, expected `rgb(255, 255, 255)` for key `color`, found `rgba(255, 142, 0, 0.6)`, expected `1px solid rgb(224, 224, 224)` for key `border`, found `0px none rgba(255, 142, 0, 0.6)`]: for command `assert-css: (
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
[ERROR] (line 116 from 147) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap.ignore .tooltip`): [expected `5px` for key `border-width`, found `0px`, expected `solid` for key `border-style`, found `none`, expected `rgba(0, 0, 0, 0) rgb(0, 0, 0) rgba(0, 0, 0, 0) rgba(0, 0, 0, 0)` for key `border-color`, found `rgba(255, 142, 0, 0.6)`]: for command `assert-css: (
            ".docblock .example-wrap.ignore .tooltip::before",
            {
                "border-width": "5px",
                "border-style": "solid",
                "border-color": "rgba(0, 0, 0, 0) " + |background| + " rgba(0, 0, 0, 0) rgba(0, 0, 0, 0)",
        )`

Build completed unsuccessfully in 0:02:06

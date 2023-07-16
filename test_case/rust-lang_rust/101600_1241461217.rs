plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between c6861df8368a35ef3814e0c379754bb4c7013401 and 7253edef330914efe053d292c5b1442be2d93762
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (70/72)
..         (72/72)


/checkout/src/test/rustdoc-gui/codeblock-tooltip.goml codeblock-tooltip... FAILED
[ERROR] (line 15) Error: Evaluation failed: The following errors happened (for selector `.docblock .information .compile_fail`): [expected `rgb(255, 0, 0)` for key `color`, found `rgba(255, 0, 0, 0.5)`]: for command `assert-css: (".docblock .information .compile_fail", {"color": "rgb(255, 0, 0)"})`
[ERROR] (line 16) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap pre.compile_fail`): [expected `2px solid rgb(255, 0, 0)` for key `border-left`, found `2px solid rgba(255, 0, 0, 0.5)`]: for command `assert-css: (".docblock .example-wrap pre.compile_fail", {"border-left": "2px solid rgb(255, 0, 0)"})`
[ERROR] (line 24) Error: Evaluation failed: The following errors happened (for selector `.docblock .information .should_panic`): [expected `rgb(255, 0, 0)` for key `color`, found `rgba(255, 0, 0, 0.5)`]: for command `assert-css: (".docblock .information .should_panic", {"color": "rgb(255, 0, 0)"})`
[ERROR] (line 25) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap pre.should_panic`): [expected `2px solid rgb(255, 0, 0)` for key `border-left`, found `2px solid rgba(255, 0, 0, 0.5)`]: for command `assert-css: (".docblock .example-wrap pre.should_panic", {"border-left": "2px solid rgb(255, 0, 0)"})`
[ERROR] (line 33) Error: Evaluation failed: The following errors happened (for selector `.docblock .information .ignore`): [expected `rgb(255, 142, 0)` for key `color`, found `rgba(255, 142, 0, 0.6)`]: for command `assert-css: (".docblock .information .ignore", {"color": "rgb(255, 142, 0)"})`
[ERROR] (line 34) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap pre.ignore`): [expected `2px solid rgb(255, 142, 0)` for key `border-left`, found `2px solid rgba(255, 142, 0, 0.6)`]: for command `assert-css: (".docblock .example-wrap pre.ignore", {"border-left": "2px solid rgb(255, 142, 0)"})`
[ERROR] (line 46) Error: Evaluation failed: The following errors happened (for selector `.docblock .information .compile_fail`): [expected `rgb(255, 0, 0)` for key `color`, found `rgba(255, 0, 0, 0.5)`]: for command `assert-css: (".docblock .information .compile_fail", {"color": "rgb(255, 0, 0)"})`
[ERROR] (line 47) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap pre.compile_fail`): [expected `2px solid rgb(255, 0, 0)` for key `border-left`, found `2px solid rgba(255, 0, 0, 0.5)`]: for command `assert-css: (".docblock .example-wrap pre.compile_fail", {"border-left": "2px solid rgb(255, 0, 0)"})`
[ERROR] (line 55) Error: Evaluation failed: The following errors happened (for selector `.docblock .information .should_panic`): [expected `rgb(255, 0, 0)` for key `color`, found `rgba(255, 0, 0, 0.5)`]: for command `assert-css: (".docblock .information .should_panic", {"color": "rgb(255, 0, 0)"})`
[ERROR] (line 56) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap pre.should_panic`): [expected `2px solid rgb(255, 0, 0)` for key `border-left`, found `2px solid rgba(255, 0, 0, 0.5)`]: for command `assert-css: (".docblock .example-wrap pre.should_panic", {"border-left": "2px solid rgb(255, 0, 0)"})`
[ERROR] (line 64) Error: Evaluation failed: The following errors happened (for selector `.docblock .information .ignore`): [expected `rgb(255, 142, 0)` for key `color`, found `rgba(255, 142, 0, 0.6)`]: for command `assert-css: (".docblock .information .ignore", {"color": "rgb(255, 142, 0)"})`
[ERROR] (line 65) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap pre.ignore`): [expected `2px solid rgb(255, 142, 0)` for key `border-left`, found `2px solid rgba(255, 142, 0, 0.6)`]: for command `assert-css: (".docblock .example-wrap pre.ignore", {"border-left": "2px solid rgb(255, 142, 0)"})`
[ERROR] (line 77) Error: Evaluation failed: The following errors happened (for selector `.docblock .information .compile_fail`): [expected `rgb(255, 0, 0)` for key `color`, found `rgba(255, 0, 0, 0.5)`]: for command `assert-css: (".docblock .information .compile_fail", {"color": "rgb(255, 0, 0)"})`
[ERROR] (line 78) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap pre.compile_fail`): [expected `2px solid rgb(255, 0, 0)` for key `border-left`, found `2px solid rgba(255, 0, 0, 0.5)`]: for command `assert-css: (".docblock .example-wrap pre.compile_fail", {"border-left": "2px solid rgb(255, 0, 0)"})`
[ERROR] (line 82) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap .should_panic`): [expected `2px solid rgba(255, 0, 0, 0.5)` for key `border-left`, found `0px none rgba(255, 0, 0, 0.5)`]: for command `assert-css: (".docblock .example-wrap .should_panic", {"border-left": "2px solid rgba(255, 0, 0, 0.5)"})`
[ERROR] (line 86) Error: Evaluation failed: The following errors happened (for selector `.docblock .information .should_panic`): [expected `rgb(255, 0, 0)` for key `color`, found `rgba(255, 0, 0, 0.5)`]: for command `assert-css: (".docblock .information .should_panic", {"color": "rgb(255, 0, 0)"})`
[ERROR] (line 87) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap pre.should_panic`): [expected `2px solid rgb(255, 0, 0)` for key `border-left`, found `2px solid rgba(255, 0, 0, 0.5)`]: for command `assert-css: (".docblock .example-wrap pre.should_panic", {"border-left": "2px solid rgb(255, 0, 0)"})`
[ERROR] (line 95) Error: Evaluation failed: The following errors happened (for selector `.docblock .information .ignore`): [expected `rgb(255, 142, 0)` for key `color`, found `rgba(255, 142, 0, 0.6)`]: for command `assert-css: (".docblock .information .ignore", {"color": "rgb(255, 142, 0)"})`
[ERROR] (line 96) Error: Evaluation failed: The following errors happened (for selector `.docblock .example-wrap pre.ignore`): [expected `2px solid rgb(255, 142, 0)` for key `border-left`, found `2px solid rgba(255, 142, 0, 0.6)`]: for command `assert-css: (".docblock .example-wrap pre.ignore", {"border-left": "2px solid rgb(255, 142, 0)"})`
Build completed unsuccessfully in 0:01:51

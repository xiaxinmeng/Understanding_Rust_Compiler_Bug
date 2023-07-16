plain
FF (100/106)
........     (106/106)


/checkout/tests/rustdoc-gui/extend-css.goml extend-css... FAILED
[ERROR] (line 5) Error: Evaluation failed: The following errors happened (for selector `.extend`): [expected `rgb(255, 0, 0)` for key `color`, found `rgb(0, 0, 0)`]: for command `assert-css: (".extend", {"color": "rgb(255, 0, 0)"})`

/checkout/tests/rustdoc-gui/jump-to-def-background.goml jump-to-def-background... FAILED
[ERROR] (line 12 from 20) "body.source .example-wrap pre.rust a" not found: for command `assert-css: (
            "body.source .example-wrap pre.rust a",
            {"background-color": |background_color|},
            ALL,
        )`
[ERROR] (line 12 from 21) "body.source .example-wrap pre.rust a" not found: for command `assert-css: (
            "body.source .example-wrap pre.rust a",
            {"background-color": |background_color|},
            ALL,
        )`
[ERROR] (line 12 from 22) "body.source .example-wrap pre.rust a" not found: for command `assert-css: (
            "body.source .example-wrap pre.rust a",
            {"background-color": |background_color|},
            ALL,
        )`

/checkout/tests/rustdoc-gui/scrape-examples-button-focus.goml scrape-examples-button-focus... FAILED
[ERROR] (line 6) ".scraped-example-list > .scraped-example pre" not found: for command `store-property: (initialScrollTop, ".scraped-example-list > .scraped-example pre", "scrollTop")`
/checkout/tests/rustdoc-gui/scrape-examples-color.goml scrape-examples-color... FAILED
/checkout/tests/rustdoc-gui/scrape-examples-color.goml scrape-examples-color... FAILED
[ERROR] (line 12 from 34) Error: The following CSS selector ".more-examples-toggle" was not found: for command `wait-for: ".more-examples-toggle"`
/checkout/tests/rustdoc-gui/scrape-examples-fonts.goml scrape-examples-fonts... FAILED
/checkout/tests/rustdoc-gui/scrape-examples-fonts.goml scrape-examples-fonts... FAILED
[ERROR] (line 6) Error: The following CSS selector ".scraped-example-title" was not found: for command `wait-for-css: (".scraped-example-title", {"font-family": |font|})`

/checkout/tests/rustdoc-gui/scrape-examples-layout.goml scrape-examples-layout... FAILED
[ERROR] (line 5) ".more-scraped-examples .scraped-example .code-wrapper .src-line-numbers" not found: for command `assert-property-false: (
    ".more-scraped-examples .scraped-example .code-wrapper .src-line-numbers",
    {"clientWidth": "0"}
)`
[ERROR] (line 11) ".more-scraped-examples .scraped-example:nth-child(2) .code-wrapper .src-line-numbers" not found: for command `store-property: (
    clientWidth,
    ".more-scraped-examples .scraped-example:nth-child(2) .code-wrapper .src-line-numbers",
    "clientWidth"


/checkout/tests/rustdoc-gui/scrape-examples-toggle.goml scrape-examples-toggle... FAILED
[ERROR] (line 14 from 29) ".more-examples-toggle" not found: for command `assert-attribute-false: (".more-examples-toggle", {"open": ""})`
[ERROR] (line 15 from 29) ".more-examples-toggle" not found: for command `click: ".more-examples-toggle"`

/checkout/tests/rustdoc-gui/source-anchor-scroll.goml source-anchor-scroll... FAILED
[ERROR] (line 10) XPath "//a[text() = "barbar"]" not found: for command `click: '//a[text() = "barbar"]'`
FFFFFFBuild completed unsuccessfully in 0:04:53

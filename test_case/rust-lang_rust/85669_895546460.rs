
goto: file://|DOC_PATH|/test_docs/index.html
// This test ensures that the correct font is applied on the modules listed items.

// modules
assert-css: ("#modules + item-table .item-left a", {"font-family": '"Fira Sans", Arial, sans-serif'})
assert-css: ("#modules + item-table .item-right.docblock-short", {"font-family": '"Source Serif 4", serif'})
// structs
assert-css: ("#structs + item-table .item-left a", {"font-family": '"Fira Sans", Arial, sans-serif'})
assert-css: ("#structs + item-table .item-right.docblock-short", {"font-family": '"Source Serif 4", serif'})
// enums
assert-css: ("#enums + item-table .item-left a", {"font-family": '"Fira Sans", Arial, sans-serif'})
assert-css: ("#enums + item-table .item-right.docblock-short", {"font-family": '"Source Serif 4", serif'})
// traits
assert-css: ("#traits + item-table .item-left a", {"font-family": '"Fira Sans", Arial, sans-serif'})
assert-css: ("#traits + item-table .item-right.docblock-short", {"font-family": '"Source Serif 4", serif'})
// functions
assert-css: ("#functions + item-table .item-left a", {"font-family": '"Fira Sans", Arial, sans-serif'})
assert-css: ("#functions + item-table .item-right.docblock-short", {"font-family": '"Source Serif 4", serif'})
// keywords
assert-css: ("#keywords + item-table .item-left a", {"font-family": '"Fira Sans", Arial, sans-serif'})
assert-css: ("#keywords + item-table .item-right.docblock-short", {"font-family": '"Source Serif 4", serif'})

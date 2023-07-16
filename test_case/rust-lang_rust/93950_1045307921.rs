plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between b17226fcc11587fed612631be372a5b4cb89988a and 5b49f10b100c5b1459ea6c88dbc2299aba3f1cf8
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

---- compile_test stdout ----
diff of fixed:

 // run-rustfix
 //! This file tests for the `DOC_MARKDOWN` lint.
 #![allow(dead_code, incomplete_features)]
 #![allow(dead_code, incomplete_features)]
 #![warn(clippy::doc_markdown)]
 #![feature(custom_inner_attributes, generic_const_exprs, const_option)]
 #![rustfmt::skip]
 
 /// The `foo_bar` function does _nothing_. See also `foo::bar`. (note the dot there)
 /// Markdown is _weird_. I mean _really weird_. This \_ is ok. So is `_`. But not `Foo::some_fun`
 /// which should be reported only once despite being __doubly bad__.
 /// Here be `::a::global:path`, and _`::another::global::path`_.  :: is not a path though.
 /// Import an item from `::awesome::global::blob::` (Intended postfix)
 /// These are the options for `::Cat`: (Intended trailing single colon, shouldn't be linted)
 /// That's not code ~`NotInCodeBlock`~.
 /// `be_sure_we_got_to_the_end_of_it`
 fn foo_bar() {
 
 
 /// That one tests multiline ticks.
 /// 
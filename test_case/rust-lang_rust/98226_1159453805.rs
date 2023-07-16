plain
    Finished release [optimized] target(s) in 18.82s
warning: the following packages contain code that will be rejected by a future version of Rust: tendril v0.4.1
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
Rustbook (x86_64-unknown-linux-gnu) - unstable-book
[2022-06-18T12:11:01Z WARN  mdbook::book::summary] Expected a start of a link, actually got Some(Text(Borrowed("[")))
Error: Summary parsing failed for file="/checkout/obj/build/x86_64-unknown-linux-gnu/md-doc/unstable-book/src/SUMMARY.md"
 Caused By: There was an error parsing the numbered chapters
 Caused By: There was an error parsing the numbered chapters
 Caused By: failed to parse SUMMARY.md line 11, column 7: The link items for nested chapters must only contain a hyperlink

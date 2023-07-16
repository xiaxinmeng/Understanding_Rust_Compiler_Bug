plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: argument never used
    --> src/librustdoc/passes/collect_intra_doc_links.rs:2023:13
     |
2022 | ...   "see https://doc.rust-lang.org/nightly/rustdoc/linking-to-items-by-name.html#namespaces-and-disambiguators for more info about disambiguator...
     |       ---------------------------------------------------------------------------------------------------------------------------------------------- formatting specifier missing
2023 | ...   channel,
     |       ^^^^^^^ argument never used
error[E0425]: cannot find value `channel` in this scope
    --> src/librustdoc/passes/collect_intra_doc_links.rs:2023:13
     |
2023 |             channel,

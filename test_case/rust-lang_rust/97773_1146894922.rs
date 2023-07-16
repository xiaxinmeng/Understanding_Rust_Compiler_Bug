
error: passing `TyCtxt<'_>` by reference
   --> src/tools/miri/src/helpers.rs:861:30
    |
861 | pub fn get_local_crates(tcx: &TyCtxt<'_>) -> Vec<CrateNum> {
    |                              ^^^^^^^^^^^ help: try passing by value: `TyCtxt<'_>`
    |
    = note: `-D rustc::pass-by-value` implied by `-D warnings`

error: unresolved link to `x`
  --> src/tools/miri/src/data_race.rs:27:39
   |
27 | //! there is some index x where clock[x] > thread_clock, when this is true clock[candidate-idx] > thread_clock
   |                                       ^ no item named `x` in scope
   |
   = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `0`
   --> src/tools/miri/src/eval.rs:400:13
    |
400 | /// in argv[0] cannot be encoded using the standard command line parsing rules.
    |             ^ no item named `0` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: Rust code block is empty
   --> src/tools/miri/src/shims/tls.rs:138:13
    |
138 |     ///     // NOTE: this does not need locks because it only operates on current thread data
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `-D rustdoc::invalid-rust-codeblocks` implied by `-D warnings`

error: this URL is not a hyperlink
 --> src/tools/miri/src/data_race.rs:3:5
  |
3 | //! https://www.doc.ic.ac.uk/~afd/homepages/papers/pdfs/2017/POPL.pdf
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use an automatic link instead: `<https://www.doc.ic.ac.uk/~afd/homepages/papers/pdfs/2017/POPL.pdf>`
  |
  = note: `-D rustdoc::bare-urls` implied by `-D warnings`
  = note: bare URLs are not automatically turned into clickable links

error: this URL is not a hyperlink
 --> src/tools/miri/src/data_race.rs:8:51
  |
8 | //! regarding the weakening of release sequences: http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2018/p0982r1.html.
  |                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use an automatic link instead: `<http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2018/p0982r1.html.>`
  |
  = note: bare URLs are not automatically turned into clickable links

error: this URL is not a hyperlink
  --> src/tools/miri/src/data_race.rs:18:31
   |
18 | //! Data-race definition from(https://en.cppreference.com/w/cpp/language/memory_model#Threads_and_data_races):
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use an automatic link instead: `<https://en.cppreference.com/w/cpp/language/memory_model#Threads_and_data_races>`
   |
   = note: bare URLs are not automatically turned into clickable links

error: this URL is not a hyperlink
   --> src/tools/miri/src/shims/tls.rs:350:9
    |
350 |     /// https://github.com/rust-lang/rust/issues/28129.
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use an automatic link instead: `<https://github.com/rust-lang/rust/issues/28129.>`
    |
    = note: bare URLs are not automatically turned into clickable links

error: this URL is not a hyperlink
   --> src/tools/miri/src/stacked_borrows.rs:948:28
    |
948 |     /// explicit. Also see https://github.com/rust-lang/rust/issues/71117.
    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use an automatic link instead: `<https://github.com/rust-lang/rust/issues/71117.>`
    |
    = note: bare URLs are not automatically turned into clickable links

error: could not document `miri`

Caused by:
  process didn't exit successfully: `/home/infrandomness/Documents/Dev/rust/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name miri src/tools/miri/src/lib.rs --target x86_64-unknown-linux-gnu -o /home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature)' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=3b297133694f0844 -L dependency=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps --extern env_logger=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libenv_logger-ab2e3e42d7aecdbc.rmeta --extern getrandom=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libgetrandom-791bcfb9383522cc.rmeta --extern libc=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/liblibc-caf8d729160798da.rmeta --extern log=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/liblog-f2b61be74cb177c1.rmeta --extern measureme=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libmeasureme-08bbdc8c28660dc4.rmeta --extern rand=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librand-ec279112e72d9cce.rmeta --extern rustc_workspace_hack=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_workspace_hack-be8672fb3f4e083e.rmeta --extern shell_escape=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libshell_escape-51a981842c8d6165.rmeta --extern smallvec=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsmallvec-8bd8d5b8f92d64b4.rmeta -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(release)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version 1.63.0-dev --document-private-items --enable-index-page --show-type-layout --generate-link-to-definition -Zunstable-options` (exit status: 1)
Build completed unsuccessfully in 0:00:22

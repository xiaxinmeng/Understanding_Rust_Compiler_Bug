
$ x check src/librustdoc
check src/librustdoc --stage 0
   Compiling autocfg v1.0.0
    Checking lazy_static v1.4.0
   Compiling proc-macro2 v1.0.24
   Compiling libc v0.2.93
   Compiling unicode-xid v0.2.1
   Compiling syn v1.0.65
    Checking cfg-if v1.0.0
   Compiling memchr v2.4.0
    Checking cfg-if v0.1.10
    Checking regex-syntax v0.6.22
    Checking scopeguard v1.1.0
   Compiling log v0.4.14
   Compiling maybe-uninit v2.0.0
   Compiling byteorder v1.3.4
   Compiling getrandom v0.2.0
   Compiling serde_derive v1.0.125
   Compiling bitflags v1.2.1
   Compiling ucd-trie v0.1.3
   Compiling serde v1.0.125
    Checking same-file v1.0.6
   Compiling version_check v0.9.3
   Compiling ryu v1.0.5
    Checking instant v0.1.6
    Checking smallvec v1.6.1
    Checking fnv v1.0.7
   Compiling maplit v1.0.2
    Checking pin-project-lite v0.2.4
   Compiling rustc-rayon-core v0.3.1
   Compiling serde_json v1.0.59
    Checking ppv-lite86 v0.2.8
   Compiling pulldown-cmark v0.8.0
    Checking ansi_term v0.12.1
    Checking itoa v0.4.6
    Checking either v1.6.0
    Checking termcolor v1.1.2
    Checking remove_dir_all v0.5.3
    Checking macro-utils v0.1.3
    Checking arrayvec v0.7.0
    Checking thread_local v1.0.1
    Checking tracing-core v0.1.17
    Checking sharded-slab v0.1.1
    Checking lock_api v0.4.1
   Compiling crossbeam-utils v0.7.2
   Compiling memoffset v0.5.5
   Compiling crossbeam-epoch v0.8.2
   Compiling crossbeam-utils v0.8.3
    Checking walkdir v2.3.1
   Compiling pest v2.1.3
   Compiling unicase v2.6.0
    Checking itertools v0.9.0
    Checking minifier v0.0.41
    Checking tracing-log v0.1.2
    Checking aho-corasick v0.7.13
    Checking bstr v0.2.13
   Compiling quote v1.0.7
    Checking regex-automata v0.1.9
   Compiling pest_meta v2.1.3
    Checking parking_lot_core v0.8.3
    Checking num_cpus v1.13.0
    Checking atty v0.2.14
    Checking crossbeam-queue v0.2.3
    Checking rand_core v0.6.2
    Checking parking_lot v0.11.1
    Checking rand_chacha v0.3.0
    Checking regex v1.4.3
    Checking crossbeam-deque v0.7.3
    Checking rand v0.8.3
    Checking matchers v0.0.1
    Checking rustc-rayon v0.3.1
    Checking tempfile v3.2.0
    Checking globset v0.4.5
    Checking ignore v0.4.17
    Checking globwalk v0.8.1
   Compiling pest_generator v2.1.3
   Compiling tracing-attributes v0.1.13
   Compiling pest_derive v2.1.0
    Checking tracing v0.1.25
    Checking tracing-subscriber v0.2.16
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/home/joshua/rustc4/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/home/joshua/rustc4/src/librustdoc)
error[E0432]: unresolved import `x`
  --> src/librustdoc/lib.rs:20:5
   |
20 | use x;
   |     ^ no external crate `x`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
error: could not compile `rustdoc`

To learn more, run the command again with --verbose.
Build completed unsuccessfully in 0:00:18


user@pc:~/rust/juniper/juniper ξ rustc +nightly --version                 
rustc 1.30.0-nightly (721913067 2018-08-26)
user@pc:~/rust/juniper/juniper ξ cargo rustc -- -Ztime-passes
warning: file found to be present in multiple build targets: /home/user/rust/juniper/juniper_tests/src/lib.rs
   Compiling matches v0.1.6
   Compiling unicode-normalization v0.1.7
   Compiling cfg-if v0.1.4
   Compiling percent-encoding v1.0.1
   Compiling fnv v1.0.6
   Compiling time v0.1.40
   Compiling uuid v0.6.5
   Compiling unicode-bidi v0.3.4
   Compiling num-traits v0.2.5
   Compiling serde v1.0.69
   Compiling num-integer v0.1.39
   Compiling chrono v0.4.4
   Compiling erased-serde v0.3.6
   Compiling indexmap v1.0.1
   Compiling idna v0.1.4
   Compiling url v1.7.1
   Compiling juniper v0.9.2 (file:///home/weiznich/Dokumente/rust/juniper/juniper)
  time: 0.054; rss: 57MB        parsing
  time: 0.000; rss: 57MB        attributes injection
  time: 0.000; rss: 58MB        garbage collect incremental cache directory
  time: 0.000; rss: 58MB        recursion limit
  time: 0.000; rss: 58MB        crate injection
  time: 0.000; rss: 58MB        plugin loading
  time: 0.000; rss: 58MB        plugin registration
  time: 0.000; rss: 58MB        background load prev dep-graph
  time: 0.004; rss: 58MB        pre ast expansion lint checks
    time: 0.186; rss: 112MB     expand crate
    time: 0.000; rss: 112MB     check unused macros
  time: 0.186; rss: 112MB       expansion
  time: 0.000; rss: 112MB       maybe building test harness
  time: 0.002; rss: 112MB       maybe creating a macro crate
  time: 0.006; rss: 112MB       creating allocators
  time: 0.005; rss: 112MB       AST validation
  time: 0.063; rss: 121MB       name resolution
  time: 0.007; rss: 121MB       complete gated feature checking
  time: 0.000; rss: 121MB       blocked while dep-graph loading finishes
  time: 0.044; rss: 130MB       lowering ast -> hir
  time: 0.020; rss: 130MB       early lint checks
  time: 0.061; rss: 133MB       indexing hir
  time: 0.000; rss: 128MB       load query result cache
  time: 0.000; rss: 128MB       looking for entry point
  time: 0.000; rss: 128MB       looking for plugin registrar
  time: 0.006; rss: 128MB       loop checking
  time: 0.006; rss: 134MB       attribute checking
  time: 0.018; rss: 136MB       stability checking
  time: 0.111; rss: 161MB       type collecting
  time: 0.000; rss: 161MB       outlives testing
  time: 0.004; rss: 161MB       impl wf inference
  time: 0.079; rss: 179MB       coherence checking
  time: 0.000; rss: 179MB       variance testing
  time: 0.332; rss: 183MB       wf checking
  time: 0.049; rss: 188MB       item-types checking
error: Could not compile `juniper`.

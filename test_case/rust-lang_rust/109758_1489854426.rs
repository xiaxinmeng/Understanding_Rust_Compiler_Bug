plain
 Documenting rustc_log v0.0.0 (/checkout/compiler/rustc_log)
error: unclosed HTML tag `T`
  --> compiler/rustc_data_structures/src/sync.rs:24:10
   |
24 | //! | Lrc<T>                | rc::Rc<T>         | sync::Arc<T>                  |
   |
   |
   = note: `-D rustdoc::invalid-html-tags` implied by `-D warnings`
   |
   |
24 | //! | `Lrc<T>`                | rc::Rc<T>         | sync::Arc<T>                  |

error: unclosed HTML tag `T`
  --> compiler/rustc_data_structures/src/sync.rs:24:37
   |
   |
24 | //! | Lrc<T>                | rc::Rc<T>         | sync::Arc<T>                  |
   |
help: try marking as source code
   |
   |
24 | //! | Lrc<T>                | `rc::Rc<T>`         | sync::Arc<T>                  |

error: unclosed HTML tag `T`
  --> compiler/rustc_data_structures/src/sync.rs:24:60
   |
   |
24 | //! | Lrc<T>                | rc::Rc<T>         | sync::Arc<T>                  |
   |
help: try marking as source code
   |
   |
24 | //! | Lrc<T>                | rc::Rc<T>         | `sync::Arc<T>`                  |

error: unclosed HTML tag `T`
  --> compiler/rustc_data_structures/src/sync.rs:25:11
   |
   |
25 | //! | Weak<T>               | rc::Weak<T>       | sync::Weak<T>                 |
   |
help: try marking as source code
   |
   |
25 | //! | `Weak<T>`               | rc::Weak<T>       | sync::Weak<T>                 |

error: unclosed HTML tag `T`
  --> compiler/rustc_data_structures/src/sync.rs:25:39
   |
   |
25 | //! | Weak<T>               | rc::Weak<T>       | sync::Weak<T>                 |
   |
help: try marking as source code
   |
   |
25 | //! | Weak<T>               | `rc::Weak<T>`       | sync::Weak<T>                 |

error: unclosed HTML tag `T`
  --> compiler/rustc_data_structures/src/sync.rs:25:61
   |
   |
25 | //! | Weak<T>               | rc::Weak<T>       | sync::Weak<T>                 |
   |
help: try marking as source code
   |
   |
25 | //! | Weak<T>               | rc::Weak<T>       | `sync::Weak<T>`                 |

error: unclosed HTML tag `bool`
  --> compiler/rustc_data_structures/src/sync.rs:27:35
   |
   |
27 | //! | AtomicBool            | Cell<bool>        | atomic::AtomicBool            |
   |
help: try marking as source code
   |
   |
27 | //! | AtomicBool            | `Cell<bool>`        | atomic::AtomicBool            |

error: unclosed HTML tag `u32`
  --> compiler/rustc_data_structures/src/sync.rs:28:35
   |
   |
28 | //! | AtomicU32             | Cell<u32>         | atomic::AtomicU32             |
   |
help: try marking as source code
   |
   |
28 | //! | AtomicU32             | `Cell<u32>`         | atomic::AtomicU32             |

error: unclosed HTML tag `u64`
  --> compiler/rustc_data_structures/src/sync.rs:29:35
   |
   |
29 | //! | AtomicU64             | Cell<u64>         | atomic::AtomicU64             |
   |
help: try marking as source code
   |
   |
29 | //! | AtomicU64             | `Cell<u64>`         | atomic::AtomicU64             |

error: unclosed HTML tag `usize`
  --> compiler/rustc_data_structures/src/sync.rs:30:35
   |
   |
30 | //! | AtomicUsize           | Cell<usize>       | atomic::AtomicUsize           |
   |
help: try marking as source code
   |
   |
30 | //! | AtomicUsize           | `Cell<usize>`       | atomic::AtomicUsize           |

error: unclosed HTML tag `T`
  --> compiler/rustc_data_structures/src/sync.rs:32:11
   |
   |
32 | //! | Lock<T>               | RefCell<T>        | parking_lot::Mutex<T>         |
   |
help: try marking as source code
   |
   |
32 | //! | `Lock<T>`               | RefCell<T>        | parking_lot::Mutex<T>         |

error: unclosed HTML tag `T`
  --> compiler/rustc_data_structures/src/sync.rs:32:38
   |
   |
32 | //! | Lock<T>               | RefCell<T>        | parking_lot::Mutex<T>         |
   |
help: try marking as source code
   |
   |
32 | //! | Lock<T>               | `RefCell<T>`        | parking_lot::Mutex<T>         |

error: unclosed HTML tag `T`
  --> compiler/rustc_data_structures/src/sync.rs:32:69
   |
   |
32 | //! | Lock<T>               | RefCell<T>        | parking_lot::Mutex<T>         |
   |
help: try marking as source code
   |
   |
32 | //! | Lock<T>               | RefCell<T>        | `parking_lot::Mutex<T>`         |

error: unclosed HTML tag `T`
  --> compiler/rustc_data_structures/src/sync.rs:33:13
   |
   |
33 | //! | RwLock<T>             | RefCell<T>        | parking_lot::RwLock<T>        |
   |
help: try marking as source code
   |
   |
33 | //! | `RwLock<T>`             | RefCell<T>        | parking_lot::RwLock<T>        |

error: unclosed HTML tag `T`
  --> compiler/rustc_data_structures/src/sync.rs:33:38
   |
   |
33 | //! | RwLock<T>             | RefCell<T>        | parking_lot::RwLock<T>        |
   |
help: try marking as source code
   |
   |
33 | //! | RwLock<T>             | `RefCell<T>`        | parking_lot::RwLock<T>        |

error: unclosed HTML tag `T`
  --> compiler/rustc_data_structures/src/sync.rs:33:70
   |
   |
33 | //! | RwLock<T>             | RefCell<T>        | parking_lot::RwLock<T>        |
   |
help: try marking as source code
   |
   |
33 | //! | RwLock<T>             | RefCell<T>        | `parking_lot::RwLock<T>`        |

error: unclosed HTML tag `T`
  --> compiler/rustc_data_structures/src/sync.rs:34:13
   |
   |
34 | //! | MTLock<T>        [^1] | T                 | Lock<T>                       |
   |
help: try marking as source code
   |
   |
34 | //! | `MTLock<T>`        [^1] | T                 | Lock<T>                       |

error: unclosed HTML tag `T`
  --> compiler/rustc_data_structures/src/sync.rs:34:55
   |
   |
34 | //! | MTLock<T>        [^1] | T                 | Lock<T>                       |
   |
help: try marking as source code
   |
   |
34 | //! | MTLock<T>        [^1] | T                 | `Lock<T>`                       |

error: unclosed HTML tag `T`
  --> compiler/rustc_data_structures/src/sync.rs:35:45
   |
   |
35 | //! | MTLockRef<'a, T> [^2] | &'a mut MTLock<T> | &'a MTLock<T>                 |
   |
help: try marking as source code
   |
   |
35 | //! | MTLockRef<'a, T> [^2] | &'a mut `MTLock<T>` | &'a MTLock<T>                 |

error: unclosed HTML tag `T`
  --> compiler/rustc_data_structures/src/sync.rs:35:61
   |
   |
35 | //! | MTLockRef<'a, T> [^2] | &'a mut MTLock<T> | &'a MTLock<T>                 |
   |
help: try marking as source code
   |
   |
35 | //! | MTLockRef<'a, T> [^2] | &'a mut MTLock<T> | &'a `MTLock<T>`                 |

error: could not document `rustc_data_structures`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_data_structures compiler/rustc_data_structures/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature, "rustc-rayon", "rustc-rayon-core", "rustc_use_parallel_compiler")' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=4b2edba66e420807 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arrayvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarrayvec-b2a22b52ea3f5b1e.rmeta --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-45eb07aeb196189e.rmeta --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcfg_if-098384a133561449.rmeta --extern elsa=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libelsa-e6b543500acba7b3.rmeta --extern ena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libena-6cf84d48f04b7427.rmeta --extern indexmap=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libindexmap-98bc4d679e7a731f.rmeta --extern jobserver_crate=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-76c0a8460ee22343.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-71db8ade030065e2.rmeta --extern measureme=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libmeasureme-1105ee199f36d1d2.rmeta --extern memmap2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libmemmap2-a75cf78a494dd7d8.rmeta --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-bab3d7e4b8141cdb.rmeta --extern rustc_hash=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hash-9cd2ae6bb5e2cf71.rmeta --extern rustc_graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_graphviz-6fa933368128c19c.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-0f9fffa2ac7435dc.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/librustc_macros-6e0f0ac42fa294f5.so --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-1ef1cb12af175b89.rmeta --extern serde_json=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserde_json-e71675645db8c3c2.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-4d2ed8dbf5f2b6bc.rmeta --extern stable_deref_trait=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libstable_deref_trait-1d766571af93ec49.rmeta --extern stacker=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libstacker-61010e5396e96836.rmeta --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-d9407093241c2581.rmeta --extern thin_vec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libthin_vec-efc016f9e25c117e.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-99e0b36291fbf743.rmeta --extern-html-root-url 'arrayvec=https://docs.rs/arrayvec/0.7.0/' --extern-html-root-url 'bitflags=https://docs.rs/bitflags/1.3.2/' --extern-html-root-url 'cfg_if=https://docs.rs/cfg-if/1.0.0/' --extern-html-root-url 'elsa=https://docs.rs/elsa/1.8.0/' --extern-html-root-url 'ena=https://docs.rs/ena/0.14.2/' --extern-html-root-url 'indexmap=https://docs.rs/indexmap/1.9.3/' --extern-html-root-url 'jobserver=https://docs.rs/jobserver/0.1.26/' --extern-html-root-url 'libc=https://docs.rs/libc/0.2.140/' --extern-html-root-url 'measureme=https://docs.rs/measureme/10.1.0/' --extern-html-root-url 'memmap2=https://docs.rs/memmap2/0.2.1/' --extern-html-root-url 'parking_lot=https://docs.rs/parking_lot/0.11.2/' --extern-html-root-url 'rustc_hash=https://docs.rs/rustc-hash/1.1.0/' --extern-html-root-url 'serde_json=https://docs.rs/serde_json/1.0.85/' --extern-html-root-url 'smallvec=https://docs.rs/smallvec/1.10.0/' --extern-html-root-url 'stable_deref_trait=https://docs.rs/stable_deref_trait/1.2.0/' --extern-html-root-url 'stacker=https://docs.rs/stacker/0.1.15/' --extern-html-root-url 'tempfile=https://docs.rs/tempfile/3.3.0/' --extern-html-root-url 'thin_vec=https://docs.rs/thin-vec/0.2.12/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.35/' -Zunstable-options --cfg=bootstrap --cfg=windows_raw_dylib -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' '--check-cfg=values(rustix_use_libc)' '--check-cfg=values(emulate_second_only_system)' '--check-cfg=values(windows_raw_dylib)' --document-private-items --document-hidden-items -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.70.0-nightly
  (d37e6d016
  2023-03-30)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
Build completed unsuccessfully in 0:00:07

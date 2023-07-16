
Apr 01 16:15:13.014 INFO blam! ---- identity_map stdout ----
Apr 01 16:15:13.014 INFO blam! 	we missed 0x00254c40: (Some("dwarf.c"), None)
Apr 01 16:15:13.014 INFO blam! we missed 0x0024de20: (Some("elf.c"), None)
Apr 01 16:15:13.014 INFO blam! we missed 0x00018320: (Some("crtstuff.c"), None)
Apr 01 16:15:13.014 INFO blam! we missed 0x00252db0: (Some("dwarf.c"), None)
Apr 01 16:15:13.014 INFO blam! thread 'identity_map' panicked at 'assertion failed: `(left == right)`
Apr 01 16:15:13.014 INFO blam!   left: `(Some("/checkout/src/liballoc_jemalloc/../jemalloc/src/prof.c"), Some(2164))`,
Apr 01 16:15:13.014 INFO blam!  right: `(Some("/checkout/src/liballoc_jemalloc/../jemalloc/src/prof.c"), Some(1984))`', tests/integration_test.rs:61:13

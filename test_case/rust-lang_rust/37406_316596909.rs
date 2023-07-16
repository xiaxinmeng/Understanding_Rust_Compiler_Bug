
error: is feature gated (see issue #37406)
   --> /home/clar/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.26/src/unix/mod.rs:227:9
    |
227 |         #[link(name = "c", kind = "static", cfg(target_feature = "crt-static"))]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(link_cfg)] to the crate attributes to enable

error: is feature gated (see issue #37406)
   --> /home/clar/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.26/src/unix/mod.rs:228:9
    |
228 |         #[link(name = "c", cfg(not(target_feature = "crt-static")))]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(link_cfg)] to the crate attributes to enable

plain
   Compiling rustc-demangle v0.1.18
error: associated function is never used: `into_key`
   --> library/alloc/src/collections/btree/map/entry.rs:293:12
    |
293 |     pub fn into_key(self) -> K {
    |
    |
    = note: `-D dead-code` implied by `-D warnings`
error: could not compile `alloc` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:04:48

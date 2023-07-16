plain
    Checking chalk-ir v0.55.0
    Checking tracing v0.1.28
    Checking rustc_index v0.0.0 (/checkout/compiler/rustc_index)
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error: field is never read: `cgu_merge_event_kind`
    |
    |
431 |     cgu_merge_event_kind: StringId,
    |
    |
    = note: `-D dead-code` implied by `-D warnings`
error: could not compile `rustc_data_structures` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:02:07

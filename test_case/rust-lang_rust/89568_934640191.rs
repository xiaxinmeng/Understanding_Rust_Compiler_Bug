plain
    Checking chalk-ir v0.55.0
    Checking tracing v0.1.28
    Checking rustc_index v0.0.0 (/checkout/compiler/rustc_index)
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error[E0560]: struct `SelfProfiler` has no field named `cgu_name_event_kind`
    |
    |
499 |             cgu_name_event_kind,
    |             ^^^^^^^^^^^^^^^^^^^ help: a field with a similar name exists: `cgu_merge_event_kind`
For more information about this error, try `rustc --explain E0560`.
error: could not compile `rustc_data_structures` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed

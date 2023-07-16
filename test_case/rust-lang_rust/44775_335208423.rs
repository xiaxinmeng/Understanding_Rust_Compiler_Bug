
[01:14:12] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[01:14:12]   left: `"Handle"`,
[01:14:12]  right: `"Handle { .. }"`', /checkout/src/libstd/sync/mpsc/select.rs:789:8
[01:14:12] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[01:14:12]   left: `"Select"`,
[01:14:12]  right: `"Select { .. }"`', /checkout/src/libstd/sync/mpsc/select.rs:781:8
[01:14:13] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[01:14:13]   left: `"Receiver"`,
[01:14:13]  right: `"Receiver { .. }"`', /checkout/src/libstd/sync/mpsc/mod.rs:3022:8
[01:14:13] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[01:14:13]   left: `"Sender"`,
[01:14:13]  right: `"Sender { .. }"`', /checkout/src/libstd/sync/mpsc/mod.rs:3016:8
[01:14:13] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[01:14:13]   left: `"SyncSender"`,
[01:14:13]  right: `"SyncSender { .. }"`', /checkout/src/libstd/sync/mpsc/mod.rs:3028:8
...
[01:14:23] failures:
[01:14:23]     sync::mpsc::select::tests::fmt_debug_handle
[01:14:23]     sync::mpsc::select::tests::fmt_debug_select
[01:14:23]     sync::mpsc::sync_tests::fmt_debug_recv
[01:14:23]     sync::mpsc::sync_tests::fmt_debug_sender
[01:14:23]     sync::mpsc::sync_tests::fmt_debug_sync_sender
[01:14:23] 
[01:14:23] test result: FAILED. 803 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out

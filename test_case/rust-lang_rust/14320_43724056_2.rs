
warning: using multiple versions of crate `std`
exp.rs:4:1: 4:37 note: used here
exp.rs:4 extern crate std = "std#0.11.0-pre";
         ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
note: crate_id: std#0.11.0-pre
exp.rs:7:1: 7:37 note: used here
exp.rs:7 extern crate std = "std#0.11.0-pre";
         ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
note: crate_id: std#0.11.0-pre
error: duplicate entry for `malloc`
error: duplicate entry for `free`
error: duplicate entry for `strdup_uniq`
error: duplicate entry for `eh_personality`
error: duplicate entry for `managed_heap`
error: duplicate entry for `gc`
error: aborting due to 6 previous errors

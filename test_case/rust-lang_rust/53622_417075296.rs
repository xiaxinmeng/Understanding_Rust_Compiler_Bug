
[00:45:11] ---- [ui] ui/panic-runtime/runtime-depend-on-needs-runtime.rs stdout ----
[00:45:11] diff of stderr:
[00:45:11] 
[00:45:11] 1 error: the crate `depends` cannot depend on a crate that needs a panic runtime, but it depends on `needs_panic_runtime`
[00:45:11] - error: aborting due to previous error
[00:45:11] - error: aborting due to previous error
[00:45:11] + error: language item required, but not found: `eh_personality`
[00:45:11] + error: aborting due to 2 previous errors

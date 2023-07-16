
   Compiling rustc v0.0.0 (file:///home/nmatsakis/versioned/rust-5/src/librustc)
^[[0m^[[1m^[[38;5;9merror[E0428]^[[0m^[[0m^[[1m: the name `ProgramClause` is defined multiple times^[[0m
^[[0m   ^[[0m^[[0m^[[1m^[[38;5;12m--> ^[[0m^[[0mlibrustc/traits/mod.rs:342:1^[[0m
^[[0m    ^[[0m^[[0m^[[1m^[[38;5;12m|^[[0m
^[[0m^[[1m^[[38;5;12m327^[[0m^[[0m ^[[0m^[[0m^[[1m^[[38;5;12m| ^[[0m^[[0mpub struct ProgramClause<'tcx> {^[[0m
^[[0m    ^[[0m^[[0m^[[1m^[[38;5;12m| ^[[0m^[[0m^[[1m^[[38;5;12m------------------------------^[[0m^[[0m ^[[0m^[[0m^[[1m^[[38;5;12mprevious definition of the type `ProgramClause` here^[[0m
^[[0m^[[1m^[[38;5;12m...^[[0m
^[[0m^[[1m^[[38;5;12m342^[[0m^[[0m ^[[0m^[[0m^[[1m^[[38;5;12m| ^[[0m^[[0mpub struct ProgramClause<'tcx> {^[[0m
^[[0m    ^[[0m^[[0m^[[1m^[[38;5;12m| ^[[0m^[[0m^[[1m^[[38;5;9m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[[0m^[[0m ^[[0m^[[0m^[[1m^[[38;5;9m`ProgramClause` redefined here^[[0m
^[[0m    ^[[0m^[[0m^[[1m^[[38;5;12m|^[[0m
^[[0m    ^[[0m^[[0m^[[1m^[[38;5;12m= ^[[0m^[[0m^[[1mnote^[[0m^[[0m: `ProgramClause` must be defined only once in the type namespace of this module^[[0m

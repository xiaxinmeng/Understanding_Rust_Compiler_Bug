

---- [ui] ui/issues/issue-18075.rs stdout ----
normalized stderr:
warning: some trace filter directives would enable traces that are disabled statically
 | `rustc::middle=debug` would enable the DEBUG level for the `rustc::middle` target
 = note: the static max level is `info`
 = help: to enable DEBUG logging, remove the `max_level_info` feature

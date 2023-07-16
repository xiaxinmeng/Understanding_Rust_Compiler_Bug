
[01:34:34] [0m[1m[38;5;9merror[E0308][0m[0m[1m: mismatched types[0m
[01:34:34] [0m   [0m[0m[1m[38;5;12m--> [0m[0msrc/tools/miri/src/lib.rs:481:74[0m
[01:34:34] [0m    [0m[0m[1m[38;5;12m|[0m
[01:34:34] [0m[1m[38;5;12m481[0m[0m [0m[0m[1m[38;5;12m| [0m[0m        let link_name = match attr::first_attr_value_str_by_name(&attrs, "link_name") {[0m
[01:34:34] [0m    [0m[0m[1m[38;5;12m| [0m[0m                                                                         [0m[0m[1m[38;5;9m^^^^^^^^^^^[0m[0m [0m[0m[1m[38;5;9mexpected struct `syntax::ast::Name`, found reference[0m
[01:34:34] [0m    [0m[0m[1m[38;5;12m|[0m
[01:34:34] [0m    [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: expected type `[0m[0m[1msyntax::ast::Name[0m[0m`[0m
[01:34:34] [0m               found type `[0m[0m[1m&'static str[0m[0m`[0m
[01:34:34] 
[01:34:34] [0m[1m[38;5;9merror[E0308][0m[0m[1m: mismatched types[0m
[01:34:34] [0m   [0m[0m[1m[38;5;12m--> [0m[0msrc/tools/miri/src/fn_call.rs:144:74[0m
[01:34:34] [0m    [0m[0m[1m[38;5;12m|[0m
[01:34:34] [0m[1m[38;5;12m144[0m[0m [0m[0m[1m[38;5;12m| [0m[0m        let link_name = match attr::first_attr_value_str_by_name(&attrs, "link_name") {[0m
[01:34:34] [0m    [0m[0m[1m[38;5;12m| [0m[0m                                                                         [0m[0m[1m[38;5;9m^^^^^^^^^^^[0m[0m [0m[0m[1m[38;5;9mexpected struct `syntax::ast::Name`, found reference[0m
[01:34:34] [0m    [0m[0m[1m[38;5;12m|[0m
[01:34:34] [0m    [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: expected type `[0m[0m[1msyntax::ast::Name[0m[0m`[0m
[01:34:34] [0m               found type `[0m[0m[1m&'static str[0m[0m`[0m
[01:34:34] 
[01:34:35] [0m[1m[38;5;9merror[E0308][0m[0m[1m: mismatched types[0m
[01:34:35] [0m  [0m[0m[1m[38;5;12m--> [0m[0msrc/tools/miri/src/helpers.rs:17:68[0m
[01:34:35] [0m   [0m[0m[1m[38;5;12m|[0m
[01:34:35] [0m[1m[38;5;12m17[0m[0m [0m[0m[1m[38;5;12m| [0m[0m            .find(|&&krate| this.tcx.original_crate_name(krate) == path[0])[0m
[01:34:35] [0m   [0m[0m[1m[38;5;12m| [0m[0m                                                                   [0m[0m[1m[38;5;9m^^^^^^^[0m[0m [0m[0m[1m[38;5;9mexpected struct `syntax::ast::Name`, found &str[0m
[01:34:35] [0m   [0m[0m[1m[38;5;12m|[0m
[01:34:35] [0m   [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: expected type `[0m[0m[1msyntax::ast::Name[0m[0m`[0m
[01:34:35] [0m              found type `[0m[0m[1m&str[0m[0m`[0m
[01:34:35] 
[01:34:35] [0m[1m[38;5;9merror[E0308][0m[0m[1m: mismatched types[0m
[01:34:35] [0m  [0m[0m[1m[38;5;12m--> [0m[0msrc/tools/miri/src/helpers.rs:28:47[0m
[01:34:35] [0m   [0m[0m[1m[38;5;12m|[0m
[01:34:35] [0m[1m[38;5;12m28[0m[0m [0m[0m[1m[38;5;12m| [0m[0m                        if item.ident.name == *segment {[0m
[01:34:35] [0m   [0m[0m[1m[38;5;12m| [0m[0m                                              [0m[0m[1m[38;5;9m^^^^^^^^[0m[0m [0m[0m[1m[38;5;9mexpected struct `syntax::ast::Name`, found &str[0m
[01:34:35] [0m   [0m[0m[1m[38;5;12m|[0m
[01:34:35] [0m   [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: expected type `[0m[0m[1msyntax::ast::Name[0m[0m`[0m
[01:34:35] [0m              found type `[0m[0m[1m&str[0m[0m`[0m
[01:34:35] 
[01:34:36] [0m[1m[38;5;9merror[0m[0m[1m: aborting due to 4 previous errors[0m

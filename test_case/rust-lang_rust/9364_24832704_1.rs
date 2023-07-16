
>rustc --pretty expanded parse.rs
stmt_list! { while y < 0 { let y = x + 1 ; x = y ; } }
parse.rs:37:8: 37:9 error: Local ambiguity: multiple parsing options: built-in NTs tt ('body') or 1 other options.
parse.rs:37         }
                    ^

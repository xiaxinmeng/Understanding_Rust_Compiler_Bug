plain
    Checking rand v0.7.3
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
error[E0658]: use of unstable library feature 'split_rinclusive'
     |
     |
1333 |     let split: Vec<&str> = data.split_rinclusive('\n').collect();
     |
     |
     = help: add `#![feature(split_rinclusive)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'split_rinclusive'
     |
     |
1338 |         .split_rinclusive(char::is_uppercase)
     |
     |
     = help: add `#![feature(split_rinclusive)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'split_rinclusive'
     |
     |
1371 |     let split: Vec<&str> = data.split_rinclusive('\n').rev().collect();
     |
     |
     = help: add `#![feature(split_rinclusive)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'split_rinclusive'
     |
     |
1376 |         .split_rinclusive(char::is_uppercase)
     |
     |
     = help: add `#![feature(split_rinclusive)]` to the crate attributes to enable
For more information about this error, try `rustc --explain E0658`.
error: could not compile `alloc` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed

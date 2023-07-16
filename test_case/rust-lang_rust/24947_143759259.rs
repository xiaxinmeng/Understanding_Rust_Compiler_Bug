
issue-24947.rs:29:29: 29:51 error: expected constant integer for repeat count, but non-constant path in constant expr [E0307]
issue-24947.rs:29     let w: [u8; 12] = [0u8; <Foo as Bar>::BAR_SIZE];
                                              ^~~~~~~~~~~~~~~~~~~~~~
issue-24947.rs:29:29: 29:51 help: run `rustc --explain E0307` to see a detailed explanation
issue-24947.rs:29:23: 29:52 error: mismatched types:
 expected `[u8; 12]`,
    found `[u8; 0]`
(expected an array with a fixed size of 12 elements,
    found one with 0 elements) [E0308]
issue-24947.rs:29     let w: [u8; 12] = [0u8; <Foo as Bar>::BAR_SIZE];
                                        ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~
issue-24947.rs:29:23: 29:52 help: run `rustc --explain E0308` to see a detailed explanation
issue-24947.rs:31:29: 31:44 error: expected constant integer for repeat count, but non-constant path in constant expr [E0307]
issue-24947.rs:31     let x: [u8; 12] = [0u8; <Foo>::BAR_SIZE];
                                              ^~~~~~~~~~~~~~~
issue-24947.rs:31:29: 31:44 help: run `rustc --explain E0307` to see a detailed explanation
issue-24947.rs:31:23: 31:45 error: mismatched types:
 expected `[u8; 12]`,
    found `[u8; 0]`
(expected an array with a fixed size of 12 elements,
    found one with 0 elements) [E0308]
issue-24947.rs:31     let x: [u8; 12] = [0u8; <Foo>::BAR_SIZE];
                                        ^~~~~~~~~~~~~~~~~~~~~~
issue-24947.rs:31:23: 31:45 help: run `rustc --explain E0308` to see a detailed explanation
error: aborting due to 4 previous errors

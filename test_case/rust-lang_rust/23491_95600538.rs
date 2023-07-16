
issue-23491-2.rs:5:41: 5:49 error: mismatched types:
     expected `Node<[isize]>`,
     found `Node<[isize; 0]>`
 (expected slice,
    found array of 0 elements) [E0308]
issue-23491-2.rs:5    let x: Box<Node<[isize]>> = Box::new(Node([]));
                                                           ^~~~~~~~
issue-23491-2.rs:5:32: 5:40 error: the trait `core::marker::Sized` is not implemented for the type  `[isize]` [E0277]
issue-23491-2.rs:5    let x: Box<Node<[isize]>> = Box::new(Node([]));
                                                   ^~~~~~~~
issue-23491-2.rs:5:32: 5:40 note: `[isize]` does not have a constant size known at compile-time
issue-23491-2.rs:5    let x: Box<Node<[isize]>> = Box::new(Node([]));
                                                    ^~~~~~~~
error: aborting due to 2 previous errors

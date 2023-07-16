plain
    Checking rustc-demangle v0.1.21
error: expected identifier, found `<<`
    --> library/alloc/src/collections/vec_deque/mod.rs:2537:1
     |
519  | impl<T, A: Allocator> VecDeque<T, A> {
     |                                      - while parsing this item list starting here
...
2537 | <<<<<<< HEAD
     | ^^ expected identifier
2748 | }
2748 | }
     | - the item list ends here
error: could not compile `alloc` due to previous error
Build completed unsuccessfully in 0:01:43

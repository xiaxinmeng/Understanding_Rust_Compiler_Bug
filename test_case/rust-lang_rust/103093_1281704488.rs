plain
    Checking rustc-demangle v0.1.21
error: specializing impl repeats parameter `A`
    --> library/alloc/src/collections/linked_list.rs:1935:1
     |
1935 | impl<T, A: Allocator> SpecExtend<LinkedList<T, A>> for LinkedList<T, A> {

error: could not compile `alloc` due to previous error
Build completed unsuccessfully in 0:01:41

plain
   Compiling rustc-demangle v0.1.21
error: specializing impl repeats parameter `A`
    --> library/alloc/src/collections/linked_list.rs:1917:1
     |
1917 | impl<T, A: Allocator + Clone> SpecExtend<LinkedList<T, A>> for LinkedList<T, A> {

error: could not compile `alloc` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:00:18

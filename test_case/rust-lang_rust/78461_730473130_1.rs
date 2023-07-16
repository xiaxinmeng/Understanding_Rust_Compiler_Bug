text
Error while trying to register breakpoint callback, id = 1, message = error: could not get num args: can't find callable: breakpoint_callback

run
Process 65506 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x000000010003d3d9 a`pretty_std_collections::main::h1ef1d7ce5967b6a7 at pretty-std-collections.rs:158:5 155 hash_set.insert(i); 156 } 157 -> 158 zzz(); // #break ^ 159 } 160 161 fn zzz() { Target 0: (a) stopped. Process 65506 launched: '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/pretty-std-collections.lldb/a' (x86_64) 
print vec_deque
(alloc::collections::vec_deque::VecDeque<int>) $0 = size=3 { [0] = 5 [1] = 3 [2] = 7 } 
print vec_deque2
(alloc::collections::vec_deque::VecDeque<int>) $1 = size=7 { [0] = 2 [1] = 3 [2] = 4 [3] = 5 [4] = 6 [5] = 7 [6] = 8 } 
print hash_map

------------------------------------------
stderr:
------------------------------------------
error: need to add support for DW_TAG_base_type '()' encoded with DW_ATE = 0x7, bit_size = 0
clangclang: CommandLine Error: Option ': CommandLine Error: Option 'h' registered more than once!
LLVM ERROR: inconsistency in registered CommandLine options
h' registered more than once!
LLVM ERROR: inconsistency in registered CommandLine options

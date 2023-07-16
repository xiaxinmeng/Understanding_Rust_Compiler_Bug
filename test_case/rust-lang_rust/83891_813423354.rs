
0-13-18 rust.git/objdir-opt (git:assign_shrink_your_normal_code) % rust-lldb ./pretty-std-collections
(lldb) command script import "/home/pnkfelix/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/etc/lldb_lookup.py"
(lldb) command source -s 0 '/home/pnkfelix/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/etc/lldb_commands'
Executing commands in '/home/pnkfelix/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/etc/lldb_commands'.
(lldb) type synthetic add -l lldb_lookup.synthetic_lookup -x ".*" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(alloc::([a-z_]+::)+)String$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^&str$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^&\\[.+\\]$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(std::ffi::([a-z_]+::)+)OsString$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(alloc::([a-z_]+::)+)Vec<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(alloc::([a-z_]+::)+)VecDeque<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(alloc::([a-z_]+::)+)BTreeSet<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(alloc::([a-z_]+::)+)BTreeMap<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(std::collections::([a-z_]+::)+)HashMap<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(std::collections::([a-z_]+::)+)HashSet<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(alloc::([a-z_]+::)+)Rc<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(alloc::([a-z_]+::)+)Arc<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(core::([a-z_]+::)+)Cell<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(core::([a-z_]+::)+)Ref<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(core::([a-z_]+::)+)RefMut<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(core::([a-z_]+::)+)RefCell<.+>$" --category Rust
(lldb) type category enable Rust
(lldb) target create "./pretty-std-collections"
Current executable set to '/media/pnkfelix/Rust/rust.git/objdir-opt/pretty-std-collections' (x86_64).
(lldb) b pretty-std-collections.rs:159
b pretty-std-collections.rs:159
Breakpoint 1: where = pretty-std-collections`pretty_std_collections::main::h809023208ec02992 + 1666 at pretty-std-collections.rs:159:5, address = 0x0000000000041c82
(lldb) run
run
Process 604949 launched: '/media/pnkfelix/Rust/rust.git/objdir-opt/pretty-std-collections' (x86_64)
Process 604949 stopped
* thread #1, name = 'pretty-std-coll', stop reason = breakpoint 1.1
    frame #0: 0x0000555555595c82 pretty-std-collections`pretty_std_collections::main::h809023208ec02992 at pretty-std-collections.rs:159:5
   156          hash_set.insert(i);
   157      }
   158
-> 159      zzz(); // #break
            ^
   160  }
   161
   162  fn zzz() {
(lldb) print vec_deque
print vec_deque
error: need to add support for DW_TAG_base_type '()' encoded with DW_ATE = 0x7, bit_size = 0
(alloc::collections::vec_deque::VecDeque<int>) $0 = size=3 {
  [0] = 5
  [1] = 3
  [2] = 7
}
(lldb) print vec_deque2
print vec_deque2
(alloc::collections::vec_deque::VecDeque<int>) $1 = size=7 {
  [0] = 2
  [1] = 3
  [2] = 4
  [3] = 5
  [4] = 6
  [5] = 7
  [6] = 8
}
(lldb) print hash_set
print hash_set
(std::collections::hash::set::HashSet<unsigned long, core::hash::BuildHasherDefault<pretty_std_collections::SimpleHasher> >) $2 = size=0
(lldb) print hash_map
print hash_map
(std::collections::hash::map::HashMap<unsigned long, unsigned long, core::hash::BuildHasherDefault<pretty_std_collections::SimpleHasher> >) $3 = size=0
(lldb) q

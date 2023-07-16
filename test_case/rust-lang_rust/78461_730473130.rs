rust
// lldb-command:print hash_map
// lldbg-check:[...]$2 = size=4 { [0] = { 0 = 1 1 = 10 } [1] = { 0 = 2 1 = 20 } [2] = { 0 = 3 1 = 30 } [3] = { 0 = 4 1 = 40 } }
// lldbr-check:(std::collections::hash::map::HashMap<u64, u64, [...]>) hash_map = size=4 size=4 { [0] = { 0 = 1 1 = 10 } [1] = { 0 = 2 1 = 20 } [2] = { 0 = 3 1 = 30 } [3] = { 0 = 4 1 = 40 } }

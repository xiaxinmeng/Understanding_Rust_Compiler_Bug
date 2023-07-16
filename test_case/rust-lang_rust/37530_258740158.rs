
target/debug/examples/rdr libgoblin.so | grep -A 3 syms
    dynsyms: ElfVec {
        count: 2288,
--
    syms: ElfVec {
        count: 69,
target/debug/examples/rdr libgoblin.so.lto | grep -A 3 syms
    dynsyms: ElfVec {
        count: 113,
--
    syms: ElfVec {
        count: 68,

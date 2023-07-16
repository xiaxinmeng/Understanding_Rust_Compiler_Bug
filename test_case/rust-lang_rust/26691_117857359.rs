 Rust
let mut pmc: PROCESS_MEMORY_COUNTERS = mem::zeroed();
pmc.cb = mem::size_of_val(pmc);
assert!(GetProcessMemoryInfo(GetCurrentProcess(), &mut pmc, pmc.cb) != 0);
return pmc.WorkingSetSize;

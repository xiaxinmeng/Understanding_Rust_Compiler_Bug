
19:56 <@brson> what are the most important metrics for measuring allocator performance?
19:57 < strcat> brson: small allocation speed/overhead, fragmentation (hard to measure, I assume) and lock contention
19:57 < strcat> I guess checking peak memory usage of a rustc build would be a good metric
19:58 < strcat> brson: the core-map test has a good test for the small allocs (treemap)
19:58 < strcat> could also try it with a bunch of threads
20:02 < strcat> brson: no, just binaries linked against librustrt.so
20:02 < strcat> brson: I don't think it would matter much for LLVM because they use their own allocator things

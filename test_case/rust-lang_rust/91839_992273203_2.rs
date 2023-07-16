
$ rustc repro.rs -C opt-level=3
$ ./repro 
[search_in] haystack(55)=[76, 111, 114, 101, 109, 32, 105, 112, 115, 117, 109, 32, 100, 111, 108, 111, 114, 32, 115, 105, 116, 32, 97, 109, 101, 116, 44, 32, 99, 111, 110, 115, 101, 99, 116, 101, 116, 117, 114, 32, 97, 100, 105, 112, 105, 115, 99, 105, 110, 103, 32, 101, 108, 105, 116]
[vector_search_in] haystack(55)=[76, 111, 114, 101, 109, 32, 105, 112, 115, 117, 109, 32, 100, 111, 108, 111, 114, 32, 115, 105, 116, 32, 97, 109, 101, 116, 44, 32, 99, 111, 110, 115, 101, 99, 116, 101, 116, 117, 114, 32, 97, 100, 105, 112, 105, 115, 99, 105, 110, 103, 32, 101, 108, 105, 116], end=45
[vector_search_in] chunk(32)=[76, 111, 114, 101, 109, 32, 105, 112, 115, 117, 109, 32, 100, 111, 108, 111, 114, 32, 115, 105, 116, 32, 97, 109, 101, 116, 44, 32, 99, 111, 110, 115]
[vector_search_in_chunk] first=__m256i(8100041059028070220, 8028914711526208883, 7881616507232526450, 8317708033332114533)
[vector_search_in_chunk] eq_first=__m256i(0, 0, 0, 1095216660480)
[vector_search_in_chunk] eq_last =__m256i(0, 0, 0, 0)
[vector_search_in_chunk] eq      =__m256i(0, 0, 0, 0)
[vector_search_in_chunk] eq      =0
[vector_search_in] remainder: 13
[vector_search_in_chunk] first=__m256i(8388362364150312047, 7142757887439102240, 8387237851300064879, 7597688451221189237)
[vector_search_in_chunk] eq_first=__m256i(0, -72057594037927936, 1095216660480, 0)
[vector_search_in_chunk] eq_last =__m256i(0, 0, 0, 0)
[vector_search_in_chunk] eq      =__m256i(0, 0, 0, 0)
[vector_search_in_chunk] eq      =0
thread 'main' panicked at 'assertion failed: unsafe { searcher.search_in(haystack) }', repro.rs:191:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

plain
[00:41:54]    Compiling idna v0.1.4
[00:41:55]    Compiling toml v0.4.6
[00:41:55]    Compiling futf v0.1.4
[00:41:55]    Compiling phf_codegen v0.7.22
[00:41:56] error[E0433]: failed to resolve. Could not find `arch` in `std`
[00:41:56]  --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/avx2.rs:3:10
[00:41:56]   |
[00:41:56] 3 | use std::arch::x86_64::*;
[00:41:56]   |          ^^^^ Could not find `arch` in `std`
[00:41:56] 
[00:41:56] error[E0433]: failed to resolve. Could not find `arch` in `std`
[00:41:56]  --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/ssse3.rs:3:10
[00:41:56]   |
[00:41:56] 3 | use std::arch::x86_64::*;
[00:41:56]   |          ^^^^ Could not find `arch` in `std`
[00:41:56] 
[00:41:56] error: cannot find macro `is_x86_feature_detected!` in this scope
[00:41:56]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/avx2.rs:11:12
[00:41:56]    |
[00:41:56] 11 |         if is_x86_feature_detected!("avx2") {
[00:41:56] 
[00:41:56] 
[00:41:56] error: cannot find macro `is_x86_feature_detected!` in this scope
[00:41:56]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/ssse3.rs:23:12
[00:41:56]    |
[00:41:56] 23 |         if is_x86_feature_detected!("ssse3") {
[00:41:56] 
[00:41:56] 
[00:41:57] error[E0412]: cannot find type `__m256i` in this scope
[00:41:57]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/avx2.rs:60:13
[00:41:57] 60 |     vector: __m256i,
[00:41:57]    |             ^^^^^^^ not found in this scope
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0425]: cannot find function `_mm256_set1_epi8` in this scope
[00:41:57]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/avx2.rs:67:25
[00:41:57]    |
[00:41:57] 67 |         u8x32 { vector: _mm256_set1_epi8(n as i8) }
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0412]: cannot find type `__m256i` in this scope
[00:41:57]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/avx2.rs:78:55
[00:41:57]    |
[00:41:57] 78 |         let p = slice.as_ptr() as *const u8 as *const __m256i;
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0425]: cannot find function `_mm256_loadu_si256` in this scope
[00:41:57]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/avx2.rs:79:25
[00:41:57]    |
[00:41:57] 79 |         u8x32 { vector: _mm256_loadu_si256(p) }
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0412]: cannot find type `__m256i` in this scope
[00:41:57]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/avx2.rs:91:55
[00:41:57]    |
[00:41:57] 91 |         let p = slice.as_ptr() as *const u8 as *const __m256i;
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0425]: cannot find function `_mm256_load_si256` in this scope
[00:41:57]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/avx2.rs:92:25
[00:41:57]    |
[00:41:57] 92 |         u8x32 { vector: _mm256_load_si256(p) }
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0425]: cannot find function `_mm256_shuffle_epi8` in this scope
[00:41:57]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/avx2.rs:111:29
[00:41:57]     |
[00:41:57] 111 |             u8x32 { vector: _mm256_shuffle_epi8(self.vector, indices.vector) }
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0425]: cannot find function `_mm256_cmpeq_epi8` in this scope
[00:41:57]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/avx2.rs:119:25
[00:41:57]     |
[00:41:57] 119 |             let boolv = _mm256_cmpeq_epi8(self.vector, other.vector);
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0425]: cannot find function `_mm256_set1_epi8` in this scope
[00:41:57]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/avx2.rs:120:24
[00:41:57]     |
[00:41:57] 120 |             let ones = _mm256_set1_epi8(0xFF as u8 as i8);
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0425]: cannot find function `_mm256_andnot_si256` in this scope
[00:41:57]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/avx2.rs:121:29
[00:41:57]     |
[00:41:57] 121 |             u8x32 { vector: _mm256_andnot_si256(boolv, ones) }
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0425]: cannot find function `_mm256_and_si256` in this scope
[00:41:57]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/avx2.rs:129:29
[00:41:57]     |
[00:41:57] 129 |             u8x32 { vector: _mm256_and_si256(self.vector, other.vector) }
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0425]: cannot find function `_mm256_movemask_epi8` in this scope
[00:41:57]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/avx2.rs:137:13
[00:41:57]     |
[00:41:57] 137 |             _mm256_movemask_epi8(self.vector) as u32
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0425]: cannot find function `_mm256_permute2x128_si256` in this scope
[00:41:57]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/avx2.rs:151:21
[00:41:57]     |
[00:41:57] 151 |             let v = _mm256_permute2x128_si256(other.vector, self.vector, 0x21);
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0425]: cannot find function `_mm256_alignr_epi8` in this scope
[00:41:57]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/avx2.rs:152:21
[00:41:57]     |
[00:41:57] 152 |             let v = _mm256_alignr_epi8(self.vector, v, 14);
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0425]: cannot find function `_mm256_permute2x128_si256` in this scope
[00:41:57]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/avx2.rs:167:21
[00:41:57]     |
[00:41:57] 167 |             let v = _mm256_permute2x128_si256(other.vector, self.vector, 0x21);
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0425]: cannot find function `_mm256_alignr_epi8` in this scope
[00:41:57]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/avx2.rs:168:21
[00:41:57]     |
[00:41:57] 168 |             let v = _mm256_alignr_epi8(self.vector, v, 15);
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0425]: cannot find function `_mm256_srli_epi16` in this scope
[00:41:57]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/avx2.rs:177:29
[00:41:57]     |
[00:41:57] 177 |             u8x32 { vector: _mm256_srli_epi16(self.vector, 4) }
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0412]: cannot find type `__m128i` in this scope
[00:41:57]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/ssse3.rs:81:13
[00:41:57] 81 |     vector: __m128i,
[00:41:57]    |             ^^^^^^^ not found in this scope
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0425]: cannot find function `_mm_set1_epi8` in this scope
[00:41:57]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/ssse3.rs:88:25
[00:41:57]    |
[00:41:57] 88 |         u8x16 { vector: _mm_set1_epi8(n as i8) }
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0425]: cannot find function `_mm_loadu_si128` in this scope
[00:41:57]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/ssse3.rs:99:17
[00:41:57]    |
[00:41:57] 99 |         let v = _mm_loadu_si128(slice.as_ptr() as *const u8 as *const __m128i);
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0412]: cannot find type `__m128i` in this scope
[00:41:57]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/ssse3.rs:99:71
[00:41:57]    |
[00:41:57] 99 |         let v = _mm_loadu_si128(slice.as_ptr() as *const u8 as *const __m128i);
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0425]: cannot find function `_mm_load_si128` in this scope
[00:41:57]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/ssse3.rs:112:17
[00:41:57]     |
[00:41:57] 112 |         let v = _mm_load_si128(slice.as_ptr() as *const u8 as *const __m128i);
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0412]: cannot find type `__m128i` in this scope
[00:41:57]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/ssse3.rs:112:70
[00:41:57]     |
[00:41:57] 112 |         let v = _mm_load_si128(slice.as_ptr() as *const u8 as *const __m128i);
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0425]: cannot find function `_mm_shuffle_epi8` in this scope
[00:41:57]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/ssse3.rs:132:29
[00:41:57]     |
[00:41:57] 132 |             u8x16 { vector: _mm_shuffle_epi8(self.vector, indices.vector) }
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0425]: cannot find function `_mm_cmpeq_epi8` in this scope
[00:41:57]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/ssse3.rs:140:25
[00:41:57]     |
[00:41:57] 140 |             let boolv = _mm_cmpeq_epi8(self.vector, other.vector);
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0425]: cannot find function `_mm_set1_epi8` in this scope
[00:41:57]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/ssse3.rs:141:24
[00:41:57]     |
[00:41:57] 141 |             let ones = _mm_set1_epi8(0xFF as u8 as i8);
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0425]: cannot find function `_mm_andnot_si128` in this scope
[00:41:57]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/ssse3.rs:142:29
[00:41:57]     |
[00:41:57] 142 |             u8x16 { vector: _mm_andnot_si128(boolv, ones) }
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0425]: cannot find function `_mm_and_si128` in this scope
[00:41:57]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/ssse3.rs:150:29
[00:41:57]     |
[00:41:57] 150 |             u8x16 { vector: _mm_and_si128(self.vector, other.vector) }
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0425]: cannot find function `_mm_movemask_epi8` in this scope
[00:41:57]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/ssse3.rs:158:13
[00:41:57]     |
[00:41:57] 158 |             _mm_movemask_epi8(self.vector) as u32
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0425]: cannot find function `_mm_alignr_epi8` in this scope
[00:41:57]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/ssse3.rs:166:29
[00:41:57]     |
[00:41:57] 166 |             u8x16 { vector: _mm_alignr_epi8(self.vector, other.vector, 14) }
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0425]: cannot find function `_mm_alignr_epi8` in this scope
[00:41:57]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/ssse3.rs:174:29
[00:41:57]     |
[00:41:57] 174 |             u8x16 { vector: _mm_alignr_epi8(self.vector, other.vector, 15) }
[00:41:57] 
[00:41:57] 
[00:41:57] error[E0425]: cannot find function `_mm_srli_epi16` in this scope
[00:41:57]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/vector/ssse3.rs:182:29
[00:41:57]     |
[00:41:57] 182 |             u8x16 { vector: _mm_srli_epi16(self.vector, 4) }
[00:41:57] 
[00:41:57] error: aborting due to 36 previous errors
[00:41:57] 
[00:41:57] Some errors occurred: E0412, E0425, E0433.
[00:41:57] Some errors occurred: E0412, E0425, E0433.
[00:41:57] For more information about an error, try `rustc --explain E0412`.
[00:41:57] error: Could not compile `regex`.
[00:41:57] 
[00:41:57] Caused by:
[00:41:57]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name regex /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.0.1/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 --cfg feature="default" --cfg feature="use_std" -C metadata=1e87609f9aded9d3 -C extra-filename=-1e87609f9aded9d3 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/release/deps --extern aho_corasick=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-310954d562c0f985.rlib --extern utf8_ranges=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libutf8_ranges-d76a166c51c7310b.rlib --extern regex_syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libregex_syntax-2ad6139f87894413.rlib --extern thread_local=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libthread_local-6de2f84028b38fe5.rlib --extern memchr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libmemchr-7c291a44722df336.rlib --cap-lints allow --cfg regex_runtime_teddy_ssse3 --cfg regex_runtime_teddy_avx2` (exit code: 101)
      4.0K     0  4.0K   0% /sys/fs/cgroup
none            5.0M     0  5.0M   0% /run/lock
none            7.4G     0  7.4G   0% /run/shm
none            100M     0  100M   0% /run/user

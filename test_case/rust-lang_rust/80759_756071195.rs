plain
   Compiling tinyvec v0.3.4
error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:186:17
    |
180 | / macro_rules! rotr_32 {
181 | |     ($name:ident, $i:expr) => {
182 | |     #[inline(always)]
183 | |     fn $name(self) -> Self {
...   |
186 | |                 _mm_srli_epi32(self.x, $i as i32),
...   |
191 | |     };
192 | | }
192 | | }
    | |_- in this expansion of `rotr_32!`
193 |   impl<S4: Copy, NI: Copy> RotateEachWord32 for u32x4_sse2<YesS3, S4, NI> {
194 |       rotr_32!(rotate_each_word_right7, 7);

error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:187:17
    |
    |
180 | / macro_rules! rotr_32 {
181 | |     ($name:ident, $i:expr) => {
182 | |     #[inline(always)]
183 | |     fn $name(self) -> Self {
...   |
187 | |                 _mm_slli_epi32(self.x, 32 - $i as i32),
...   |
191 | |     };
192 | | }
192 | | }
    | |_- in this expansion of `rotr_32!`
193 |   impl<S4: Copy, NI: Copy> RotateEachWord32 for u32x4_sse2<YesS3, S4, NI> {
194 |       rotr_32!(rotate_each_word_right7, 7);

   Compiling regex-syntax v0.6.18
error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:186:17
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:186:17
    |
180 | / macro_rules! rotr_32 {
181 | |     ($name:ident, $i:expr) => {
182 | |     #[inline(always)]
183 | |     fn $name(self) -> Self {
...   |
186 | |                 _mm_srli_epi32(self.x, $i as i32),
...   |
191 | |     };
192 | | }
192 | | }
    | |_- in this expansion of `rotr_32!`
...
200 |       rotr_32!(rotate_each_word_right11, 11);

error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:187:17
    |
    |
180 | / macro_rules! rotr_32 {
181 | |     ($name:ident, $i:expr) => {
182 | |     #[inline(always)]
183 | |     fn $name(self) -> Self {
...   |
187 | |                 _mm_slli_epi32(self.x, 32 - $i as i32),
...   |
191 | |     };
192 | | }
192 | | }
    | |_- in this expansion of `rotr_32!`
...
200 |       rotr_32!(rotate_each_word_right11, 11);

error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:186:17
    |
    |
180 | / macro_rules! rotr_32 {
181 | |     ($name:ident, $i:expr) => {
182 | |     #[inline(always)]
183 | |     fn $name(self) -> Self {
...   |
186 | |                 _mm_srli_epi32(self.x, $i as i32),
...   |
191 | |     };
192 | | }
192 | | }
    | |_- in this expansion of `rotr_32!`
...
201 |       rotr_32!(rotate_each_word_right12, 12);

error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:187:17
    |
    |
180 | / macro_rules! rotr_32 {
181 | |     ($name:ident, $i:expr) => {
182 | |     #[inline(always)]
183 | |     fn $name(self) -> Self {
...   |
187 | |                 _mm_slli_epi32(self.x, 32 - $i as i32),
...   |
191 | |     };
192 | | }
192 | | }
    | |_- in this expansion of `rotr_32!`
...
201 |       rotr_32!(rotate_each_word_right12, 12);

error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:186:17
    |
    |
180 | / macro_rules! rotr_32 {
181 | |     ($name:ident, $i:expr) => {
182 | |     #[inline(always)]
183 | |     fn $name(self) -> Self {
...   |
186 | |                 _mm_srli_epi32(self.x, $i as i32),
...   |
191 | |     };
192 | | }
192 | | }
    | |_- in this expansion of `rotr_32!`
...
207 |       rotr_32!(rotate_each_word_right20, 20);

error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:187:17
    |
    |
180 | / macro_rules! rotr_32 {
181 | |     ($name:ident, $i:expr) => {
182 | |     #[inline(always)]
183 | |     fn $name(self) -> Self {
...   |
187 | |                 _mm_slli_epi32(self.x, 32 - $i as i32),
...   |
191 | |     };
192 | | }
192 | | }
    | |_- in this expansion of `rotr_32!`
...
207 |       rotr_32!(rotate_each_word_right20, 20);

error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:186:17
    |
    |
180 | / macro_rules! rotr_32 {
181 | |     ($name:ident, $i:expr) => {
182 | |     #[inline(always)]
183 | |     fn $name(self) -> Self {
...   |
186 | |                 _mm_srli_epi32(self.x, $i as i32),
...   |
191 | |     };
192 | | }
192 | | }
    | |_- in this expansion of `rotr_32!`
...
213 |       rotr_32!(rotate_each_word_right25, 25);

error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:187:17
    |
    |
180 | / macro_rules! rotr_32 {
181 | |     ($name:ident, $i:expr) => {
182 | |     #[inline(always)]
183 | |     fn $name(self) -> Self {
...   |
187 | |                 _mm_slli_epi32(self.x, 32 - $i as i32),
...   |
191 | |     };
192 | | }
192 | | }
    | |_- in this expansion of `rotr_32!`
...
213 |       rotr_32!(rotate_each_word_right25, 25);

error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:186:17
    |
    |
180 | / macro_rules! rotr_32 {
181 | |     ($name:ident, $i:expr) => {
182 | |     #[inline(always)]
183 | |     fn $name(self) -> Self {
...   |
186 | |                 _mm_srli_epi32(self.x, $i as i32),
...   |
191 | |     };
192 | | }
192 | | }
    | |_- in this expansion of `rotr_32!`
...
216 |       rotr_32!(rotate_each_word_right7, 7);

error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:187:17
    |
    |
180 | / macro_rules! rotr_32 {
181 | |     ($name:ident, $i:expr) => {
182 | |     #[inline(always)]
183 | |     fn $name(self) -> Self {
...   |
187 | |                 _mm_slli_epi32(self.x, 32 - $i as i32),
...   |
191 | |     };
192 | | }
192 | | }
    | |_- in this expansion of `rotr_32!`
...
216 |       rotr_32!(rotate_each_word_right7, 7);

error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:186:17
    |
    |
180 | / macro_rules! rotr_32 {
181 | |     ($name:ident, $i:expr) => {
182 | |     #[inline(always)]
183 | |     fn $name(self) -> Self {
...   |
186 | |                 _mm_srli_epi32(self.x, $i as i32),
...   |
191 | |     };
192 | | }
192 | | }
    | |_- in this expansion of `rotr_32!`
...
217 |       rotr_32!(rotate_each_word_right8, 8);

error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:187:17
    |
    |
180 | / macro_rules! rotr_32 {
181 | |     ($name:ident, $i:expr) => {
182 | |     #[inline(always)]
183 | |     fn $name(self) -> Self {
...   |
187 | |                 _mm_slli_epi32(self.x, 32 - $i as i32),
...   |
191 | |     };
192 | | }
192 | | }
    | |_- in this expansion of `rotr_32!`
...
217 |       rotr_32!(rotate_each_word_right8, 8);

error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:186:17
    |
    |
180 | / macro_rules! rotr_32 {
181 | |     ($name:ident, $i:expr) => {
182 | |     #[inline(always)]
183 | |     fn $name(self) -> Self {
...   |
186 | |                 _mm_srli_epi32(self.x, $i as i32),
...   |
191 | |     };
192 | | }
192 | | }
    | |_- in this expansion of `rotr_32!`
...
218 |       rotr_32!(rotate_each_word_right11, 11);

error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:187:17
    |
    |
180 | / macro_rules! rotr_32 {
181 | |     ($name:ident, $i:expr) => {
182 | |     #[inline(always)]
183 | |     fn $name(self) -> Self {
...   |
187 | |                 _mm_slli_epi32(self.x, 32 - $i as i32),
...   |
191 | |     };
192 | | }
192 | | }
    | |_- in this expansion of `rotr_32!`
...
218 |       rotr_32!(rotate_each_word_right11, 11);

error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:186:17
    |
    |
180 | / macro_rules! rotr_32 {
181 | |     ($name:ident, $i:expr) => {
182 | |     #[inline(always)]
183 | |     fn $name(self) -> Self {
...   |
186 | |                 _mm_srli_epi32(self.x, $i as i32),
...   |
191 | |     };
192 | | }
192 | | }
    | |_- in this expansion of `rotr_32!`
...
219 |       rotr_32!(rotate_each_word_right12, 12);

error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:187:17
    |
    |
180 | / macro_rules! rotr_32 {
181 | |     ($name:ident, $i:expr) => {
182 | |     #[inline(always)]
183 | |     fn $name(self) -> Self {
...   |
187 | |                 _mm_slli_epi32(self.x, 32 - $i as i32),
...   |
191 | |     };
192 | | }
192 | | }
    | |_- in this expansion of `rotr_32!`
...
219 |       rotr_32!(rotate_each_word_right12, 12);

error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:186:17
    |
    |
180 | / macro_rules! rotr_32 {
181 | |     ($name:ident, $i:expr) => {
182 | |     #[inline(always)]
183 | |     fn $name(self) -> Self {
...   |
186 | |                 _mm_srli_epi32(self.x, $i as i32),
...   |
191 | |     };
192 | | }
192 | | }
    | |_- in this expansion of `rotr_32!`
...
224 |       rotr_32!(rotate_each_word_right20, 20);

error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:187:17
    |
    |
180 | / macro_rules! rotr_32 {
181 | |     ($name:ident, $i:expr) => {
182 | |     #[inline(always)]
183 | |     fn $name(self) -> Self {
...   |
187 | |                 _mm_slli_epi32(self.x, 32 - $i as i32),
...   |
191 | |     };
192 | | }
192 | | }
    | |_- in this expansion of `rotr_32!`
...
224 |       rotr_32!(rotate_each_word_right20, 20);

error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:186:17
    |
    |
180 | / macro_rules! rotr_32 {
181 | |     ($name:ident, $i:expr) => {
182 | |     #[inline(always)]
183 | |     fn $name(self) -> Self {
...   |
186 | |                 _mm_srli_epi32(self.x, $i as i32),
...   |
191 | |     };
192 | | }
192 | | }
    | |_- in this expansion of `rotr_32!`
...
225 |       rotr_32!(rotate_each_word_right24, 24);

error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:187:17
    |
    |
180 | / macro_rules! rotr_32 {
181 | |     ($name:ident, $i:expr) => {
182 | |     #[inline(always)]
183 | |     fn $name(self) -> Self {
...   |
187 | |                 _mm_slli_epi32(self.x, 32 - $i as i32),
...   |
191 | |     };
192 | | }
192 | | }
    | |_- in this expansion of `rotr_32!`
...
225 |       rotr_32!(rotate_each_word_right24, 24);

error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:186:17
    |
    |
180 | / macro_rules! rotr_32 {
181 | |     ($name:ident, $i:expr) => {
182 | |     #[inline(always)]
183 | |     fn $name(self) -> Self {
...   |
186 | |                 _mm_srli_epi32(self.x, $i as i32),
...   |
191 | |     };
192 | | }
192 | | }
    | |_- in this expansion of `rotr_32!`
...
226 |       rotr_32!(rotate_each_word_right25, 25);

error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:187:17
    |
    |
180 | / macro_rules! rotr_32 {
181 | |     ($name:ident, $i:expr) => {
182 | |     #[inline(always)]
183 | |     fn $name(self) -> Self {
...   |
187 | |                 _mm_slli_epi32(self.x, 32 - $i as i32),
...   |
191 | |     };
192 | | }
192 | | }
    | |_- in this expansion of `rotr_32!`
...
226 |       rotr_32!(rotate_each_word_right25, 25);

error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:248:17
    |
    |
242 | / macro_rules! rotr_64 {
243 | |     ($name:ident, $i:expr) => {
244 | |     #[inline(always)]
245 | |     fn $name(self) -> Self {
...   |
248 | |                 _mm_srli_epi64(self.x, $i as i32),
...   |
253 | |     };
254 | | }
254 | | }
    | |_- in this expansion of `rotr_64!`
255 |   impl<S4: Copy, NI: Copy> RotateEachWord32 for u64x2_sse2<YesS3, S4, NI> {
256 |       rotr_64!(rotate_each_word_right7, 7);

error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:249:17
    |
    |
242 | / macro_rules! rotr_64 {
243 | |     ($name:ident, $i:expr) => {
244 | |     #[inline(always)]
245 | |     fn $name(self) -> Self {
...   |
249 | |                 _mm_slli_epi64(self.x, 64 - $i as i32),
...   |
253 | |     };
254 | | }
254 | | }
    | |_- in this expansion of `rotr_64!`
255 |   impl<S4: Copy, NI: Copy> RotateEachWord32 for u64x2_sse2<YesS3, S4, NI> {
256 |       rotr_64!(rotate_each_word_right7, 7);

error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:248:17
    |
    |
242 | / macro_rules! rotr_64 {
243 | |     ($name:ident, $i:expr) => {
244 | |     #[inline(always)]
245 | |     fn $name(self) -> Self {
...   |
248 | |                 _mm_srli_epi64(self.x, $i as i32),
...   |
253 | |     };
254 | | }
254 | | }
    | |_- in this expansion of `rotr_64!`
...
262 |       rotr_64!(rotate_each_word_right11, 11);

error: argument 2 is required to be a constant
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ppv-lite86-0.2.8/src/x86_64/sse2.rs:249:17
    |
    |
242 | / macro_rules! rotr_64 {
243 | |     ($name:ident, $i:expr) => {
244 | |     #[inline(always)]
245 | |     fn $name(self) -> Self {
...   |
249 | |                 _mm_slli_epi64(self.x, 64 - $i as i32),
...   |
253 | |     };
254 | | }

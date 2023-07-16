rust
mod intrinsics {
    extern "rust-intrinsic" {
        pub fn wrapping_add<T>(a: T, b: T) -> T;
        pub fn rotate_left<T>(a: T, b: T) -> T;
        pub fn rotate_right<T>(a: T, b: T) -> T;
        pub fn offset<T>(ptr: *const T, count: isize) -> *const T;
    }
}

mod mem {
    extern "rust-intrinsic" {
        fn transmute<T, U>(_: T) -> U;
        fn size_of<T>() -> usize;
    }
}

macro_rules! impl_uint {
    ($($ty:ident = $lang:literal),*) => {
        $(
            impl $ty {
                pub fn wrapping_add(self, rhs: Self) -> Self {
                    // intrinsics::wrapping_add(self, rhs)
                    self + rhs
                }

                pub fn rotate_left(self, n: u32) -> Self {
                    unsafe {
                        intrinsics::rotate_left(self, n as Self)
                    }
                }

                pub fn rotate_right(self, n: u32) -> Self {
                    unsafe {
                        intrinsics::rotate_right(self, n as Self)
                    }
                }

                pub fn to_le(self) -> Self {
                    #[cfg(target_endian = "little")]
                    {
                        self
                    }
                }

                pub const fn from_le_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
                    Self::from_le(Self::from_ne_bytes(bytes))
                }

                pub const fn from_le(x: Self) -> Self {
                    #[cfg(target_endian = "little")]
                    {
                        x
                    }
                }

                pub const fn from_ne_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
                    unsafe { mem::transmute(bytes) }
                }
            }
        )*
    }
}

impl_uint!(
    u8 = "u8",
    u16 = "u16",
    u32 = "u32",
    u64 = "u64",
    u128 = "u128",
    usize = "usize"
);

mod cmp {
    pub fn min(a: usize, b: usize) -> usize {
        if a < b {
            a
        } else {
            b
        }
    }
}

extern "C" {
    fn printf(s: *const i8, ...);
}

struct FatPtr<T> {
    data: *const T,
    len: usize,
}

pub union Repr<T> {
    rust: *const [T],
    rust_mut: *mut [T],
    raw: FatPtr<T>,
}

pub enum Option<T> {
    None,
    Some(T),
}

#[lang = "RangeFull"]
pub struct RangeFull;

#[lang = "Range"]
pub struct Range<Idx> {
    pub start: Idx,
    pub end: Idx,
}

#[lang = "RangeFrom"]
pub struct RangeFrom<Idx> {
    pub start: Idx,
}

#[lang = "RangeTo"]
pub struct RangeTo<Idx> {
    pub end: Idx,
}

#[lang = "RangeInclusive"]
pub struct RangeInclusive<Idx> {
    pub start: Idx,
    pub end: Idx,
}

#[lang = "const_slice_ptr"]
impl<T> *const [T] {
    pub const fn len(self) -> usize {
        let a = unsafe { Repr { rust: self }.raw };
        a.len
    }

    pub const fn as_ptr(self) -> *const T {
        self as *const T
    }
}

#[lang = "const_ptr"]
impl<T> *const T {
    pub const unsafe fn offset(self, count: isize) -> *const T {
        unsafe { intrinsics::offset(self, count) }
    }

    pub const unsafe fn add(self, count: usize) -> Self {
        unsafe { self.offset(count as isize) }
    }

    pub const fn as_ptr(self) -> *const T {
        self as *const T
    }
}

const fn slice_from_raw_parts<T>(data: *const T, len: usize) -> *const [T] {
    unsafe {
        Repr {
            raw: FatPtr { data, len },
        }
        .rust
    }
}

#[lang = "index"]
trait Index<Idx> {
    type Output;

    fn index(&self, index: Idx) -> &Self::Output;
}

impl<T> [T] {
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub const fn len(&self) -> usize {
        unsafe { Repr { rust: self }.raw.len }
    }
}

pub unsafe trait SliceIndex<T> {
    type Output;

    fn get(self, slice: &T) -> Option<&Self::Output>;

    unsafe fn get_unchecked(self, slice: *const T) -> *const Self::Output;

    fn index(self, slice: &T) -> &Self::Output;
}

unsafe impl<T> SliceIndex<[T]> for usize {
    type Output = T;

    fn get(self, slice: &[T]) -> Option<&T> {
        unsafe { Option::Some(&*self.get_unchecked(slice)) }
    }

    unsafe fn get_unchecked(self, slice: *const [T]) -> *const T {
        // SAFETY: the caller guarantees that `slice` is not dangling, so it
        // cannot be longer than `isize::MAX`. They also guarantee that
        // `self` is in bounds of `slice` so `self` cannot overflow an `isize`,
        // so the call to `add` is safe.
        unsafe { slice.as_ptr().add(self) }
    }

    fn index(self, slice: &[T]) -> &T {
        unsafe { &*self.get_unchecked(slice) }
    }
}

unsafe impl<T> SliceIndex<[T]> for Range<usize> {
    type Output = [T];

    fn get(self, slice: &[T]) -> Option<&[T]> {
        if self.start > self.end || self.end > slice.len() {
            Option::None
        } else {
            unsafe { Option::Some(&*self.get_unchecked(slice)) }
        }
    }

    unsafe fn get_unchecked(self, slice: *const [T]) -> *const [T] {
        unsafe {
            let a: *const T = slice.as_ptr();
            let b: *const T = a.add(self.start);
            slice_from_raw_parts(b, self.end - self.start)
        }
    }

    fn index(self, slice: &[T]) -> &[T] {
        unsafe { &*self.get_unchecked(slice) }
    }
}

// issue-1270
// unsafe impl<T> SliceIndex<[T]> for RangeTo<usize> {
//     type Output = [T];

//     fn get(self, slice: &[T]) -> Option<&[T]> {
//         (0..self.end).get(slice)
//     }

//     unsafe fn get_unchecked(self, slice: *const [T]) -> *const [T] {
//         // SAFETY: the caller has to uphold the safety contract for `get_unchecked`.
//         unsafe { (0..self.end).get_unchecked(slice) }
//     }

//     fn index(self, slice: &[T]) -> &[T] {
//         (0..self.end).index(slice)
//     }
// }

// #[stable(feature = "slice_get_slice_impls", since = "1.15.0")]
// unsafe impl<T> SliceIndex<[T]> for RangeFrom<usize> {
//     type Output = [T];

//     fn get(self, slice: &[T]) -> Option<&[T]> {
//         (self.start..slice.len()).get(slice)
//     }

//     unsafe fn get_unchecked(self, slice: *const [T]) -> *const [T] {
//         // SAFETY: the caller has to uphold the safety contract for `get_unchecked`.
//         unsafe { (self.start..slice.len()).get_unchecked(slice) }
//     }

//     fn index(self, slice: &[T]) -> &[T] {
//         unsafe { &*self.get_unchecked(slice) }
//     }
// }

unsafe impl<T> SliceIndex<[T]> for RangeFull {
    type Output = [T];

    fn get(self, slice: &[T]) -> Option<&[T]> {
        Option::Some(slice)
    }

    unsafe fn get_unchecked(self, slice: *const [T]) -> *const [T] {
        slice
    }

    fn index(self, slice: &[T]) -> &[T] {
        slice
    }
}

// unsafe impl<T> SliceIndex<[T]> for RangeToInclusive<usize> {
//     type Output = [T];

//     fn get(self, slice: &[T]) -> Option<&[T]> {
//         (0..=self.end).get(slice)
//     }

//     unsafe fn get_unchecked(self, slice: *const [T]) -> *const [T] {
//         // SAFETY: the caller has to uphold the safety contract for `get_unchecked`.
//         unsafe { (0..=self.end).get_unchecked(slice) }
//     }

//     fn index(self, slice: &[T]) -> &[T] {
//         (0..=self.end).index(slice)
//     }
// }

impl<T, I> Index<I> for [T]
where
    I: SliceIndex<[T]>,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &I::Output {
        index.index(self)
    }
}

// end prelude from libcore

const OUT_LEN: usize = 32;
const KEY_LEN: usize = 32;
const BLOCK_LEN: usize = 64;
const CHUNK_LEN: usize = 1024;

const CHUNK_START: u32 = 1 << 0;
const CHUNK_END: u32 = 1 << 1;
const PARENT: u32 = 1 << 2;
const ROOT: u32 = 1 << 3;
const KEYED_HASH: u32 = 1 << 4;
const DERIVE_KEY_CONTEXT: u32 = 1 << 5;
const DERIVE_KEY_MATERIAL: u32 = 1 << 6;

const IV: [u32; 8] = [
    0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
];

const MSG_PERMUTATION: [usize; 16] = [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8];

// The mixing function, G, which mixes either a column or a diagonal.
fn g(state: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize, mx: u32, my: u32) {
    state[a] = state[a].wrapping_add(state[b]).wrapping_add(mx);
    state[d] = (state[d] ^ state[a]).rotate_right(16);
    state[c] = state[c].wrapping_add(state[d]);
    state[b] = (state[b] ^ state[c]).rotate_right(12);
    state[a] = state[a].wrapping_add(state[b]).wrapping_add(my);
    state[d] = (state[d] ^ state[a]).rotate_right(8);
    state[c] = state[c].wrapping_add(state[d]);
    state[b] = (state[b] ^ state[c]).rotate_right(7);
}

fn round(state: &mut [u32; 16], m: &[u32; 16]) {
    // Mix the columns.
    g(state, 0, 4, 8, 12, m[0], m[1]);
    g(state, 1, 5, 9, 13, m[2], m[3]);
    g(state, 2, 6, 10, 14, m[4], m[5]);
    g(state, 3, 7, 11, 15, m[6], m[7]);
    // Mix the diagonals.
    g(state, 0, 5, 10, 15, m[8], m[9]);
    g(state, 1, 6, 11, 12, m[10], m[11]);
    g(state, 2, 7, 8, 13, m[12], m[13]);
    g(state, 3, 4, 9, 14, m[14], m[15]);
}

fn permute(m: &mut [u32; 16]) {
    let mut permuted = [0; 16];
    // for i in 0..16 {
    //     permuted[i] = m[MSG_PERMUTATION[i]];
    // }
    *m = permuted;
}

fn compress(
    chaining_value: &[u32; 8],
    block_words: &[u32; 16],
    counter: u64,
    block_len: u32,
    flags: u32,
) -> [u32; 16] {
    let mut state = [
        chaining_value[0],
        chaining_value[1],
        chaining_value[2],
        chaining_value[3],
        chaining_value[4],
        chaining_value[5],
        chaining_value[6],
        chaining_value[7],
        IV[0],
        IV[1],
        IV[2],
        IV[3],
        counter as u32,
        (counter >> 32) as u32,
        block_len,
        flags,
    ];
    let mut block = *block_words;

    round(&mut state, &block); // round 1
    permute(&mut block);
    round(&mut state, &block); // round 2
    permute(&mut block);
    round(&mut state, &block); // round 3
    permute(&mut block);
    round(&mut state, &block); // round 4
    permute(&mut block);
    round(&mut state, &block); // round 5
    permute(&mut block);
    round(&mut state, &block); // round 6
    permute(&mut block);
    round(&mut state, &block); // round 7

    // for i in 0..8 {
    //     state[i] ^= state[i + 8];
    //     state[i + 8] ^= chaining_value[i];
    // }
    state
}

fn words_from_little_endian_bytes(bytes: &[u8], words: &mut [u32]) {
    // debug_assert_eq!(bytes.len(), 4 * words.len());
    // for (four_bytes, word) in bytes.chunks_exact(4).zip(words) {
    //     *word = u32::from_le_bytes(four_bytes.try_into().unwrap());
    // }
}

// Each chunk or parent node can produce either an 8-word chaining value or, by
// setting the ROOT flag, any number of final output bytes. The Output struct
// captures the state just prior to choosing between those two possibilities.
struct Output {
    input_chaining_value: [u32; 8],
    block_words: [u32; 16],
    counter: u64,
    block_len: u32,
    flags: u32,
}

impl Output {
    fn root_output_bytes(&self, out_slice: &mut [u8]) {
        let mut output_block_counter = 0;
        // for out_block in out_slice.chunks_mut(2 * OUT_LEN) {
        //     let words = compress(
        //         &self.input_chaining_value,
        //         &self.block_words,
        //         output_block_counter,
        //         self.block_len,
        //         self.flags | ROOT,
        //     );
        //     // The output length might not be a multiple of 4.
        //     for (word, out_word) in words.iter().zip(out_block.chunks_mut(4)) {
        //         out_word.copy_from_slice(&word.to_le_bytes()[..out_word.len()]);
        //     }
        //     output_block_counter += 1;
        // }
    }
}

struct ChunkState {
    chaining_value: [u32; 8],
    chunk_counter: u64,
    block: [u8; BLOCK_LEN],
    block_len: u8,
    blocks_compressed: u8,
    flags: u32,
}

impl ChunkState {
    fn new(key_words: [u32; 8], chunk_counter: u64, flags: u32) -> Self {
        Self {
            chaining_value: key_words,
            chunk_counter,
            block: [0; BLOCK_LEN],
            block_len: 0,
            blocks_compressed: 0,
            flags,
        }
    }

    fn len(&self) -> usize {
        BLOCK_LEN * self.blocks_compressed as usize + self.block_len as usize
    }

    fn start_flag(&self) -> u32 {
        if self.blocks_compressed == 0 {
            CHUNK_START
        } else {
            0
        }
    }

    fn update(&mut self, mut input: &[u8]) {
        while !input.is_empty() {
            // If the block buffer is full, compress it and clear it. More
            // input is coming, so this compression is not CHUNK_END.
            if self.block_len as usize == BLOCK_LEN {
                let mut block_words = [0; 16];
                words_from_little_endian_bytes(&self.block, &mut block_words);

                self.blocks_compressed += 1;
                self.block = [0; BLOCK_LEN];
                self.block_len = 0;
            }

            // Copy input bytes into the block buffer.
            let want = BLOCK_LEN - self.block_len as usize;
            let take = cmp::min(want, input.len());

            // FIXME issue-1270 range-from range-to
            //
            // self.block[self.block_len as usize..][..take].copy_from_slice(&input[..take]);
            self.block_len += take as u8;
            // input = &input[take..];
        }
    }

    fn output(&self) -> Output {
        let mut block_words = [0; 16];
        words_from_little_endian_bytes(&self.block, &mut block_words);
        Output {
            input_chaining_value: self.chaining_value,
            block_words,
            counter: self.chunk_counter,
            block_len: self.block_len as u32,
            flags: self.flags | self.start_flag() | CHUNK_END,
        }
    }
}

fn parent_output(
    left_child_cv: [u32; 8],
    right_child_cv: [u32; 8],
    key_words: [u32; 8],
    flags: u32,
) -> Output {
    let mut block_words = [0; 16];

    // FIXME issue-1270 range-from range-to
    // block_words[..8].copy_from_slice(&left_child_cv);
    // block_words[8..].copy_from_slice(&right_child_cv);
    Output {
        input_chaining_value: key_words,
        block_words,
        counter: 0,                  // Always 0 for parent nodes.
        block_len: BLOCK_LEN as u32, // Always BLOCK_LEN (64) for parent nodes.
        flags: PARENT | flags,
    }
}

/// An incremental hasher that can accept any number of writes.
pub struct Hasher {
    chunk_state: ChunkState,
    key_words: [u32; 8],
    cv_stack: [[u32; 8]; 54], // Space for 54 subtree chaining values:
    cv_stack_len: u8,         // 2^54 * CHUNK_LEN = 2^64
    flags: u32,
}

impl Hasher {
    fn new_internal(key_words: [u32; 8], flags: u32) -> Self {
        Self {
            chunk_state: ChunkState::new(key_words, 0, flags),
            key_words,
            cv_stack: [[0; 8]; 54],
            cv_stack_len: 0,
            flags,
        }
    }

    /// Construct a new `Hasher` for the regular hash function.
    pub fn new() -> Self {
        Self::new_internal(IV, 0)
    }

    /// Construct a new `Hasher` for the keyed hash function.
    pub fn new_keyed(key: &[u8; KEY_LEN]) -> Self {
        let mut key_words = [0; 8];
        words_from_little_endian_bytes(key, &mut key_words);
        Self::new_internal(key_words, KEYED_HASH)
    }

    fn push_stack(&mut self, cv: [u32; 8]) {
        self.cv_stack[self.cv_stack_len as usize] = cv;
        self.cv_stack_len += 1;
    }

    fn pop_stack(&mut self) -> [u32; 8] {
        self.cv_stack_len -= 1;
        self.cv_stack[self.cv_stack_len as usize]
    }

    // Section 5.1.2 of the BLAKE3 spec explains this algorithm in more detail.
    fn add_chunk_chaining_value(&mut self, mut new_cv: [u32; 8], mut total_chunks: u64) {
        // This chunk might complete some subtrees. For each completed subtree,
        // its left child will be the current top entry in the CV stack, and
        // its right child will be the current value of `new_cv`. Pop each left
        // child off the stack, merge it with `new_cv`, and overwrite `new_cv`
        // with the result. After all these merges, push the final value of
        // `new_cv` onto the stack. The number of completed subtrees is given
        // by the number of trailing 0-bits in the new total number of chunks.
        while total_chunks & 1 == 0 {
            // new_cv = parent_cv(self.pop_stack(), new_cv, self.key_words, self.flags);
            total_chunks >>= 1;
        }
        self.push_stack(new_cv);
    }
}


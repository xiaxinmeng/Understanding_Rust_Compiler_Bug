Rust
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

trait Vector: Copy + std::fmt::Debug {
    const LANES: usize;

    unsafe fn set1_epi8(a: i8) -> Self;

    unsafe fn loadu_si(a: *const u8) -> Self;

    unsafe fn cmpeq_epi8(a: Self, b: Self) -> Self;

    unsafe fn and_si(a: Self, b: Self) -> Self;

    unsafe fn movemask_epi8(a: Self) -> i32;
}

impl Vector for __m256i {
    const LANES: usize = 32;

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn set1_epi8(a: i8) -> Self {
        _mm256_set1_epi8(a)
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn loadu_si(a: *const u8) -> Self {
        _mm256_loadu_si256(a as *const Self)
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn cmpeq_epi8(a: Self, b: Self) -> Self {
        _mm256_cmpeq_epi8(a, b)
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn and_si(a: Self, b: Self) -> Self {
        _mm256_and_si256(a, b)
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn movemask_epi8(a: Self) -> i32 {
        _mm256_movemask_epi8(a)
    }
}

struct VectorHash<V: Vector> {
    first: V,
    last: V,
}

impl<V: Vector> VectorHash<V> {
    unsafe fn new(first: u8, last: u8) -> Self {
        Self {
            first: Vector::set1_epi8(first as i8),
            last: Vector::set1_epi8(last as i8),
        }
    }
}

pub struct Avx2Searcher {
    position: usize,
    avx2_hash: VectorHash<__m256i>,
    needle_len: usize,
}

impl Avx2Searcher {
    #[target_feature(enable = "avx2")]
    pub unsafe fn with_position(needle: &[u8], position: usize) -> Self {
        // Implicitly checks that the needle is not empty because position is an
        // unsized integer.
        assert!(position < needle.len());

        let avx2_hash = VectorHash::new(needle[0], needle[position]);

        Self {
            position,
            avx2_hash,
            needle_len: needle.len(),
        }
    }

    /// Performs a substring search for the `needle` within `haystack`.
    #[target_feature(enable = "avx2")]
    pub unsafe fn search_in(&self, haystack: &[u8]) -> bool {
        println!("[search_in] haystack({})={:?}", haystack.len(), haystack);

        let end = haystack.len() - self.needle_len + 1;

        self.vector_search_in(haystack, end, &self.avx2_hash)
    }
}

trait Searcher {
    fn position(&self) -> usize;

    #[inline]
    unsafe fn vector_search_in_chunk<V: Vector>(
        &self,
        hash: &VectorHash<V>,
        start: *const u8,
        mask: i32,
    ) -> bool {
        let first = Vector::loadu_si(start);
        let last = Vector::loadu_si(start.add(self.position()));

        // Uncommenting the following lines makes it work
        // println!("[vector_search_in_chunk] hash.first={:?}", hash.first);
        println!("[vector_search_in_chunk] first={:?}", first);
        let eq_first = Vector::cmpeq_epi8(hash.first, first);
        println!(
            "[vector_search_in_chunk] eq_first={:?}",
            eq_first
        );
        let eq_last = Vector::cmpeq_epi8(hash.last, last);
        println!(
            "[vector_search_in_chunk] eq_last ={:?}",
            eq_last
        );

        let eq = Vector::and_si(eq_first, eq_last);
        println!(
            "[vector_search_in_chunk] eq      ={:?}",
            eq
        );
        let eq = (Vector::movemask_epi8(eq) & mask) as u32;
        println!(
        	"[vector_search_in_chunk] eq      ={:?}",
        	eq
        );

        eq != 0
    }

    #[inline]
    // Uncommenting the following lines makes it work
    // #[target_feature(enable = "avx2")]
    unsafe fn vector_search_in<V: Vector>(
        &self,
        haystack: &[u8],
        end: usize,
        hash: &VectorHash<V>,
    ) -> bool {
        println!(
            "[vector_search_in] haystack({})={:?}, end={}",
            haystack.len(),
            haystack,
            end
        );
        // debug_assert!(haystack.len() >= self.needle().size());

        let mut chunks = haystack[..end].chunks_exact(V::LANES);
        for chunk in &mut chunks {
            println!("[vector_search_in] chunk({})={:?}", chunk.len(), chunk);
            if self.vector_search_in_chunk(hash, chunk.as_ptr(), -1) {
                return true;
            }
        }

        let remainder = chunks.remainder().len();
        println!("[vector_search_in] remainder: {}", remainder);
        if remainder > 0 {
            let start = haystack.as_ptr().add(end - V::LANES);
            let mask = -1 << (V::LANES - remainder);

            if self.vector_search_in_chunk(hash, start, mask) {
                return true;
            }
        }

        false
    }
}

impl Searcher for Avx2Searcher {
    #[inline(always)]
    fn position(&self) -> usize {
        self.position
    }
}

pub fn main() {
	let needle = b"consectetur";
	let haystack = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit";
	let searcher = unsafe { Avx2Searcher::with_position(needle, 0) };
	assert!(unsafe { searcher.search_in(haystack) });
}

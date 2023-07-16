rust
    #[inline]
    #[target_feature(enable = "sse4.2")]
    unsafe fn prev<const IMM8: i32>(self, prev: Self) -> Self {
        Self::from(_mm_alignr_epi8::<{ 16 - IMM8 }>(
            self.0,
            prev.0
        ))
    }

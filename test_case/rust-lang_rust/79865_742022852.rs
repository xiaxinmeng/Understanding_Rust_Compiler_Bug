rust
                let v0 = _mm256_add_epi64(
                    _mm256_and_si256(v0, _mm256_set_epi64x(-1, 0x3ffffff, 0x3ffffff, 0x3ffffff)),
                    _mm256_permute4x64_epi64(
                        _mm256_srlv_epi64(v0, _mm256_set_epi64x(64, 26, 26, 26)),
                        set02(2, 1, 0, 3),
                    ),
                );

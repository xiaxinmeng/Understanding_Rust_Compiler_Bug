
                     | size_of | min_align_of | align_of | size_of >= min_align_of
---------------------+---------+--------------+----------+-------------------------
 f32x3               |      12 |           16 |       16 | false
 f32x3_no_c          |      12 |           16 |       16 | false
 f32x3_no_simd       |      12 |            4 |        8 | true

                     | size_of | min_align_of | align_of | size_of >= min_align_of
---------------------+---------+--------------+----------+-------------------------
 u8                  |       1 |            1 |        1 | true
 OneByte             |       1 |            1 |        8 | true
 u16                 |       2 |            2 |        2 | true
 TwoByte             |       2 |            2 |        8 | true
 (u8, u16)           |       4 |            2 |        8 | true
 (u16, u8)           |       4 |            2 |        8 | true
 ThreeByte           |       4 |            2 |        8 | true
 (u32, u8)           |       8 |            4 |        8 | true
 FiveByte            |       8 |            4 |        8 | true
 (f32x3, u8)         |      32 |           16 |       16 | true
 (f32x3_no_simd, u8) |      16 |            4 |        8 | true


# original compressed size, the total size of given binary (*.so).
2117863 5081718
# uncompressed size of various encoding strategies.
# - orig: the original (unaltered)
# - relax: optimized size fields. the original metadata uses lots of
#   4-byte-long sizes even when less bytes are sufficient;
#   reclaiming them will require some works.
# - no-label: relax + no `Label` node. used for debugging purpose but
#   never disabled afaik.
# - size-elide: relax + one-byte tag. all tags are <0x100, so ignoring EBML
#   (requires 2-byte encoding for tags >=0x80) gives some gains.
#   also do not add sizes for known fixed-size tags (e.g. `U64`).
# - size-elide-2-or-4: one-byte tag + another relaxation strategy.
#   uses different size encoding algorithm: 2 bytes (big endian)
#   for sizes <0x8000, 4 bytes with MSB set otherwise.
#   trade-off between size and performance.
# - size-elide-4: one-byte tag + fixed 4-byte-long size.
orig 16084126 relax 13577526 no-label 8654851 size-elide 13019868 size-elide-2-or-4 14418657 size-elide-4 17563943
# recompressed (zlib -9) size of above.
# note that the original compressed size is *not* optimal.
orig 2087004 relax 1991335 no-label 1747192 size-elide 1966400 size-elide-2-or-4 2014455 size-elide-4 2123731

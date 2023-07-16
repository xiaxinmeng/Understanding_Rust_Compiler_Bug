rust
extend_from_slice:
for i in 0..len {
    dst[i] = src[i];
}

extend/TrustedLen:
while src != src_end {
    *dst++ = *src++;
}

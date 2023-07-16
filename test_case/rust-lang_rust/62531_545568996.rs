c++
int compare() {
    uint8_t big[4] = {0x41, 0x48, 0x00, 0x00};
    uint8_t lit[4] = {0x00, 0x00, 0x48, 0x41};
    ieee_float_shape_type gf_u;
    gf_u.value = 12.5;

    if (__BYTE_ORDER__ == __ORDER_BIG_ENDIAN__) {
        return *(uint32_t *)gf_u.bytes == *(uint32_t *)big;
    } else {
        return *(uint32_t *)gf_u.bytes == *(uint32_t *)lit;
    }
}

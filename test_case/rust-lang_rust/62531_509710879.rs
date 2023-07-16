
#if __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__
    return *(uint32_t *)gf_u.bytes == *(uint32_t *)big;
#elif __BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__
    return *(uint32_t *)gf_u.bytes == *(uint32_t *)lit;
#endif

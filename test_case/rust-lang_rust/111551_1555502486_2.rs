c
#ifdef _MSC_VER
__declspec(align(16))
struct TwoU64s
{
    uint64_t a;
    uint64_t b;
};
#endif

extern int32_t many_args(struct TwoU64s h);
